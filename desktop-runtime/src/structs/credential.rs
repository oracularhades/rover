use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Credential {
    pub host: String,
    pub device_id: String,
    pub private_key: String
}