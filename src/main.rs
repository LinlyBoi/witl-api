use std::env;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use arrivals::init_arrivals_scope;
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
    let database_url = env::var("DATABASE_URL").expect("Put a DB url in the .env file dumbass");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url.as_str())
        .await
        .expect("No pool connection man :(");
    HttpServer::new(|| {
        App::new()
            .service(echo)
            .service(init_arrivals_scope())
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(init_address())?
    .run()
    .await
}
