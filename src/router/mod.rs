use actix_web::web;

mod member;
use member::member_router;

pub fn router_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        .configure(member_router)
    );
}