use actix_web::{web, get};
mod member;
use member::member_router;

pub fn router_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        .configure(member_router),
    );
    cfg.service(web::scope("").service(no_params));
}

#[get("/")]
async fn no_params() -> &'static str {
    "Hello world!\r\n"
}