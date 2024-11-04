use actix_web::web;
use crate::controller::sse::{close_sse_connect, create_sse_connect};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
    web::scope("api/sse/v1")
                .route("/close-sse-connect", web::get().to(close_sse_connect))
                .route("/create-sse-connect", web::get().to(create_sse_connect))
    );
}