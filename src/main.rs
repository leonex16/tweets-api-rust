#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;

use std::{env, io};
use actix_web::{middleware, App, HttpServer};
use diesel::{PgConnection, r2d2::ConnectionManager};
use r2d2::{Pool, PooledConnection};

mod tweet;
mod like;
mod constants;
mod responses;
mod schema;

pub type ConnMang = ConnectionManager::<PgConnection>;
pub type DBPool = Pool<ConnMang>;
pub type DBPooledConnection = PooledConnection<ConnMang>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // SET UP DATABASE CONNECTION POOL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL Must to be set");
    let manager = ConnMang::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
        .data(pool.clone())
        .wrap(middleware::Logger::default())
        .service(tweet::list)
        .service(tweet::get)
        .service(tweet::create)
        .service(tweet::delete)
        .service(like::list)
        .service(like::plus_one)
        .service(like::minus_one)
    })
    .bind("0.0.0.0::80")?
    .run()
    .await
}
