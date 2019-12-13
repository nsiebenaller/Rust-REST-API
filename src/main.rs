use actix_web::{App, HttpServer, middleware};

mod router;
use router::router_config;

extern crate postgres;
use postgres::{Connection, TlsMode};

extern crate chrono;
use chrono::{DateTime, Utc};


fn main() {
    let host = "127.0.0.1";
    let port = ":8080";
    let address = format!("{}{}", host, port);

    let conn = Connection::connect("postgres://postgres:postgres@localhost", TlsMode::None).unwrap();

    let result = conn.query("SELECT now()", &[]).unwrap();
    let time: DateTime<Utc> = result.get(0).get(0);
    println!("{:?}", time);

    return;


    println!("Starting server on port {}{}", host, port);
    
    HttpServer::new(|| {
        App::new()
        .configure(router_config)
        .wrap(middleware::Logger::default())
    })
    .bind(address)
    .unwrap()
    .run()
    .unwrap();
}
