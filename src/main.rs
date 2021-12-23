#[macro_use] extern crate rocket;

use rocket_sync_db_pools::{diesel, database};

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[catch(503)]
fn service_not_available() -> &'static str {
    "Service not available..."
}

#[database("wahapi_40k_db")]
struct Wahapi40kDbConn(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Wahapi40kDbConn::fairing())
        .register("/", catchers![service_not_available])
        .mount("/", routes![index])
}
