use actix_web::web::Data;
use actix_web::{get, HttpRequest, HttpResponse};

use crate::db::DbContext;
use crate::handlers::models::ListResult;

#[get("")]
pub async fn list_roles(db: Data<DbContext>) -> HttpResponse {
    match db.roles.find_all().await {
        Ok(roles) => HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&ListResult::new(roles)).unwrap()),

        Err(e) => {
            error!("failed to fetch role list: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/{id}")]
pub async fn show_role(req: HttpRequest, db: Data<DbContext>) -> HttpResponse {
    let id = req.match_info().get("id").unwrap();

    match db.roles.find_one(id).await {
        Ok(role) => HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&role).unwrap()),

        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound(),
            _ => {
                error!("failed to retrieve role: {:?}", e);
                HttpResponse::InternalServerError()
            }
        }
        .finish(),
    }
}
