use std::env;

pub struct Config {
    pub db_host: String,
    pub db_port: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub pool_size: u32,
}

impl Config {
    pub fn new() -> Self {
        Config {
            db_host: env::var("DB_HOST").unwrap_or("localhost".to_string()),
            db_port: env::var("DB_PORT").unwrap_or("5432".to_string()),
            db_user: env::var("DB_USER").unwrap_or("identity".to_string()),
            db_password: env::var("DB_PASSWORD").unwrap_or("".to_string()),
            db_name: env::var("DB_NAME").unwrap_or("identity".to_string()),

            pool_size: env::var("POOL_SIZE")
                .unwrap_or("5".to_string())
                .parse::<u32>()
                .unwrap(),
        }
    }

    pub fn database_url(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}", self.db_user, self.db_password, self.db_host, self.db_port, self.db_name)
    }
}
