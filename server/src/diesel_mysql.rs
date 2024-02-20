use rocket::fairing::AdHoc;
use rocket::response::{Debug, status::Created};
use rocket::serde::{Serialize, Deserialize, json::Json};

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{MysqlPool, prelude::*};

use diesel::prelude::*;
use diesel::sql_types::*;

use std::process::{Command, Stdio};
use std::env;

use std::fs::{File};
use std::io::Write;

use rand::prelude::*;

use crate::global::{ send_email, generate_random_id, is_null_or_whitespace };
use crate::structs::*;
use hades_auth::*;

use crate::structs::Db;

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
    login_codes (code) {
        attempt_id -> Text,
        code -> Nullable<BigInt>,
        created -> Nullable<BigInt>,
        attempts -> Nullable<BigInt>,
        user_id -> Text,
    }
}

#[get("/")]
async fn list(mut db: Connection<Db>) -> Result<Json<Vec<Post>>> {
    let results = posts::table
        // .filter(posts::published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(&mut db)
        .await?;
        // .expect("Error loading posts");
    
    // send_email("hi@oracularhades.com").await;

    Ok(Json(results))
}

#[post("/login", format = "application/json", data = "<body>")]
async fn login(mut db: Connection<Db>, mut body: Json<Login_body>) -> Result<&'static str> {
    // diesel::sql_function!(fn last_insert_id() -> BigInt);

    if (is_null_or_whitespace(body.email.clone())) {
        return Ok("body.email is null or whitespace.");
    }

    let result: Option<Rover_users> = rover_users::table
        .filter(rover_users::email.eq(body.email.clone()))
        .first(&mut db)
        .await
        .optional().expect("Something went wrong querying the DB.");

    if (result.is_none()) {
        return Ok("User not found");
    }

    let user = result.unwrap();

    let attempt_id = generate_random_id();
    let number: i32 = rand::thread_rng().gen_range(0..999999);

    send_email(body.email.clone(), "Your login code".to_string(), format!("Do not share this with anyone. This code serves no purpose except logging you into your account. If you didn't request this code, you can ignore it.\n\nLogin code: {}", number)).await;

    let code_insert = Login_code_record {
        attempt_id: attempt_id,
        code: Some(number.into()),
        created: Some(0),
        attempts: Some(0),
        user_id: user.id
    };
    diesel::insert_into(login_codes::table)
        .values(&code_insert)
        .execute(&mut db)
        .await?;

    Ok("lol")
    
    // Ok(Created::new("/").body(post))
}

#[post("/authenticate", format = "application/json", data = "<body>")]
async fn authenticate(mut db: Connection<Db>, mut body: Json<Authenticate_Body>) -> Result<&'static str> {
    // diesel::sql_function!(fn last_insert_id() -> BigInt);

    if (is_null_or_whitespace(body.attempt_id.clone())) {
        return Ok("body.attempt_id is null or whitespace.");
    }
    if (body.code.is_none()) {
        return Ok("body.code is null or whitespace.");
    }
    if (is_null_or_whitespace(body.public_key.clone())) {
        return Ok("body.public_key is null or whitespace.");
    }

    let result: Option<Login_code_record> = login_codes::table
        .filter(login_codes::attempt_id.eq(body.attempt_id.clone()))
        .first(&mut db)
        .await
        .optional().expect("Something went wrong querying the DB.");

    if (result.is_none()) {
        return Ok("Invalid attempt id");
    }

    let login_attempt_data = result.unwrap();

    if (login_attempt_data.code.is_none() || login_attempt_data.code != body.code) {
        // Invalid code, add an attempt.
        return Ok("Invalid code...make this an actual response.");
    }

    onboard_new_device(&body.public_key).await.expect("Fail to onboard device.");

    // diesel::insert_into(login_codes::table)
    //     .values(&code_insert)
    //     .execute(&mut db)
    //     .await?;

    Ok("lol")
    
    // Ok(Created::new("/").body(post))
}

#[get("/<id>")]
async fn read(mut db: Connection<Db>, id: i64) -> Option<Json<Post>> {
    posts::table
        .filter(posts::id.eq(id))
        .first(&mut db)
        .await
        .map(Json)
        .ok()
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

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket.attach(Db::init())
            .mount("/", routes![list, read, login, authenticate, delete, destroy])
    })
}
