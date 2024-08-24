use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::serde::json::{Value, json};
use rocket::response::{status, status::Custom};
use rocket::http::Status;

use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::sql_query;

use crate::global::request_authentication;
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;
use crate::SQL_TABLES;

#[get("/list")]
pub async fn process_list(params: &Query_string) -> Custom<Value> {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");
    let sql: Config_sql = (&*SQL_TABLES).clone();

    let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/process/list", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };

    let process_result: Vec<Rover_processes> = sql_query(format!("SELECT device_id, process, last_seen, user, admin_user, is_admin_process, PID, publisher, hash, threads, size, pathname, created FROM {} ORDER BY created DESC", sql.process.unwrap()))
    .load::<Rover_processes>(&mut db)
    .expect("Something went wrong querying the DB.");

    let mut process_public: Vec<Rover_processes_data_for_admins> = process_result
    .into_iter()
    .map(Rover_processes_data_for_admins::from)
    .collect();

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": process_public
    }))
}