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
async fn show_specific(db_pool: Data<PgPool>, t_line: Vec<u8>, week_day: Some(u8)) -> impl Responder {
    // Query Logic
    // Idea: Construct a query bit by bit depending on input

    let mut dyn_query = QueryBuilder::new("SELECT * FROM arrivals WHERE tram_line = ");

    if t_line.len() == 1 {
        dyn_query.push_bind(t_line[0]);
    } else {
        dyn_query.push_bind(t_line[0]);
        dyn_query.push("OR tram_line = ");
        dyn_query.push_bind(t_line[1]);
    } // Should be fine for tramline?

    if let Some(week_day) {
        dyn_query.push("AND week_day = ");
        dyn_query.push_bind(week_day);
    }

    dyn_query.build().sql().into();

    // We don't talk about this
    // and idk if `query_as!()` will take a QueryBuilder
    let arrivals = String::from("AIDS");

    // Idk what to do, can't test with db cuz no .env xdxd
    // linly pls i never made an api before and royal son doesn't count

	dbg!(&arrivals);
	HttpResponse::Ok()
        .content_type("application/json")
        .json(arrivals)
}