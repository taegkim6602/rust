use actix_web::HttpResponse;

pub async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}
