use actix_cors::Cors;
use actix_web::{middleware as actix_middleware, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod errors;
mod middleware;
mod models;
mod modules;
mod utils;

use config::Settings;
use middleware::logging::RequestLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "social_network=debug,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let settings = Settings::new().expect("Failed to load configuration");
    let settings_data = web::Data::new(settings.clone());

    // Create database pool
    let database_url = settings.connection_string();
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .min_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(database_url)
        .await
        .expect("Failed to connect to database");

    info!("Connected to database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    info!("Database migrations completed");

    let pool_data = web::Data::new(pool.clone());
    let server_address = settings.server_address();

    info!("Starting server at http://{}", server_address);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(pool_data.clone())
            .app_data(settings_data.clone())
            .wrap(RequestLogger)
            .wrap(actix_middleware::Compress::default())
            .wrap(cors)
            .configure(|cfg| routes(cfg, pool.clone(), settings.clone()))
    })
    .bind(server_address)?
    .run()
    .await
}

fn routes(cfg: &mut web::ServiceConfig, pool: sqlx::PgPool, _settings: Settings) {
    cfg.service(
        web::scope("/api/v1")
            .configure(|c| modules::messages::configure_module(c, pool.clone()))
            .configure(|c| modules::timeline::configure_module(c, pool.clone()))
            .configure(|c| modules::forums::configure_module(c, pool.clone()))
            .configure(auth_routes),
    );
}

fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/me", web::get().to(get_current_user)),
    );
}

use crate::errors::AppError;
use crate::models::user::{CreateUserRequest, User, UserResponse};
use crate::models::ApiResponse;
use crate::utils::auth::generate_token;
use crate::utils::{hash_password, verify_password};
use actix_web::HttpResponse;

async fn register(
    pool: web::Data<sqlx::PgPool>,
    settings: web::Data<Settings>,
    request: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let req = request.into_inner();

    // Validate input
    if req.username.len() < 3 || req.username.len() > 32 {
        return Err(AppError::ValidationError(
            "Username must be between 3 and 32 characters".to_string(),
        ));
    }

    if req.password.len() < 8 {
        return Err(AppError::ValidationError(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    // Hash password
    let password_hash = hash_password(&req.password)
        .map_err(|e| AppError::InternalError(format!("Password hashing failed: {}", e)))?;

    // Create user
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, email, password_hash, display_name)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(&req.username)
    .bind(&req.email)
    .bind(&password_hash)
    .bind(req.display_name.as_ref().unwrap_or(&req.username))
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) if db_err.constraint().is_some() => {
            AppError::ConflictError("Username or email already exists".to_string())
        }
        _ => e.into(),
    })?;

    // Generate token
    let token = generate_token(user.id, user.username.clone(), user.email.clone(), &settings)?;

    let response = serde_json::json!({
        "user": UserResponse::from(user),
        "token": token
    });

    Ok(HttpResponse::Created().json(ApiResponse::success(response)))
}

#[derive(Debug, serde::Deserialize)]
struct LoginRequest {
    username_or_email: String,
    password: String,
}

async fn login(
    pool: web::Data<sqlx::PgPool>,
    settings: web::Data<Settings>,
    request: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let req = request.into_inner();

    // Find user by username or email
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1 OR email = $1",
    )
    .bind(&req.username_or_email)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::AuthenticationError("Invalid credentials".to_string()))?;

    // Verify password
    let valid = verify_password(&req.password, &user.password_hash)
        .map_err(|_| AppError::AuthenticationError("Invalid credentials".to_string()))?;

    if !valid {
        return Err(AppError::AuthenticationError("Invalid credentials".to_string()));
    }

    // Generate token
    let token = generate_token(user.id, user.username.clone(), user.email.clone(), &settings)?;

    let response = serde_json::json!({
        "user": UserResponse::from(user),
        "token": token
    });

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

async fn get_current_user(
    pool: web::Data<sqlx::PgPool>,
    user: crate::middleware::auth::AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user.user_id)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(UserResponse::from(user))))
}
