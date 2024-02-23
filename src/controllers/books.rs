use actix_web::{get, post, put, delete, HttpRequest, Responder};
use log::info;

use crate::database::books;

fn base(req: &HttpRequest) {
    info!("Received {} request from {:?}", req.method(), req.peer_addr().unwrap())
}

#[post("/books")]
async fn create(req: HttpRequest) -> impl Responder {
    base(&req);
    format!("{:?}", books::get_books().await)
}

#[get("/books/{id}")]
async fn retrieve(req: HttpRequest) -> impl Responder {
    base(&req);
    format!("{:?}", books::get_books().await)
}

#[get("/books")]
async fn retrieveAll(req: HttpRequest) -> impl Responder {
    base(&req);
    format!("{:?}", books::get_books().await)
}

#[put("/books/{id}")]
async fn update(req: HttpRequest) -> impl Responder {
    base(&req);
    format!("{:?}", books::get_books().await)
}

#[delete("/books/{id}")]
async fn delete(req: HttpRequest) -> impl Responder {
    base(&req);
    format!("{:?}", books::get_books().await)
}