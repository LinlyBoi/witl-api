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
use sqlx::{PgPool, query_as, QueryBuilder, Execute};
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
async fn show_specific(db_pool: Data<PgPool>, t_line: Data<i32>, week_day: Data<i32>) -> impl Responder {

    // Extract data and match nullables
    let tram_line: i32 = match t_line.into() {
        Some(num) => **num,
        None => 1 as i32

    };
    let day =  match week_day.into() {
        Some(num) => **num,
        None => 1 as i32,
    };
    //Le query
    let arrivals = query_as!(Arrival, "SELECT * FROM arrivals WHERE tram_line = $1 AND week_day = $2", tram_line, day)
        .fetch_all(db_pool.get_ref())
        .await
        .expect("Could not fetch arrivals");


    // Delet cuz we hardcoding

	// dbg!(&arrivals);
	HttpResponse::Ok()
        .content_type("application/json")
        .json(arrivals)
}
