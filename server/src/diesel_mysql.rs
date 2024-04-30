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

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{MysqlPool, prelude::*};

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

use crate::global::{ send_email, generate_random_id, is_null_or_whitespace, request_authentication };
use crate::responses::*;
use crate::structs::*;
use hades_auth::*;

use core::sync::atomic::{AtomicUsize, Ordering};

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

diesel::table! {
    posts (id) {
        id -> Nullable<BigInt>,
        title -> Text,
        text -> Text,
        published -> Bool,
    }
}
diesel::table! {
    rover_users (id) {
        id -> Text,
        email -> Text,
        username -> Text,
        admin_permission_flags -> Nullable<BigInt>,
    }
}
diesel::table! {
    rover_network (device_id) {
        device_id -> Text,
        domain -> Text,
        ip_address -> Text,
        destination_country -> Text,
        destination_registrant -> Text,
        protocol -> Text,
        size -> Nullable<BigInt>,
        info -> Text,
    }
}
diesel::table! {
    rover_processes (device_id) {
        device_id -> Text,
        process -> Text,
        last_seen ->  Nullable<BigInt>,
        user -> Text,
        admin_user -> Text,
        is_admin_process -> Text,
        PID -> Nullable<BigInt>,
        publisher -> Text,
        hash -> Text,
        threads ->  Nullable<BigInt>,
        size ->  Nullable<BigInt>,
        pathname -> Text,
    }
}
diesel::table! {
    login_codes (code) {
        attempt_id -> Text,
        code -> Nullable<BigInt>,
        created -> Nullable<BigInt>,
        attempts -> Nullable<BigInt>,
        user_id -> Text,
    }
}

#[post("/login", format = "application/json", data = "<body>")]
async fn login(mut db: Connection<Db>, mut body: Json<Login_body>) -> Custom<Value> {
    // diesel::sql_function!(fn last_insert_id() -> BigInt);

    if (is_null_or_whitespace(body.email.clone())) {
        return status::Custom(Status::BadRequest, error_message("body.email is null or whitespace."));
    }

    let result: Option<Rover_users> = rover_users::table
        .filter(rover_users::email.eq(body.email.clone()))
        .first(&mut db)
        .await
        .optional().expect("Something went wrong querying the DB.");

    if (result.is_none()) {
        return status::Custom(Status::BadRequest, error_message("User not found"));
    }

    let user = result.unwrap();

    let attempt_id = generate_random_id();
    let number: i32 = rand::thread_rng().gen_range(0..999999);

    send_email(body.email.clone(), "Your login code".to_string(), format!("Do not share this with anyone. This code serves no purpose except logging you into your account. If you didn't request this code, you can ignore it.\n\nLogin code: {}", number)).await;

    let code_insert = Login_code_record {
        attempt_id: attempt_id.clone(),
        code: Some(number.into()),
        created: Some(0),
        attempts: Some(0),
        user_id: user.id
    };
    diesel::insert_into(login_codes::table)
        .values(&code_insert)
        .execute(&mut db)
        .await.expect("fail");

    return status::Custom(Status::Ok, json!({
        "ok": true,
        "attempt_id": attempt_id.clone()
    }));
    
    // Ok(Created::new("/").body(post))
}

#[post("/authenticate", format = "application/json", data = "<body>")]
async fn authenticate(mut db: Connection<Db>, mut body: Json<Authenticate_Body>) -> Custom<Value> {
    if (is_null_or_whitespace(body.attempt_id.clone())) {
        return status::Custom(Status::Ok, error_message("body.attempt_id is null or whitespace."));
    }
    if (body.code.is_none()) {
        return status::Custom(Status::Ok, error_message("body.code is null or whitespace."));
    }
    if (is_null_or_whitespace(body.public_key.clone())) {
        return status::Custom(Status::Ok, error_message("body.public_key is null or whitespace."));
    }

    // TODO: missing expiry

    let result: Option<Login_code_record> = login_codes::table
        .filter(login_codes::attempt_id.eq(body.attempt_id.clone()))
        .first(&mut db)
        .await
        .optional().expect("Something went wrong querying the DB.");

    if (result.is_none()) {
        return status::Custom(Status::BadRequest, error_message("Invalid attempt id"));
    }

    let login_attempt_data = result.unwrap();

    if (login_attempt_data.code.is_none() || login_attempt_data.code != body.code) {
        // Invalid code, TODO: add an attempt.
        return status::Custom(Status::Unauthorized, error_message("Invalid code"));
    }

    onboard_new_device(&body.public_key).await.expect("Fail to onboard device.");

    let device_id = generate_random_id();

    let device_insert = Rover_devices {
        id: device_id.clone(),
        user_id: login_attempt_data.user_id.clone(),
        location: "admin_panel".to_string(),
        public_key: body.public_key.clone(),
        created: Some(TryInto::<i64>::try_into(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get duration since unix epoch")
        .as_millis()).expect("Failed to get timestamp")),
        active: true,
        alias: "".to_string(),
        compliant: false,
        os_type: "".to_string(),
        os_version: "".to_string()
    };

    diesel::insert_into(crate::device::rover_devices::table)
        .values(&device_insert)
        .execute(&mut db)
        .await.expect("fail");

    let affected = diesel::delete(login_codes::table)
        .filter(login_codes::attempt_id.eq(body.attempt_id.clone()))
        .execute(&mut db)
        .await.expect("fail");

    status::Custom(Status::Ok, json!({
        "ok": true,
        "device_id": device_id
    }))
}

