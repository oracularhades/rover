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
pub async fn network_list(params: &Query_string) -> Custom<Value> {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");
    let sql: Config_sql = (&*SQL_TABLES).clone();

    let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/network/list", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };

    let network_result: Vec<Rover_network> = sql_query(format!("SELECT device_id, domain, ip_address, destination_country, destination_registrant, protocol, size, info, created FROM {} ORDER BY created DESC", sql.network.unwrap()))
    .load::<Rover_network>(&mut db)
    .expect("Something went wrong querying the DB.");

    let mut network_public: Vec<Rover_network_data_for_admins> = network_result
    .into_iter()
    .map(Rover_network_data_for_admins::from)
    .collect();

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": network_public
    }))
}