use actix_web::{get, HttpRequest, Responder};
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