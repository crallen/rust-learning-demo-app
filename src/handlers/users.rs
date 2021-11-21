use actix_web::web::Data;
use actix_web::{get, HttpRequest, HttpResponse};

use crate::db::DbContext;

#[get("")]
pub async fn list_users(db: Data<DbContext>) -> HttpResponse {
    match db.users.find_all().await {
        Ok(users) => HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&users).unwrap()),

        Err(e) => {
            error!("failed to fetch user list: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/{id}")]
pub async fn show_user(req: HttpRequest, db: Data<DbContext>) -> HttpResponse {
    let id = req.match_info().get("id").unwrap();

    match db.users.find_one(id).await {
        Ok(user) => HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&user).unwrap()),

        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound(),
            _ => {
                error!("failed to retrieve user: {:?}", e);
                HttpResponse::InternalServerError()
            }
        }
        .finish(),
    }
}
