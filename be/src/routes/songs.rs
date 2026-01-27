use actix_web::HttpResponse;

pub async fn songs() -> HttpResponse {
    HttpResponse::Ok().finish()
}
