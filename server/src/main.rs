#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;

#[cfg(test)] mod tests;

mod diesel_mysql;
mod global;
mod structs;

use rocket::response::Redirect;

#[get("/")]
fn index() -> &'static str {
    // Redirect::to(uri!("/sqlx", sqlx::list()))
    "Smashing babe"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        // .mount("/", routes![index])
        .attach(diesel_mysql::stage())
}
