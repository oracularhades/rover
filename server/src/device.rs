use rocket::fairing::AdHoc;
use rocket::response::{Debug, status::Created};
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::response::status;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Value;
use rocket::serde::json::json;

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{MysqlPool, prelude::*};

use diesel::prelude::*;
use diesel::sql_types::*;

use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

use std::fs::{File};
use std::io::Write;

use rand::prelude::*;

use crate::global::{ send_email, generate_random_id, is_null_or_whitespace, request_authentication };
use crate::responses::*;
use crate::structs::*;
use hades_auth::*;
use crate::diesel_mysql::*;
use rocket::serde::json::serde_json;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

diesel::table! {
    rover_devices (id) {
        id -> Text,
        user_id -> Text,
        location -> Text,
        public_key -> Text,
        created -> Nullable<BigInt>,
        active -> Bool,
        os_type -> Text,
        os_version -> Text,
        alias -> Text,
        compliant -> Bool,
    }
}

#[get("/list")]
pub async fn device_list(mut db: Connection<Db>, params: &Query_string) -> Custom<Value> {
    println!("device params: {:?}", params);
    let request_authentication_output: Option<Request_authentication_output> = match request_authentication(db, None, params, "/device/list", false).await {
        Ok(data) => Some(data),
        Err(e) => None
    };
    if (request_authentication_output.is_none()) {
        return status::Custom(Status::Unauthorized, not_authorized());
    }

    let results = rover_devices::table
        .filter(rover_devices::location.eq("onboard_client"))
        .select(Rover_devices::as_select())
        .load(&mut request_authentication_output.unwrap().returned_connection)
        .await.expect("Query failed");

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": results
    }))
}

#[post("/update", format = "application/json", data = "<body>")]
pub async fn device_update(mut db: Connection<Db>, mut body: &str, params: &Query_string) -> Custom<Value> {
    let request_authentication_output: Option<Request_authentication_output> = match request_authentication(db, Some(body.to_string()), params, "/device/update", false).await {
        Ok(data) => Some(data),
        Err(e) => None
    };
    if (request_authentication_output.is_none()) {
        return status::Custom(Status::Unauthorized, not_authorized());
    }

    let results = rover_processes::table
        .select(Rover_processes::as_select())
        .load(&mut request_authentication_output.unwrap().returned_connection)
        .await.expect("Query failed");

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": results
    }))
}

#[post("/onboard", format = "application/json", data = "<body>")]
pub async fn device_onboard(mut db: Connection<Db>, mut body: &str, params: &Query_string) -> Custom<Value> {
    let request_authentication_output: Option<Request_authentication_output> = match request_authentication(db, Some(body.to_string()), params, "/device/delete", false).await {
        Ok(data) => Some(data),
        Err(e) => None
    };
    if (request_authentication_output.is_none()) {
        return status::Custom(Status::Unauthorized, not_authorized());
    }

    let results = rover_processes::table
        .select(Rover_processes::as_select())
        .load(&mut request_authentication_output.unwrap().returned_connection)
        .await.expect("Query failed");

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": results
    }))
}