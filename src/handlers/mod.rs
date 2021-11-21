mod users;

use actix_web::{get, web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(ready);

    cfg.service(
        web::scope("/users")
            .service(users::list_users)
            .service(users::show_user),
    );
}

#[get("/ready")]
async fn ready() -> impl Responder {
    HttpResponse::Ok().body("ready")
}
