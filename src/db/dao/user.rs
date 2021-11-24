use std::sync::Arc;

use sqlx::postgres::PgQueryResult;
use sqlx::{Pool, Postgres, Result};

use crate::db::entities::User;

pub struct UserDao {
    pool: Arc<Pool<Postgres>>,
}

impl UserDao {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        UserDao { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<User>> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE is_deleted = FALSE")
            .fetch_all(self.pool.as_ref())
            .await
    }

    pub async fn find_one(&self, id: &str) -> Result<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id::text = $1 AND is_deleted = FALSE")
            .bind(id)
            .fetch_one(self.pool.as_ref())
            .await
    }

    pub async fn insert(&self, user: User) -> Result<User> {
        let mut tx = self.pool.begin().await?;

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, password)
            VALUES ($1, $2)
            RETURNING id, username, password, created_at, updated_at
            "#,
        )
        .bind(user.username)
        .bind(user.password)
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(user)
    }

    pub async fn update(&self, user: User) -> Result<User> {
        let mut tx = self.pool.begin().await?;

        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users SET username = $1, password = $2, updated_at = now()
            WHERE id = $3
            RETURNING id, username, password, created_at, updated_at
            "#,
        )
        .bind(user.username)
        .bind(user.password)
        .bind(user.id)
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(user)
    }

    pub async fn delete(&self, id: &str) -> Result<PgQueryResult> {
        let mut tx = self.pool.begin().await?;

        let result = sqlx::query(
            r#"
            UPDATE users SET is_deleted = TRUE, updated_at = now()
            WHERE id::text = $1
            "#,
        )
        .bind(id)
        .execute(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(result)
    }
}
