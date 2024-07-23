use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::serde::json::{Value, json};
use rocket::response::{status, status::Custom};
use rocket::http::Status;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{MysqlPool, prelude::*};

use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::sql_query;

use crate::global::request_authentication;
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;
use crate::SQL_TABLES;

#[get("/list")]
pub async fn network_list(mut db: Connection<Db>, params: &Query_string) -> Custom<Value> {
    let sql: Config_sql = (&*SQL_TABLES).clone();

    let request_authentication_output: Request_authentication_output = match request_authentication(db, None, params, "/network/list", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };
    db = request_authentication_output.returned_connection;

    let network_result: Vec<Rover_network> = sql_query(format!("SELECT device_id, domain, ip_address, destination_country, destination_registrant, protocol, size, info, created FROM {} ORDER BY created DESC", sql.network.unwrap()))
    .load::<Rover_network>(&mut db)
    .await
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