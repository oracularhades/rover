use rocket::response::{Debug, status::Created};
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::response::status;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Value;
use rocket::serde::json::json;

use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::sql_query;

use crate::global::{ generate_random_id, is_null_or_whitespace, request_authentication };
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;
use crate::SQL_TABLES;

#[get("/list")]
pub async fn device_list(params: &Query_string) -> Custom<Value> {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");
    let sql: Config_sql = (&*SQL_TABLES).clone();

    let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/device/list").await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };

    let devices_result: Vec<Rover_devices> = rover_devices::table
    .filter(rover_devices::location.eq("onboard_client"))
    .order(rover_devices::created.desc())
    .load::<Rover_devices>(&mut *db)
    .expect("Something went wrong querying the DB.");

    let mut devices_public: Vec<Rover_devices_data_for_admins> = devices_result
    .into_iter()
    .map(Rover_devices_data_for_admins::from)
    .collect();

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": devices_public
    }))
}

#[get("/get?<id>")]
pub async fn device_get(params: &Query_string, id: Option<String>) -> Custom<Value> {
    // Check params.id is not null or whitespace.
    if (id.is_none() == true) {
        return status::Custom(Status::BadRequest, error_message("params.id is null or whitespace."));
    }

    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");
    let sql: Config_sql = (&*SQL_TABLES).clone();

    // Authenticate the user - this makes this endpoint authenticated.
    let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/device/get").await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };

    let devices_result: Vec<Rover_devices> = rover_devices::table
    .filter(rover_devices::id.eq(id.unwrap())) // SQL injection safe version of: WHERE rover_devices.id=VALUE
    .filter(rover_devices::location.eq("onboard_client")) // WHERE rover_devices.id='onboard_client'
    .order(rover_devices::created.desc()) // ORDER BY rover_devices.created DESC
    .load::<Rover_devices>(&mut *db)
    .expect("Something went wrong querying the DB.");

    // Loop through returned results and format the data to the API spec. For example, remove private system data from the results.
    let mut devices_public: Vec<Rover_devices_data_for_admins> = devices_result
    .into_iter()
    .map(Rover_devices_data_for_admins::from)
    .collect();

    // Return results to client.
    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": devices_public
    }))
}

// #[post("/update", format = "application/json", data = "<body>")]
// pub async fn device_update(mut db: Connection<Db>, mut body: &str, params: &Query_string) -> Custom<Value> {
//     let sql: Config_sql = (&*SQL_TABLES).clone();

//     let request_authentication_output: Request_authentication_output = match request_authentication(db, None, Some(params), "/process/list").await {
//         Ok(data) => data,
//         Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
//     };
//     db = request_authentication_output.returned_connection;

//     let results = rover_processes::table
//         .select(Rover_processes::as_select())
//         .load(&mut db)
//         .await.expect("Query failed");

//     status::Custom(Status::Ok, json!({
//         "ok": true,
//         "data": results
//     }))
// }

// #[post("/onboard", format = "application/json", data = "<body>")]
// pub async fn device_onboard(mut db: Connection<Db>, mut body: &str, params: &Query_string) -> Custom<Value> {
//     let sql: Config_sql = (&*SQL_TABLES).clone();

//     let request_authentication_output: Request_authentication_output = match request_authentication(db, None, Some(params), "/process/list").await {
//         Ok(data) => data,
//         Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
//     };
//     db = request_authentication_output.returned_connection;

//     let results = rover_processes::table
//         .select(Rover_processes::as_select())
//         .load(&mut db)
//         .await.expect("Query failed");

//     status::Custom(Status::Ok, json!({
//         "ok": true,
//         "data": results
//     }))
// }