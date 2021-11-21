use std::env;

pub struct Config {
    pub database_url: String,
    pub pool_size: u32,
}

impl Config {
    pub fn new() -> Self {
        Config {
            database_url: env::var("DATABASE_URL").unwrap(),
            pool_size: env::var("POOL_SIZE")
                .unwrap_or("5".to_string())
                .parse::<u32>()
                .unwrap(),
        }
    }
}
