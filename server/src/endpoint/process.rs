use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::serde::json::{Value, json};
use rocket::response::{status, status::Custom};
use rocket::http::Status;

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{MysqlPool, prelude::*};

use crate::global::request_authentication;
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;

#[get("/process/list")]
pub async fn process_list(mut db: Connection<Db>, params: &Query_string) -> Custom<Value> {
    let request_authentication_output: Request_authentication_output = match request_authentication(db, None, params, "/process/list", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };
    db = request_authentication_output.returned_connection;

    let results = rover_processes::table
        // .filter(rover_network::location.eq("onboard_client"))
        .select(Rover_processes::as_select())
        .load(&mut db)
        .await.expect("Query failed");

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": results
    }))
}