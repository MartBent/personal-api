use actix_cors::Cors;
use actix_web::{App, HttpServer};
use std::io::Result;

pub mod controllers;
pub mod database;
pub mod models;

#[actix_rt::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(controllers::audio::stream_audio)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}