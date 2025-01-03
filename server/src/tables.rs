use rocket::serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::sql_types::*;
use crate::structs::*;

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
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        email -> Nullable<Text>,
        permission -> Nullable<BigInt>,
    }
}

diesel::table! {
    rover_devices (id) {
        id -> Text,
        user_id -> Text,
        public_key -> Text,
        created -> Nullable<BigInt>,
        active -> Nullable<Bool>,
        compliant -> Nullable<Bool>,
        os_type -> Nullable<Text>,
        os_version -> Nullable<Text>,
        location -> Nullable<Text>,
        alias -> Nullable<Text>
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
        created -> Nullable<BigInt>
    }
}
diesel::table! {
    rover_processes (device_id) {
        device_id -> Text,
        PID -> Nullable<BigInt>,
        process -> Nullable<Text>,
        user -> Nullable<Text>,
        admin_user -> Nullable<Bool>,
        is_admin_process -> Nullable<Bool>,
        publisher -> Nullable<Text>,
        hash -> Nullable<Text>,
        threads ->  Nullable<BigInt>,
        size ->  Nullable<BigInt>,
        pathname -> Nullable<Text>,
        last_seen ->  Nullable<BigInt>,
        created -> Nullable<BigInt>
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

diesel::allow_tables_to_appear_in_same_query!(rover_users, rover_devices, rover_network, rover_processes);