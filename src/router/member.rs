use actix_web::{web,  get};
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;
use diesel::dsl::sql_query;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sql_types::BigInt;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
const QUERY: &str = "SELECT now()";

pub fn member_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/members").service(get_all)
    );
}

#[derive(Debug, QueryableByName)]
pub struct TimeRow {
    #[sql_type="BigInt"]
    pub now: i64,
}

#[get("/")]
async fn get_all(pool: web::Data<DbPool>) -> String {
    let conn = pool.get().expect("Failed to get DB Conn from Pool");

    let query = sql_query(QUERY);
    let time: Vec<TimeRow> = query.load::<TimeRow>(&conn).unwrap();

    let resp = format!("{:?}", time);
    resp
}


// #[post("/new")]
// fn create_member() -> impl Responder {
//     "Create User"
// }

// #[post("/update")]
// fn update_member() -> impl Responder {
//     "Update User"
// }

// #[post("/add_tag")]
// fn add_tag() -> impl Responder {
//     "Add tag to user"
// }

// #[post("/remove_tag")]
// fn remove_tag() -> impl Responder {
//     "Remove tag from user"
// }