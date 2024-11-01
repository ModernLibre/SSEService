use actix_web::{App, HttpServer};

mod models;
mod routes;
mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv::dotenv().ok();
    env_logger::init();
    log::info!("日志库测试！");

    //TODO: database wait to be connnected

    HttpServer::new(|| {
        App::new()
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await

}

