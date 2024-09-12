use diesel::prelude::*;
use crate::tables::*;
use rocket::serde::{Serialize, Deserialize};
use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

// #[database("mysql_db")]
// struct DbConn(MysqlConnection);

// Incoming body structs
#[derive(Clone, Debug, Deserialize)]
pub struct Login_body {
    pub email: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct User_update_body {
    pub action: Option<String>,
    pub id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub permission: Option<i64>
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

// Internal structs
#[derive(Debug)]
pub struct Query_string(pub String);

pub struct Request_authentication(pub Option<Request_authentication_output>);

pub struct Request_authentication_output {
    // pub returned_connection: &MysqlConnection,
    // #[derive(Clone, Debug, Deserialize)]
    pub user_id: String,
    pub device_id: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config_sql {
    pub user: Option<String>,
    pub device: Option<String>,
    pub magiclink: Option<String>,
    pub network: Option<String>,
    pub process: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config_database_mysql {
    pub username: Option<String>,
    pub password_env: Option<String>,
    pub hostname: Option<String>,
    pub port: Option<i64>,
    pub database: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config_smtp {
    pub host: Option<String>,
    pub port: Option<i64>,
    pub username: Option<String>,
    pub from_alias: Option<String>,
    pub from_header: Option<String>,
    pub reply_to_address: Option<String>,
    pub password_env: Option<String>
}

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

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName, Identifiable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = rover_users)]
pub struct Rover_users {
    #[serde(skip_deserializing)]
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub permission: Option<i64>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rover_users_data_for_admins {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub permission: Option<i64>
}

impl From<Rover_users> for Rover_users_data_for_admins {
    fn from(user: Rover_users) -> Self {
        Rover_users_data_for_admins {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            permission: user.permission
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName, Identifiable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = rover_devices)]
pub struct Rover_devices {
    #[serde(skip_deserializing)]
    pub id: String,
    pub user_id: String,
    pub public_key: String,
    pub created: Option<i64>,
    pub active: Option<bool>,
    pub compliant: Option<bool>,
    pub os_type: Option<String>,
    pub os_version: Option<String>,
    pub location: Option<String>,
    pub alias: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rover_devices_data_for_admins {
    pub id: String,
    pub user_id: String,
    pub created: Option<i64>,
    pub active: Option<bool>,
    pub compliant: Option<bool>,
    pub os_type: Option<String>,
    pub os_version: Option<String>,
    pub alias: Option<String>
}

impl From<Rover_devices> for Rover_devices_data_for_admins {
    fn from(user: Rover_devices) -> Self {
        Rover_devices_data_for_admins {
            id: user.id,
            user_id: user.user_id,
            created: user.created,
            active: user.active,
            compliant: user.compliant,
            os_type: user.os_type,
            os_version: user.os_version,
            alias: user.alias
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = rover_network)]
pub struct Rover_network {
    pub device_id: String,
    pub domain: String,
    pub ip_address: String,
    pub destination_country: String,
    pub destination_registrant: String,
    pub protocol: String,
    pub size: Option<i64>,
    pub info: String,
    pub created: Option<i64>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rover_network_data_for_admins {
    pub device: Option<Rover_devices_data_for_admins>,
    pub user: Option<Rover_users_data_for_admins>,
    pub domain: String,
    pub ip_address: String,
    pub destination_country: String,
    pub destination_registrant: String,
    pub protocol: String,
    pub size: Option<i64>,
    pub info: String,
    pub created: Option<i64>,
}

impl From<(Rover_network, Option<Rover_devices>, Option<Rover_users>)> for Rover_network_data_for_admins {
    fn from((network, device, user): (Rover_network, Option<Rover_devices>, Option<Rover_users>)) -> Self {
        Rover_network_data_for_admins {
            device: device.map(|d| d.into()),
            user: user.map(|d| d.into()),
            domain: network.domain,
            ip_address: network.ip_address,
            destination_country: network.destination_country,
            destination_registrant: network.destination_registrant,
            protocol: network.protocol,
            size: network.size,
            info: network.info,
            created: network.created
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = rover_processes)]
pub struct Rover_processes {
    pub device_id: String,
    pub process: Option<String>,
    pub last_seen: Option<i64>,
    pub user: Option<String>,
    pub admin_user: Option<bool>,
    pub is_admin_process: Option<bool>,
    pub PID: Option<i64>,
    pub publisher: Option<String>,
    pub hash: Option<String>,
    pub threads: Option<i64>,
    pub size: Option<i64>,
    pub pathname: Option<String>,
    pub created: Option<i64>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rover_processes_data_for_admins {
    pub device: Option<Rover_devices_data_for_admins>,
    pub user: Option<Rover_users_data_for_admins>,
    pub process: Option<String>,
    pub last_seen: Option<i64>,
    pub admin_user: Option<bool>,
    pub is_admin_process: Option<bool>,
    pub PID: Option<i64>,
    pub publisher: Option<String>,
    pub hash: Option<String>,
    pub threads: Option<i64>,
    pub size: Option<i64>,
    pub pathname: Option<String>,
    pub created: Option<i64>
}

impl From<(Rover_processes, Option<Rover_devices>, Option<Rover_users>)> for Rover_processes_data_for_admins {
    fn from((process, device, user): (Rover_processes, Option<Rover_devices>, Option<Rover_users>)) -> Self {
        Rover_processes_data_for_admins {
            device: device.map(|d| d.into()),
            user: user.map(|d| d.into()),
            process: process.process,
            last_seen: process.last_seen,
            admin_user: process.admin_user,
            is_admin_process: process.is_admin_process,
            PID: process.PID,
            publisher: process.publisher,
            hash: process.hash,
            threads: process.threads,
            size: process.size,
            pathname: process.pathname,
            created: process.created
        }
    }
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