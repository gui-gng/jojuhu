use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::dev::Service;
use actix_web::{http::header, middleware as actix_middleware, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod config;
mod docs;
mod errors;
mod middleware;
mod models;
mod modules;
mod routes;
mod utils;

use config::Settings;
use middleware::logging::RequestLogger;
use routes::configure;

    /// CORS allowed origins - in production, this should be restricted
const DEFAULT_ALLOWED_ORIGINS_LOCAL: &[&str] = &[
    "*",
];

const DEFAULT_ALLOWED_ORIGIN:String = "https://yourdomain.com".to_string();



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "jojuhu_backend=debug,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration from environment
    let settings = Settings::from_env().expect(
        "Failed to load configuration from environment. Make sure .env file exists with DATABASE_URL, HOST, PORT, JWT_SECRET"
    );
    let settings_data = web::Data::new(settings.clone());

    // Create database pool
    let database_url = settings.connection_string();
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(3))
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

    // Configure rate limiting
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(1) // 1 request per second per IP
        .burst_size(10) // Allow bursts of up to 10 requests
        .finish()
        .expect("Failed to create rate limiter config");

    // Configure CORS - use settings from config if available
    let allowed_origins: Vec<String> = if let Some(cors_settings) = &settings.cors {
        cors_settings.allowed_origins.clone()
    } else if settings.environment == "production" {
        // In production without explicit config, use environment variable or strict default
        std::env::var("ALLOWED_ORIGINS")
            .map(|origins| {
                origins.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>()
            })
            .unwrap_or_else(|_| vec![DEFAULT_ALLOWED_ORIGIN.clone()])
    } else {
        DEFAULT_ALLOWED_ORIGINS.iter().map(|&s| s.to_string()).collect()
    };

    info!("Starting server at http://{}", server_address);
    info!("Rate limiting: 1 req/sec with burst of 10");
    info!("CORS allowed origins: {:?}", allowed_origins);
    info!("API Documentation: http://{}/docs", server_address);
    info!("Routes:");
    info!("  Public: GET  /docs (Swagger UI)");
    info!("  Public: GET  /api-docs/openapi.json");
    info!("  Public: GET  /health");
    info!("  Public: POST /api/v1/auth/register");
    info!("  Public: POST /api/v1/auth/login");
    info!("  Protected: GET /api/v1/me");
    info!("  Protected: /api/v1/messages/*");
    info!("  Protected: /api/v1/timeline/*");
    info!("  Protected: /api/v1/forums/*");
    info!("  Protected: /api/v1/search");

    HttpServer::new(move || {
        let mut cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .max_age(3600)
            .supports_credentials();

        // Add allowed origins
        for origin in &allowed_origins {
            cors = cors.allowed_origin(origin);
        }

        App::new()
            .app_data(pool_data.clone())
            .app_data(settings_data.clone())
            // Rate limiting middleware
            .wrap(Governor::new(&governor_conf))
            // Request logger
            .wrap(RequestLogger)
            // Security headers middleware
            .wrap_fn(|req, srv| {
                let fut = srv.call(req);
                async {
                    let mut res = fut.await?;
                    
                    // Prevent MIME type sniffing
                    res.headers_mut().insert(
                        header::HeaderName::from_static("x-content-type-options"),
                        header::HeaderValue::from_static("nosniff"),
                    );
                    
                    // Prevent clickjacking
                    res.headers_mut().insert(
                        header::HeaderName::from_static("x-frame-options"),
                        header::HeaderValue::from_static("DENY"),
                    );
                    
                    // Enable XSS filter in browsers
                    res.headers_mut().insert(
                        header::HeaderName::from_static("x-xss-protection"),
                        header::HeaderValue::from_static("1; mode=block"),
                    );
                    
                    // HSTS - force HTTPS
                    res.headers_mut().insert(
                        header::HeaderName::from_static("strict-transport-security"),
                        header::HeaderValue::from_static("max-age=31536000; includeSubDomains"),
                    );
                    
                    // CSP - restrict resource loading
                    res.headers_mut().insert(
                        header::HeaderName::from_static("content-security-policy"),
                        header::HeaderValue::from_static("default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'"),
                    );
                    
                    // Referrer Policy
                    res.headers_mut().insert(
                        header::HeaderName::from_static("referrer-policy"),
                        header::HeaderValue::from_static("strict-origin-when-cross-origin"),
                    );
                    
                    // Permissions Policy
                    res.headers_mut().insert(
                        header::HeaderName::from_static("permissions-policy"),
                        header::HeaderValue::from_static("accelerometer=(), camera=(), geolocation=(), gyroscope=(), magnetometer=(), microphone=(), payment=(), usb=()"),
                    );
                    
                    Ok(res)
                }
            })
            // Compression
            .wrap(actix_middleware::Compress::default())
            // CORS
            .wrap(cors)
            // Request body size limit - 10MB
            .app_data(web::JsonConfig::default().limit(10_485_760))
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
            cors: Some(config::CorsSettings {
                allowed_origins: vec!["*".to_string()],
            }),
        };
        
        assert_eq!(settings.server.port, 8080);
        assert_eq!(settings.jwt.expiration_hours, 24);
    }

    #[test]
    fn test_allowed_origins_in_dev() {
        let origins: Vec<String> = DEFAULT_ALLOWED_ORIGINS
            .iter()
            .map(|&s| s.to_string())
            .collect();
        assert!(origins.contains(&"http://localhost:3000".to_string()));
        assert_eq!(origins.len(), 4);
    }
}
