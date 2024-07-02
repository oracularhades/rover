#[macro_use] extern crate rocket;
// #[macro_use] extern crate rocket_sync_db_pools;

// #[cfg(test)] mod tests;
pub struct Cors;

mod diesel_mysql;
mod device;
mod global;
mod structs;
mod responses;
mod tables;
mod database;

pub mod globals {
    pub mod environment_variables;
}

// use diesel::r2d2;
// use diesel::r2d2::ConnectionManager;
// use diesel::r2d2::Pool;
// use diesel::mysql::MysqlConnection;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response, request, request::FromRequest};

use std::error::Error;
use std::fs;
use std::collections::HashMap;
use std::env;

use once_cell::sync::Lazy;
use toml::Value;

use crate::responses::*;
use crate::structs::*;
use crate::database::{ validate_sql_table_inputs, check_database_environment };

// type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

// static POOL: Lazy<DbPool> = Lazy::new(|| {
    // let sql_json = serde_json::to_string(&CONFIG_VALUE["database"]["mysql"]).expect("Failed to serialize");
    // let sql: Config_database_mysql = serde_json::from_str(&sql_json).expect("Failed to parse");

    // let password_env = environment_variables::get(sql.password_env.clone().expect("config.sql.password_env is missing.")).expect(&format!("The environment variable specified in config.sql.password_env ('{:?}') is missing.", sql.password_env.clone()));

    // let username = sql.username.expect("Missing username.");
    // let hostname = sql.hostname.expect("Missing hostname.");
    // let port = sql.port.expect("Missing port.");
    // let database = sql.database.expect("Missing database.");

    // let database_url = database::create_database_url(username, password_env, hostname, port, database);
    // let manager = ConnectionManager::<MysqlConnection>::new(database_url);
//     r2d2::Pool::builder()
//         .build(manager)
//         .expect("Failed to create pool.")
// });

pub static CONFIG_VALUE: Lazy<Value> = Lazy::new(|| {
    get_config().expect("Failed to get config")
});

pub static SQL_TABLES: Lazy<Config_sql> = Lazy::new(|| {
    let (sql_tables, raw_sql_tables) = get_sql_tables().expect("failed to get_sql_tables()");
    sql_tables
});

fn get_config() -> Result<Value, Box<dyn Error>> {
    let mut config_value: String = String::new();
    if let Some(val) = env::var("rover_config").ok() {
        println!("Value of rover_config: {}", val);

        config_value = val;
    } else {
        return Err("Missing \"rover_config\" environment variable".into());
        // ROCKET_DATABASES is not set
    }

    // let contents = fs::read_to_string("./config.toml")
    //     .expect("Should have been able to read the file");

    let config: Value = toml::from_str(&config_value).unwrap();

    // let value = contents.parse::<toml::Value>().expect("lmao");
    // let table = value.as_table().unwrap();
    // let auth_methods = table.get("authentication_methods").unwrap().as_table().unwrap();

    // let mut valid: Option<AuthMethod> = None;
    // for (key, value) in auth_methods {
    //     if (key.to_string() == id) {
    //         valid = Some(serde_json::from_str(&value.to_string()).expect(&format!("Failed to parse authentication method: {}", key)));
    //     }
    // }

    Ok(config)
}

fn get_sql_tables() -> Result<(Config_sql, Value), String> {
    let config_value_sql = CONFIG_VALUE.get("sql");
    if (config_value_sql.is_none() == true) {
        return Err("Missing config.sql".into());
    }
    let config_value_sql_tables = config_value_sql.unwrap().get("tables");
    if (config_value_sql_tables.is_none() == true) {
        return Err("Missing config.sql.tables".into());
    }

    let sql_json = serde_json::to_string(&config_value_sql_tables).expect("Failed to serialize");
    let sql: Config_sql = serde_json::from_str(&sql_json).expect("Failed to parse");

    return Ok((sql, config_value_sql_tables.unwrap().clone()));
}

#[catch(500)]
fn internal_error() -> serde_json::Value {
    error_message("Internal server error")
}

#[launch]
async fn rocket() -> _ {
    let (unsafe_do_not_use_sql_tables, unsafe_do_not_use_raw_sql_tables) = get_sql_tables().unwrap();
    validate_sql_table_inputs(unsafe_do_not_use_raw_sql_tables).await.expect("Config validation failed.");
    check_database_environment().await.expect("Check database environment failed.");

    rocket::build()
    .register("/", catchers![internal_error])
    // .attach(Cors)
    .attach(diesel_mysql::stage())
}

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.remove_header("server");
    }
}