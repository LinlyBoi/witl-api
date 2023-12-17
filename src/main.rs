use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use witl_api::*;
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
mod arrivals;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let db_pool = init_dbpool().await;
    HttpServer::new(move || {
        App::new()
            .service(echo)
            .service(arrivals::init_arrivals_scope())
            .app_data(web::Data::new(db_pool.clone()))
    })
    .bind(init_address())?
    .run()
    .await
}
