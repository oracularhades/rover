use rocket::serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::sql_types::*;
use crate::diesel_mysql::*;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{MysqlPool, prelude::*};

#[derive(Clone, Debug, Deserialize)]
pub struct Login_body {
    pub email: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct Authenticate_Body {
    pub attempt_id: String,
    pub code: Option<i64>,
    pub public_key: String
}

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
    pub username: String,
    pub admin_permission_flags: Option<i64>,
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