// #[get("/")]
// async fn list(mut db: Connection<Db>) -> Result<Json<Vec<Post>>> {
//     let results = posts::table
//         // .filter(posts::published.eq(true))
//         .limit(5)
//         .select(Post::as_select())
//         .load(&mut db)
//         .await?;
//         // .expect("Error loading posts");
    
//     // send_email("hi@oracularhades.com").await;

//     Ok(Json(results))
// }

#[get("/users/list")]
async fn users_list(db: Connection<Db>, params: &Query_string) -> Custom<Value> {
    let request_authentication_output: Option<Request_authentication_output> = match request_authentication(db, None, params, "/users/list", false).await {
        Ok(data) => Some(data),
        Err(e) => None
    };
    if (request_authentication_output.is_none()) {
        return status::Custom(Status::Unauthorized, not_authorized());
    }

    // println!("request_authentication_output: {:?}", request_authentication_output);
    
    let results = rover_users::table
        .select(Rover_users::as_select())
        .load(&mut request_authentication_output.unwrap().returned_connection)
        .await.expect("Query failed");

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": results
    }))
}

#[get("/network/list")]
async fn network_list(mut db: Connection<Db>, params: &Query_string) -> Custom<Value> {
    let request_authentication_output: Option<Request_authentication_output> = match request_authentication(db, None, params, "/network/list", false).await {
        Ok(data) => Some(data),
        Err(e) => None
    };
    if (request_authentication_output.is_none()) {
        return status::Custom(Status::Unauthorized, not_authorized());
    }

    let results = rover_network::table
        // .filter(rover_network::location.eq("onboard_client"))
        .select(Rover_network::as_select())
        .load(&mut request_authentication_output.unwrap().returned_connection)
        .await.expect("Query failed");

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": results
    }))
}

#[get("/processes/list")]
async fn processes_list(mut db: Connection<Db>, params: &Query_string) -> Custom<Value> {
    let request_authentication_output: Option<Request_authentication_output> = match request_authentication(db, None, params, "/processes/list", false).await {
        Ok(data) => Some(data),
        Err(e) => None
    };
    if (request_authentication_output.is_none()) {
        return status::Custom(Status::Unauthorized, not_authorized());
    }

    let results = rover_processes::table
        // .filter(rover_network::location.eq("onboard_client"))
        .select(Rover_processes::as_select())
        .load(&mut request_authentication_output.unwrap().returned_connection)
        .await.expect("Query failed");

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": results
    }))
}

#[delete("/<id>")]
async fn delete(mut db: Connection<Db>, id: i64) -> Result<Option<()>> {
    let affected = diesel::delete(posts::table)
        .filter(posts::id.eq(id))
        .execute(&mut db)
        .await?;

    Ok((affected == 1).then(|| ()))
}

#[delete("/")]
async fn destroy(mut db: Connection<Db>) -> Result<()> {
    diesel::delete(posts::table).execute(&mut db).await?;
    Ok(())
}

#[options("/<_..>")]
fn options_handler() -> &'static str {
    ""
}

/// Returns the current request's ID, assigning one only as necessary.
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
        rocket.attach(Db::init())
        .mount("/", routes![users_list, network_list, processes_list, login, authenticate, delete, destroy, options_handler])
        .mount("/device", routes![crate::device::device_list, crate::device::device_onboard, crate::device::device_update])
    })
}
