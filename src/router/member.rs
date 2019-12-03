use actix_web::{web, Responder, get, post};

pub fn member_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/members")
        .service(get_all)
    );
}

#[get("")]
fn get_all() -> impl Responder {
    "Hello world!"
}

#[post("/new")]
fn create_member() -> impl Responder {
    "Create User"
}

#[post("/update")]
fn update_member() -> impl Responder {
    "Update User"
}

#[post("/add_tag")]
fn add_tag() -> impl Responder {
    "Add tag to user"
}

#[post("/remove_tag")]
fn remove_tag() -> impl Responder {
    "Remove tag from user"
}