use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

use crate::config::Config;
use crate::db::dao::{RoleDao, UserDao};

pub struct DbContext {
    pub roles: RoleDao,
    pub users: UserDao,
}

impl DbContext {
    pub async fn new(cfg: Config) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(cfg.pool_size)
            .connect(cfg.database_url().as_str())
            .await?;

        let pool = Arc::new(pool);

        Ok(DbContext {
            roles: RoleDao::new(pool.clone()),
            users: UserDao::new(pool.clone()),
        })
    }
}
