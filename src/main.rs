use actix_web::{App, HttpServer, middleware};

mod router;
use router::router_config;

fn main() {
    let host = "127.0.0.1";
    let port = ":8080";
    let address = format!("{}{}", host, port);

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
