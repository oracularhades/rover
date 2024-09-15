#[macro_use] extern crate rocket;

// #[cfg(test)] mod tests;
pub struct Cors;

mod diesel_mysql;
mod global;
mod structs;
mod responses;
mod tables;
mod database;

pub mod globals {
    pub mod environment_variables;
}

pub mod endpoint {
    pub mod device;
    pub mod network;
    pub mod process;
    pub mod user;
}

pub mod websocket {
    pub mod connection;
    pub mod event;
}

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response, request, request::FromRequest};
use websocket::connection::handle_connection;

use std::error::Error;
use std::fs;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use once_cell::sync::Lazy;
use toml::Value;

use crate::responses::*;
use crate::structs::*;
use crate::database::{ validate_sql_table_inputs };

use diesel::MysqlConnection;
use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::r2d2::{self, ConnectionManager};

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex, watch};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message, tungstenite::protocol::CloseFrame};
use futures_util::{StreamExt, SinkExt};

pub static CHANNEL: Lazy<(Mutex<mpsc::UnboundedSender<Message>>, Mutex<mpsc::UnboundedReceiver<Message>>)> = Lazy::new(|| {
    let (tx, rx) = mpsc::unbounded_channel(); // Use tokio's mpsc::channel instead of std::sync
    (Mutex::new(tx), Mutex::new(rx))
});

// Create a type alias for the connection pool
type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

// Create a Lazy static variable for the connection pool
static DB_POOL: Lazy<Pool> = Lazy::new(|| {
    let manager = ConnectionManager::<MysqlConnection>::new(crate::database::get_default_database_url());
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
});

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
    }

    let config: Value = toml::from_str(&config_value).unwrap();

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

    // Bind the TCP listener to the address
    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("Listening on: {}", addr);

    tokio::spawn(async move {
        // Accept incoming connections
        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(handle_connection(stream));
        }
    });
    
    let figment = rocket::Config::figment();

    rocket::custom(figment)
        .attach(Cors)
        .attach(diesel_mysql::stage())
        .register("/", catchers![internal_error])
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