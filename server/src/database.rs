use std::fmt::format;
use std::process::{Command, Stdio};
use std::error::Error;
use std::collections::{HashMap};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use std::fs::{File};
use std::io::Write;
use url::Url;

use rand::prelude::*;

use crate::globals::environment_variables;
use crate::structs::*;
use crate::tables::*;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{MysqlPool, prelude::*};
use regex::Regex;
use std::env;

use crate::CONFIG_VALUE;

fn validate_table_name(input: &str) -> bool {
    let re = Regex::new(r"^[A-Za-z1-9]+$").unwrap();
    re.is_match(input)
}

pub async fn validate_sql_table_inputs() -> Result<bool, Box<dyn Error>> {
    if let Ok(current_dir) = env::current_dir() {
        if let Some(path) = current_dir.to_str() {
            println!("Current directory: {}", path);
        } else {
            println!("Failed to get current directory path.");
        }
    } else {
        println!("Failed to get current directory.");
    }

    let value = (*CONFIG_VALUE).clone();
    let table = value.as_table().unwrap();
    let sql_tables = table.get("sql").unwrap().as_table().unwrap();

    for (key, value) in sql_tables {
        if let Some(table_name) = value.as_str() {
            let output = validate_table_name(table_name);
            if (output != true) {
                return Err(format!("\"{}\" does not match A-Za-z1-9. This is necessary for SQL security, as table names are not bind-able.", key).into());
            }
        }
    }

    Ok(true)
}

pub fn create_database_url(username: String, password: String, hostname: String, port: i64, database: String) -> String {
    return format!("mysql://{}:{}@{}:{}/{}", username, password, hostname, port, database);
}

pub fn get_default_database_url() -> String {
    let sql_json = serde_json::to_string(&CONFIG_VALUE["database"]["mysql"]).expect("Failed to serialize");
    let sql: Config_database_mysql = serde_json::from_str(&sql_json).expect("Failed to parse");

    let password_env = environment_variables::get(sql.password_env.clone().expect("config.sql.password_env is missing.")).expect(&format!("The environment variable specified in config.sql.password_env ('{:?}') is missing.", sql.password_env.clone()));

    let username = sql.username.expect("Missing username.");
    let hostname = sql.hostname.expect("Missing hostname.");
    let port = sql.port.expect("Missing port.");
    let database = sql.database.expect("Missing database.");

    return create_database_url(username, password_env, hostname, port, database);
}