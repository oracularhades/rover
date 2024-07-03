use rocket::fairing::AdHoc;
use rocket::response::{Debug, status::Created};
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::response::status;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Value;
use rocket::serde::json::json;
use rocket::serde::json::serde_json;

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

use crate::global::{ generate_random_id, is_null_or_whitespace, request_authentication };
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;

use hades_auth::*;

#[get("/list")]
pub async fn device_list(mut db: Connection<Db>, params: &Query_string) -> Custom<Value> {
    println!("device params: {:?}", params);
    let request_authentication_output: Request_authentication_output = match request_authentication(db, None, params, "/process/list", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };
    db = request_authentication_output.returned_connection;

    let results = rover_devices::table
        // .filter(rover_devices::location.eq("onboard_client"))
        .select(Rover_devices::as_select())
        .load(&mut db)
        .await.expect("Query failed");

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": results
    }))
}

#[post("/update", format = "application/json", data = "<body>")]
pub async fn device_update(mut db: Connection<Db>, mut body: &str, params: &Query_string) -> Custom<Value> {
    let request_authentication_output: Request_authentication_output = match request_authentication(db, None, params, "/process/list", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };
    db = request_authentication_output.returned_connection;

    let results = rover_processes::table
        .select(Rover_processes::as_select())
        .load(&mut db)
        .await.expect("Query failed");

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": results
    }))
}

#[post("/onboard", format = "application/json", data = "<body>")]
pub async fn device_onboard(mut db: Connection<Db>, mut body: &str, params: &Query_string) -> Custom<Value> {
    let request_authentication_output: Request_authentication_output = match request_authentication(db, None, params, "/process/list", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };
    db = request_authentication_output.returned_connection;

    let results = rover_processes::table
        .select(Rover_processes::as_select())
        .load(&mut db)
        .await.expect("Query failed");

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": results
    }))
}