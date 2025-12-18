use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub environment: String,
    pub port: u16,
    pub database_url: String,
    #[allow(dead_code)]
    pub redis_url: String,
    #[allow(dead_code)]
    pub jwt_secret: String,
    #[allow(dead_code)]
    pub jwt_expiration_hours: i64,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenvy::dotenv().ok();

        Ok(Config {
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a number"),
            database_url: env::var("DATABASE_URL")?,
            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "default-secret-change-me".to_string()),
            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .expect("JWT_EXPIRATION_HOURS must be a number"),
        })
    }
}