mod config;
mod db;
mod handlers;
mod dto;

#[macro_use]
extern crate log;

use actix_web::middleware::{Logger, NormalizePath};
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use config::Config;
use dotenv::dotenv;
use env_logger::Env;

use crate::db::DbContext;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let cfg = Config::new();

    let db = DbContext::new(cfg)
        .await
        .expect("could not connect to database");

    let db = Data::new(db);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .configure(handlers::init)
    })
    .bind("127.0.0.1:8080")?;

    info!("Starting server");
    server.run().await
}
