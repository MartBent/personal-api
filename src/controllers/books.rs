use actix_web::{get, HttpRequest, Responder};
use log::info;

use crate::database::books;

fn base(req: &HttpRequest) {
    info!("Received {} request from {:?}", req.method(), req.peer_addr().unwrap())
}

#[get("/")]
async fn retrieve(req: HttpRequest) -> impl Responder {
    base(&req);
    format!("{:?}", books::get_books().await)
}