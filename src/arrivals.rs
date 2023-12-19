use actix_web::{get, web, HttpResponse, Responder, post};

pub fn init_arrivals_scope() -> actix_web::Scope {
    let scope = web::scope("/arrivals")
        .service(show_arrivals)
        .service(show_specific)
        .service(insert_arrival);
    scope
}
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Arrival {
    // a_id: i32,
    time_of_day: NaiveTime,
    week_day: i32,
    tram_line: i32,
    direction: bool,
}

use sqlx::{query, query_as, PgPool};
use web::Data;
#[get("all")]
async fn show_arrivals(db_pool: Data<PgPool>) -> impl Responder {
    let arrivals = query_as!(
        Arrival,
        r#"SELECT time_of_day,week_day,tram_line,direction FROM arrivals"#
    )
    .fetch_all(db_pool.get_ref())
    .await
    .expect("Could not fetch arrivals");
    dbg!(&arrivals);
    HttpResponse::Ok()
        .content_type("application/json")
        .json(arrivals)
}
#[derive(Deserialize)]
struct ArrivalFilter {
    tram_line: i32,
    week_day: i32,
}

#[get("specific")]
async fn show_specific(db_pool: Data<PgPool>, filter: web::Query<ArrivalFilter>) -> impl Responder {
    //Le query
    let arrivals = query_as!(
        Arrival,
        "SELECT time_of_day,week_day,tram_line,direction FROM arrivals WHERE tram_line = $1 AND week_day = $2",
        filter.tram_line,
        filter.week_day
    )
    .fetch_all(db_pool.get_ref())
    .await
    .expect("Could not fetch arrivals");

    // Delet cuz we hardcoding

    // dbg!(&arrivals);
    HttpResponse::Ok()
        .content_type("application/json")
        .json(arrivals)
}
#[post("new")]
async fn insert_arrival(db_pool: Data<PgPool>, arrival: web::Query<Arrival>) -> impl Responder {
    query!(
        "INSERT INTO arrivals (time_of_day,week_day,tram_line,direction) VALUES ($1, $2, $3, $4)",
        arrival.time_of_day,
        arrival.week_day,
        arrival.tram_line,
        arrival.direction
    ).execute(db_pool.get_ref()).await.expect("I shat");
    HttpResponse::Ok().body("inserted")
}
