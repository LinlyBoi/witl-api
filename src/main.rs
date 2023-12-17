use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
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
    let (port, address) = init_address();
    HttpServer::new(|| {
        App::new()
            .service(echo)
            .service(arrivals::init_arrivals_scope())
            .app_data(web::Data::new(init_dbpool()))
    })
    .bind(init_address())?
    .run()
    .await
}
