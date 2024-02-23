use actix_web::{get, http::StatusCode, HttpRequest, HttpResponse, Responder};


#[get("/audio")]
pub async fn stream_audio(request: HttpRequest) -> impl Responder {
    let stream_response = HttpResponse::Ok().streaming(stream);


    stream_response.respond_to(&request)
}