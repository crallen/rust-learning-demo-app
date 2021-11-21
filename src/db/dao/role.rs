use sqlx::{Pool, Postgres, Result};
use std::sync::Arc;

use crate::db::entities::Role;

pub struct RoleDao {
    pool: Arc<Pool<Postgres>>,
}

impl RoleDao {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        RoleDao { pool: pool.clone() }
    }

    pub async fn find_all(&self) -> Result<Vec<Role>> {
        sqlx::query_as::<_, Role>("SELECT * FROM roles")
            .fetch_all(self.pool.as_ref())
            .await
    }

    pub async fn find_one(&self, id: &str) -> Result<Role> {
        sqlx::query_as::<_, Role>("SELECT * FROM roles WHERE id::text = $1")
            .bind(id)
            .fetch_one(self.pool.as_ref())
            .await
    }
}
