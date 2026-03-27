use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtSettings {
    pub secret: String,
    pub expiration_hours: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CorsSettings {
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmailSettings {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_password: String,
    pub from_email: String,
    pub from_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub server: ServerSettings,
    pub jwt: JwtSettings,
    pub email: EmailSettings,
    pub cors: Option<CorsSettings>,
    #[allow(dead_code)]
    pub environment: String,
}

impl Settings {
    #[allow(dead_code)]
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".into());

        // Build config from multiple sources
        let mut builder = Config::builder()
            // Optional config files
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false));

        // Add environment variables with proper prefix/separator
        // This expects: APP_DATABASE__URL, APP_SERVER__HOST, etc.
        builder = builder.add_source(Environment::with_prefix("APP").separator("__"));

        let config = builder.build()?;

        // Try to deserialize
        let mut settings: Settings = config.try_deserialize()?;

        // Set environment field
        settings.environment = run_mode;

        Ok(settings)
    }

    /// Alternative constructor that loads directly from environment variables
    /// without requiring APP_ prefix (uses DATABASE_URL, HOST, PORT, etc.)
    pub fn from_env() -> Result<Self, ConfigError> {
        let run_mode = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".into());

        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| ConfigError::NotFound("DATABASE_URL".into()))?;
        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse::<u16>()
            .map_err(|e| ConfigError::Message(format!("Invalid PORT: {}", e)))?;
        let jwt_secret =
            std::env::var("JWT_SECRET").map_err(|_| ConfigError::NotFound("JWT_SECRET".into()))?;
        let jwt_expiration_hours = std::env::var("JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| "24".into())
            .parse::<i64>()
            .map_err(|e| ConfigError::Message(format!("Invalid JWT_EXPIRATION_HOURS: {}", e)))?;

        // Parse CORS allowed origins from environment
        let allowed_origins = std::env::var("ALLOWED_ORIGINS")
            .map(|origins| {
                origins
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(|_| {
                // Default origins for development
                vec![
                    "http://localhost:3000".to_string(),
                    "http://localhost:8080".to_string(),
                    "http://localhost:5000".to_string(),
                    "http://localhost:4200".to_string(),
                    "http://127.0.0.1:3000".to_string(),
                    "http://127.0.0.1:8080".to_string(),
                    "http://127.0.0.1:5000".to_string(),
                    "http://127.0.0.1:4200".to_string(),
                ]
            });

        let cors = Some(CorsSettings { allowed_origins });

        // Email settings with defaults for development
        let email = EmailSettings {
            smtp_host: std::env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".into()),
            smtp_port: std::env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".into())
                .parse::<u16>()
                .map_err(|e| ConfigError::Message(format!("Invalid SMTP_PORT: {}", e)))?,
            smtp_user: std::env::var("SMTP_USER").unwrap_or_else(|_| "".into()),
            smtp_password: std::env::var("SMTP_PASSWORD").unwrap_or_else(|_| "".into()),
            from_email: std::env::var("FROM_EMAIL").unwrap_or_else(|_| "noreply@jojuhu.com".into()),
            from_name: std::env::var("FROM_NAME").unwrap_or_else(|_| "Jojuhu".into()),
        };

        Ok(Settings {
            database: DatabaseSettings { url: database_url },
            server: ServerSettings { host, port },
            jwt: JwtSettings {
                secret: jwt_secret,
                expiration_hours: jwt_expiration_hours,
            },
            email,
            cors,
            environment: run_mode,
        })
    }

    pub fn connection_string(&self) -> &str {
        &self.database.url
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}
