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

    let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/device/list", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };

    let devices_result: Vec<Rover_devices> = sql_query(format!("SELECT id, user_id, created, active, compliant, os_type, os_version, alias, public_key FROM {} WHERE location=? ORDER BY created DESC", sql.device.unwrap()))
    .bind::<Text, _>("onboard_client".to_string())
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

// #[post("/update", format = "application/json", data = "<body>")]
// pub async fn device_update(mut db: Connection<Db>, mut body: &str, params: &Query_string) -> Custom<Value> {
//     let sql: Config_sql = (&*SQL_TABLES).clone();

//     let request_authentication_output: Request_authentication_output = match request_authentication(db, None, params, "/process/list", false).await {
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

//     let request_authentication_output: Request_authentication_output = match request_authentication(db, None, params, "/process/list", false).await {
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