use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;

use crate::config::Config;
use crate::db::dao::UserDao;

pub struct DbContext {
    pub users: UserDao,
}

impl DbContext {
    pub async fn new(cfg: Config) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(cfg.pool_size)
            .connect(cfg.database_url.as_str())
            .await?;

        let pool = Arc::new(pool);

        Ok(DbContext {
            users: UserDao::new(pool)
        })
    }
}