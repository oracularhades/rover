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