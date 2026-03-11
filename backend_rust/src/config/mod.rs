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
pub struct Settings {
    pub database: DatabaseSettings,
    pub server: ServerSettings,
    pub jwt: JwtSettings,
    pub environment: String,
}

impl Settings {
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
        let host = std::env::var("HOST")
            .unwrap_or_else(|_| "127.0.0.1".into());
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse::<u16>()
            .map_err(|e| ConfigError::Message(format!("Invalid PORT: {}", e)))?;
        let jwt_secret = std::env::var("JWT_SECRET")
            .map_err(|_| ConfigError::NotFound("JWT_SECRET".into()))?;
        let jwt_expiration_hours = std::env::var("JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| "24".into())
            .parse::<i64>()
            .map_err(|e| ConfigError::Message(format!("Invalid JWT_EXPIRATION_HOURS: {}", e)))?;

        Ok(Settings {
            database: DatabaseSettings { url: database_url },
            server: ServerSettings { host, port },
            jwt: JwtSettings {
                secret: jwt_secret,
                expiration_hours: jwt_expiration_hours,
            },
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
