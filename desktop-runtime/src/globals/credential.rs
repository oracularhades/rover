use serde_json::{json, Value};
use crate::{globals::macos::{retrieve_password, store_password}, structs::credential::Credential};

static SERVICE: &str = "rover_agent";

pub fn credentials_get() -> Credential {
    let mut output: Option<Credential> = None;
    if let Some(retrieved_password) = retrieve_password(SERVICE, "credential") {
        // println!("Retrieved password: {}", retrieved_password);
        output = Some(serde_json::from_str(&retrieved_password).expect("Failed to parse string"));
    } else {
        println!("Failed to retrieve password.");
    }

    return output.expect("Failed to get credentials");
}

pub fn credentials_set(host: String, device_id: String, private_key: String) -> Value {
    let credentials = json!({
        "host": host,
        "device_id": device_id,
        "private_key": private_key
    });

    store_password(SERVICE, "credential", &serde_json::to_string(&credentials).expect("Failed to serialize value"));

    return credentials;
}