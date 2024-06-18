use rocket::serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::sql_types::*;
use crate::diesel_mysql::*;
use crate::device::*;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{MysqlPool, prelude::*};

// Incoming body structs
#[derive(Clone, Debug, Deserialize)]
pub struct Login_body {
    pub email: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct User_create_body {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Authenticate_Body {
    pub attempt_id: String,
    pub code: Option<i64>,
    pub public_key: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct System_users {
    pub username: String,
    pub is_admin: bool,
    pub permissions: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Device_startup_struct {
    pub os_type: String,
    pub os_version: Option<i64>,
    pub alias: Option<i64>,
    pub users: Vec<System_users>,
    pub rover_permissions: Vec<String>
}

// Table structs
#[derive(Database)]
#[database("diesel_mysql")]
pub struct Db(MysqlPool);

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
pub struct Post {
    #[serde(skip_deserializing)]
    id: Option<i64>,
    title: String,
    text: String,
    #[serde(skip_deserializing)]
    published: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = rover_users)]
pub struct Rover_users {
    #[serde(skip_deserializing)]
    pub id: String,
    pub email: String,
    pub admin_permission_flags: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = rover_devices)]
pub struct Rover_devices {
    // #[serde(skip_deserializing)]
    pub id: String,
    pub user_id: String,
    pub location: String,
    pub public_key: String,
    pub created: Option<i64>,
    pub active: bool,
    pub os_type: String,
    pub os_version: String,
    pub alias: String,
    pub compliant: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = rover_network)]
pub struct Rover_network {
    #[serde(skip_deserializing)]
    pub device_id: String,
    pub domain: String,
    pub ip_address: String,
    pub destination_country: String,
    pub destination_registrant: String,
    pub protocol: String,
    pub size: Option<i64>,
    pub info: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = rover_processes)]
pub struct Rover_processes {
    #[serde(skip_deserializing)]
    pub device_id: String,
    pub process: String,
    pub last_seen: Option<i64>,
    pub user: String,
    pub admin_user: String,
    pub is_admin_process: String,
    pub PID: Option<i64>,
    pub publisher: String,
    pub hash: String,
    pub threads: Option<i64>,
    pub size: Option<i64>,
    pub pathname: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = login_codes)]
pub struct Login_code_record {
    pub attempt_id: String,
    pub code: Option<i64>,
    pub created: Option<i64>,
    pub attempts: Option<i64>,
    pub user_id: String,
}

// Internal structs
#[derive(Debug)]
pub struct Query_string(pub String);

pub struct Request_authentication_output {
    pub returned_connection: Connection<Db>,
    // #[derive(Clone, Debug, Deserialize)]
    pub user_id: String,
    pub device_id: String
}