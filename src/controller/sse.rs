use actix_web::HttpResponse;

pub async fn close_sse_connect() -> HttpResponse {
    HttpResponse::Ok().json("close sse connect")
}

pub async fn create_sse_connect() -> HttpResponse {
    HttpResponse::Ok().json("create sse connect")
}
