#[macro_use]
extern crate log;

use std::env;

use actix_web::{App, get, HttpResponse, HttpServer, middleware::Logger};
use dotenvy::dotenv;

mod api;
mod models;
mod pgdb;
mod schema;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("-- > APP STARTED");
    dotenv().ok();
    env_logger::init();

    println!("-- > Init db");
    pgdb::init();

    println!("-- > Prepare host");
    let addr = env::var("AC_HOST").expect("HOST not set");
    let port = env::var("AC_PORT")
        .expect("PORT not set")
        .parse::<u16>()
        .expect("PORT value invalid");
    println!("-- > Run server");
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
