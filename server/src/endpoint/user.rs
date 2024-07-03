use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::serde::json::{Value, json};
use rocket::response::{status, status::Custom};
use rocket::http::Status;

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{MysqlPool, prelude::*};

use crate::global::{ request_authentication, generate_random_id };
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;

#[get("/list")]
pub async fn user_list(mut db: Connection<Db>, params: &Query_string) -> Custom<Value> {
    let request_authentication_output: Request_authentication_output = match request_authentication(db, None, params, "/user/list", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };
    db = request_authentication_output.returned_connection;

    // println!("request_authentication_output: {:?}", request_authentication_output);
    
    let results = rover_users::table
        .select(Rover_users::as_select())
        .load(&mut db)
        .await.expect("Query failed");

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": results
    }))
}

#[post("/create", format = "application/json", data = "<body>")]
pub async fn user_create(mut db: Connection<Db>, params: &Query_string, mut body: Json<User_create_body>) -> Custom<Value> {
    let request_authentication_output: Request_authentication_output = match request_authentication(db, None, params, "/user/create", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };
    db = request_authentication_output.returned_connection;

    if (body.first_name.is_none() == true) {
        return status::Custom(Status::BadRequest, error_message("body.first_name is null or whitespace."));
    }
    if (body.last_name.is_none() == true) {
        return status::Custom(Status::BadRequest, error_message("body.last_name is null or whitespace."));
    }
    if (body.email.is_none() == true) {
        return status::Custom(Status::BadRequest, error_message("body.email is null or whitespace."));
    }

    let first_name = body.first_name.clone().expect("missing body.first_name");
    let last_name = body.last_name.clone().expect("missing body.last_name");
    let email = body.email.clone().expect("missing body.email");

    let result: Option<Rover_users> = rover_users::table
        .filter(rover_users::email.eq(email.clone()))
        .first(&mut db)
        .await
        .optional().expect("Something went wrong querying the DB.");

    if (result.is_none() == false) {
        return status::Custom(Status::BadRequest, error_message(&format!("'{}' is already a user. Please use a different email address.", email)));
    }

    let user = result.unwrap();

    let user_id = generate_random_id();
    // let number: i32 = rand::thread_rng().gen_range(0..999999);

    let code_insert = Rover_users {
        id: user_id.clone(),
        email: email.clone(),
        admin_permission_flags: None
    };
    diesel::insert_into(rover_users::table)
        .values(&code_insert)
        .execute(&mut db)
        .await.expect("fail");

    return status::Custom(Status::Ok, json!({
        "ok": true,
        "user_id": user_id.clone()
    }));
}