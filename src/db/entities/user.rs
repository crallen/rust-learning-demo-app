use bcrypt::{hash, verify, BcryptResult, DEFAULT_COST};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::dto::CreateUserDto;
use crate::{IdentityError, Result};

#[derive(sqlx::FromRow, Serialize, Default)]
pub struct User {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(dto: CreateUserDto) -> Result<User> {
        match hash(dto.password.to_owned(), DEFAULT_COST) {
            Ok(password) => Ok(User {
                username: Some(dto.username.to_owned()),
                password: Some(password),
                ..Default::default()
            }),
            Err(e) => Err(IdentityError::from(e)),
        }
    }

    pub fn verify_password(&self, challenge: &'static str) -> BcryptResult<bool> {
        match self.password.as_ref() {
            Some(password) => verify(password, challenge),
            None => Ok(false),
        }
    }
}
