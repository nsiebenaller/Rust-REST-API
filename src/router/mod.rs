use actix_web::web;

mod member;
use member::member_router;

extern crate postgres;
use postgres::{Connection, TlsMode};

pub fn router_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        .configure(member_router)
    ).data(create_connection());
}

fn create_connection() -> Connection {
    Connection::connect("postgres://postgres:postgres@localhost", TlsMode::None).unwrap()
}