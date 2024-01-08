#[macro_use]
extern crate log;

// #[macro_use]
// extern crate diesel;

// #[macro_use]
// extern crate diesel_migrations;

mod api;
mod models;
mod pgdb;
mod schema;

use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer};
use dotenvy::dotenv;
use std::env;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    pgdb::init();

    let addr = env::var("AC_HOST").expect("HOST not set");
    let port = env::var("AC_PORT")
        .expect("PORT not set")
        .parse::<u16>()
        .expect("PORT value invalid");
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .configure(api::routes::init_routes)
            .wrap(logger)
            .service(index)
    })
    .bind((addr, port))?
    .run()
    .await
}
