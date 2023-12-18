use actix_web::{get, web, HttpResponse, Responder};

pub fn init_arrivals_scope() -> actix_web::Scope {
    let scope = web::scope("/arrivals").service(show_arrivals);
    scope
}
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Arrival {
    a_id: i32,
    time_of_day: NaiveTime,
    week_day: i32,
    tram_line: i32,
    direction: bool,
}
use web::Data;
use sqlx::{PgPool, query_as, query};
#[get("all")]
async fn show_arrivals(db_pool: Data<PgPool>) -> impl Responder {
    let arrivals = query_as!(Arrival, r#"SELECT * FROM arrivals"#)
        .fetch_all(db_pool.get_ref())
        .await
        .expect("Could not fetch arrivals");
	dbg!(&arrivals);
	HttpResponse::Ok()
        .content_type("application/json")
        .json(arrivals)
}

#[get("specific")]
async fn show_specific(db_pool: Data<PgPool>, type_arr: Vec<u8>) -> impl Responder {
    // Query Logic
    // Idea: Construct a query bit by bit depending on input

    let mut dyn_query = String::from("SELECT  * FROM arrivals WHERE ");

    if type_arr.len() == 1 {
        dyn_query.push_str(&format!("tram_line = {}", type_arr[0]));
    }


    // Check: https://stackoverflow.com/questions/74956100/how-to-build-safe-dynamic-query-with-sqlx-in-rust
    // lappy ded ;-;
    let arrivals = String::from("AIDS");

	dbg!(&arrivals);
	HttpResponse::Ok()
        .content_type("application/json")
        .json(arrivals)
}