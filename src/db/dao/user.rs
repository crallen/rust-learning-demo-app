use sqlx::{Pool, Postgres, Result};
use std::sync::Arc;

use crate::db::entities::User;

pub struct UserDao {
    pool: Arc<Pool<Postgres>>,
}

impl UserDao {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        UserDao { pool: pool.clone() }
    }

    pub async fn find_all(&self) -> Result<Vec<User>> {
        sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(self.pool.as_ref())
            .await
    }

    pub async fn find_one(&self, id: &str) -> Result<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id::text = $1")
            .bind(id)
            .fetch_one(self.pool.as_ref())
            .await
    }

    pub async fn insert(&self, user: User) -> Result<User> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, password)
            VALUES ($1, $2)
            RETURNING id, username, password, created_at, updated_at
            "#,
        )
        .bind(user.username)
        .bind(user.password)
        .fetch_one(self.pool.as_ref())
        .await
    }
}
