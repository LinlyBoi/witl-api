use actix_web::{get, web, HttpResponse, Responder};

pub fn init_arrivals_scope() -> actix_web::Scope {
    let scope = web::scope("/arrivals").service(show_arrivals);
    scope
}
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Arrival {
    a_id: u32,
    time_of_day: NaiveDateTime,
    week_day: u32,
    tram_line: u32,
    direction: bool,
}
use web::Data;
use sqlx::{PgPool, query_as};
#[get("arrivals")]
async fn show_arrivals(db_pool: Data<PgPool>) -> impl Responder {
    let arrivals = query_as!(Arrival, r#"SELECT * FROM arrivals"#)
        .fetch_all(db_pool)
        .await
        .expect("Could not fetch arrivals");
    HttpResponse::Ok()
        .content_type("application/json")
        .json(arrivals);
    todo!()
}
