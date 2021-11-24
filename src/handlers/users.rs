use actix_web::{delete, get, post, web, HttpRequest, HttpResponse};

use crate::db::entities::User;
use crate::db::DbContext;
use crate::dto::{CreateUserDto, ListResultDto};

#[get("")]
pub async fn list_users(db: web::Data<DbContext>) -> HttpResponse {
    match db.users.find_all().await {
        Ok(users) => HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&ListResultDto::new(users)).unwrap()),

        Err(e) => {
            error!("failed to fetch user list: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/{id}")]
pub async fn show_user(req: HttpRequest, db: web::Data<DbContext>) -> HttpResponse {
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

#[post("")]
pub async fn create_user(dto: web::Json<CreateUserDto>, db: web::Data<DbContext>) -> HttpResponse {
    let user = match User::new(dto.0) {
        Ok(user) => user,
        Err(e) => {
            error!("failed to create user: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    match db.users.insert(user).await {
        Ok(user) => HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&user).unwrap()),

        Err(e) => {
            error!("failed to create user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/{id}")]
pub async fn delete_user(req: HttpRequest, db: web::Data<DbContext>) -> HttpResponse {
    let id = req.match_info().get("id").unwrap();

    match db.users.delete(id).await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                return HttpResponse::NotFound().finish();
            }
            HttpResponse::NoContent().finish()
        }

        Err(e) => {
            error!("failed to delete user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
