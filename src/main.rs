#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{App, HttpServer, middleware};

mod router;
use router::router_config;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};


use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Configure
    dotenv().ok();
    let host = "127.0.0.1";
    let port = "8080";
    let bind = format!("{}:{}", host, port);
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    let connspec = get_connspec();

    // set up database connection pool
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(router_config)
    })
    .bind(&bind)?
    .run()
    .await
}

// Verifies that the given db url works
pub fn get_connspec() -> String {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
    database_url
}
