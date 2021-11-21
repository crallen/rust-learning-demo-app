use bcrypt::{hash, DEFAULT_COST};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub id: Option<Uuid>,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Default)]
pub struct UserBuilder {
    username: Option<String>,
    password: Option<String>,
}

impl UserBuilder {
    pub fn new() -> UserBuilder {
        UserBuilder {
            username: None,
            password: None,
        }
    }

    pub fn with_username(&mut self, username: String) -> &mut UserBuilder {
        self.username = Some(username);
        self
    }

    pub fn with_password(&mut self, password: String) -> &mut UserBuilder {
        match hash(password.to_owned(), DEFAULT_COST) {
            Ok(hash) => self.password = Some(hash),
            Err(e) => error!("could not hash password: {:?}", e),
        };
        self
    }

    pub fn build(&self) -> Result<User, &str> {
        let username = match self.username.as_ref() {
            Some(username) => username,
            None => return Err("username is required"),
        };
        let password = match self.password.as_ref() {
            Some(password) => password,
            None => return Err("password is required"),
        };

        Ok(User {
            id: None,
            username: username.to_owned(),
            password: password.to_owned(),
            created_at: None,
            updated_at: None,
        })
    }
}
