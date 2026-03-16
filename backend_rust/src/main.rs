use actix_cors::Cors;
use actix_web::{middleware as actix_middleware, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod config;
mod errors;
mod middleware;
mod models;
mod modules;
mod routes;
mod utils;

use config::Settings;
use middleware::logging::RequestLogger;
use routes::configure;

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

    // Load configuration from environment
    let settings = Settings::from_env().expect("Failed to load configuration from environment. Make sure .env file exists with DATABASE_URL, HOST, PORT, JWT_SECRET");
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
    info!("Routes:");
    info!("  Public: POST /api/v1/auth/register");
    info!("  Public: POST /api/v1/auth/login");
    info!("  Protected: GET /api/v1/me");
    info!("  Protected: /api/v1/messages/*");
    info!("  Protected: /api/v1/timeline/*");
    info!("  Protected: /api/v1/forums/*");

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
            .configure(|cfg| configure(cfg, pool.clone(), settings.clone()))
    })
    .bind(server_address)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_creation() {
        // This test verifies that the Settings struct can be created
        // In a real test, you'd use mock environment variables
        let settings = Settings {
            database: config::DatabaseSettings {
                url: "postgres://localhost/test".to_string(),
            },
            server: config::ServerSettings {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            jwt: config::JwtSettings {
                secret: "test_secret".to_string(),
                expiration_hours: 24,
            },
            environment: "test".to_string(),
        };
        
        assert_eq!(settings.server.port, 8080);
        assert_eq!(settings.jwt.expiration_hours, 24);
    }
}
