use rocket::response::{Debug, status::Created};
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::response::status;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Value;
use rocket::serde::json::json;
use rocket::request::{self, Request, FromRequest};
use rocket::{fairing::{Fairing, Info, Kind}, State};
use rocket::fairing::AdHoc;
use rocket::fs::FileServer;

use diesel::prelude::*;
use diesel::sql_types::*;

use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

use std::fs::{File};
use std::io::Write;

use rand::prelude::*;

use crate::global::{ generate_random_id, is_null_or_whitespace, request_authentication };
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;

use hades_auth::*;

use core::sync::atomic::{AtomicUsize, Ordering};

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::sync::Arc;

#[options("/<_..>")]
fn options_handler() -> &'static str {
    ""
}

// Returns the current request's ID, assigning one only as necessary.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r Query_string {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // The closure passed to `local_cache` will be executed at most once per
        // request: the first time the `RequestId` guard is used. If it is
        // requested again, `local_cache` will return the same value.

        request::Outcome::Success(request.local_cache(|| {
            let query_params = request.uri().query().map(|query| query.as_str().to_owned()).unwrap_or_else(|| String::new());

            Query_string(query_params)
        }))
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket
        .mount("/", FileServer::from(format!("{}/frontend/_static", env::current_dir().expect("Could not get current process directory.").display())))
        .mount("/api", routes![options_handler])
        .mount("/api/user", routes![crate::endpoint::user::user_list, crate::endpoint::user::user_get, crate::endpoint::user::user_update])
        .mount("/api/network", routes![crate::endpoint::network::network_list])
        .mount("/api/process", routes![crate::endpoint::process::process_list])
        .mount("/api/device", routes![crate::endpoint::device::device_list, crate::endpoint::device::device_get])
    })
}