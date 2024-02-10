use actix_cors::Cors;
use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use std::io::Result;
use log::info;

fn base(req: &HttpRequest) {
    info!("Received {} request from {:?}", req.method(), req.peer_addr().unwrap())
}

#[get("/")]
async fn retrieve(req: HttpRequest) -> impl Responder {
    base(&req);

    let result = format!("Hello, {:?}", req.peer_addr().unwrap());
    result
}

#[actix_rt::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(retrieve)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}