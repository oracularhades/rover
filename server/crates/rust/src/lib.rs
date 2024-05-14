use std::time::{SystemTime, UNIX_EPOCH};
use oracularhades_mirror_frank_jwt::{Algorithm, encode, decode};
use serde::Serialize;
use serde_json::{json, Value};
use reqwest;
use url::Url;
use openssl::ec::{EcGroup, EcKey};
use openssl::nid::Nid;
// use web_sys::{window, FormData};
use std::collections::HashMap;
use std::error::Error;
use sha2::{Sha256, Sha512, Digest};
use hex_literal::hex;

mod globals;
pub mod structs;

use crate::globals::{get_creds, resolve_txt_promise, url_sanitize, is_valid_hostname, unsafe_noverification_jwt_payload_decode, VerifyJWT, is_null_or_whitespace, generate_random_id};
use crate::structs::*;

use oracularhades_mirror_frank_jwt::ValidationOptions;

pub async fn Sign(params: HashMap<String, String>, body: Option<&str>, private_key: &str, only_use_field_for_body: Option<&str>) -> Result<Sign_output, String> {
    let mut unsorted_data: HashMap<String, String> = HashMap::new();

    // Copy metadata into unsorted_data
    unsorted_data.extend(params.into_iter());

    if let Some(body_str) = body {
        let hash_hex: String;
        let mut hash_hex_file: Option<String> = None;

        let mut hasher = Sha512::new();
        hasher.update(body_str);
        let result = hasher.finalize();
        println!("{:?}", Sha512::digest(result.clone()));
        hash_hex = hex::encode(result);

        // if let Some(only_use_field) = only_use_field_for_body {
        //     let form_data = FormData::new().unwrap();
        //     form_data.append_with_str(only_use_field, body_str).unwrap();
        //     let hash_field = get_formdata_field_hash(&form_data, only_use_field).await?;
        //     hash_hex_file = Some(hash_field);
        // }

        // Update unsorted_data with body SHA512 hash
        unsorted_data.insert("body_sha512".to_string(), hash_hex);

        // Update unsorted_data with file SHA512 hash if applicable
        if let Some(hash_hex_file) = hash_hex_file {
            unsorted_data.insert("just_file_sha512".to_string(), hash_hex_file);
        }
    }

    let mut keys: Vec<String> = unsorted_data.keys().cloned().collect();
    keys.sort();

    let mut data: HashMap<String, String> = HashMap::new();
    for key in keys.iter() {
        if key != "authenticator_JWT_Token" {
            if let Some(value) = unsorted_data.get(key) {
                data.insert(key.clone(), value.clone());
            }
        }
    }

    // Serialize sorted map to JSON
    let data_json = serde_json::to_string(&data).expect("Fail");

    // Calculate SHA512 checksum
    let mut hasher = Sha512::new();
    hasher.update(data_json);
    let output_sha512_checksum = hasher.finalize();
    let output_sha512_checksum_hex = output_sha512_checksum.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

    // Create JWT data
    let jwt_data = json!({
        "checksum": output_sha512_checksum_hex,
        "body_checksum": data.get("body_sha512").unwrap_or(&"".to_string()),
        "just_file_sha512": data.get("just_file_sha512").unwrap_or(&"".to_string())
    });

    let mut header = json!({});
    let jwt = encode(header, &private_key, &jwt_data, Algorithm::ES512).expect("JWT signing failed.");

    println!("HMMM: {:?}", data);

    Ok(Sign_output {
        params: "".to_string(),
        jwt: jwt
    })
}

pub async fn authenticate(
    body: Option<String>,
    params: HashMap<String, String>,
    jwt: &str,
    public_key: &str,
    pathname: &str,
    use_cropped_body: bool,
) -> Result<bool, String> {
    let mut keys: Vec<String> = Vec::new();
    let mut unsorted_data: HashMap<String, String> = HashMap::new();

    unsorted_data.extend(params.into_iter());

    keys.extend(unsorted_data.keys().cloned());
    keys.sort();

    let mut data: HashMap<String, String> = HashMap::new();
    for key in keys.iter() {
        if key != "authenticator_metadata" && key != "authenticator_JWT_Token" {
            if let Some(value) = unsorted_data.get(key) {
                data.insert(key.clone(), value.clone());
            }
        }
    }

    // Make sure deviceid and JWT_Token are specified.
    if is_null_or_whitespace(jwt) {
        return Err("JWT is null or whitespace.".to_string());
    }

    // Check the pathname inside the signed-object.
    if let Some(data_pathname) = data.get("authenticator_pathname") {
        if !is_null_or_whitespace(data_pathname) {
            if data_pathname != pathname {
                return Err(format!(
                    "Signed URL is \"{}\" and does not match \"{}\"",
                    data_pathname,
                    pathname
                ));
            }
        }
    }

    let public_key_pem = format!(
        "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----\n",
        public_key
    );
    let verify_jwt_status_value = VerifyJWT(jwt.to_string(), public_key_pem.to_string()).expect("Failed to verify jwt");
    let verify_jwt_status: Signed_data = serde_json::from_str(&verify_jwt_status_value).unwrap();

    let mut sha512_authed_checksum = String::new();
    let mut body_sha512_authed_checksum = String::new();

    let sha512_authed_checksum_v = verify_jwt_status.checksum.unwrap();
    sha512_authed_checksum = sha512_authed_checksum_v;

    if let Some(body_checksum) = verify_jwt_status.body_checksum {
        body_sha512_authed_checksum = body_checksum;
    }
    if use_cropped_body {
        if let Some(just_file_sha512) = verify_jwt_status.just_file_sha512 {
            body_sha512_authed_checksum = just_file_sha512;
        } else {
            return Err(
                "use_cropped_body is true - however, jwt.just_file_sha512 is null."
                    .to_string(),
            );
        }
    }

    println!("data {:?}", data.clone());

    let data_new = serde_json::to_string(&data).unwrap();
    println!("data_new {}", data_new.clone());

    // todo: there isn't anything checking for null with values like the checksum, so they could be null, you probably won't get very far, but is a good thing to implement.
    
    // println!("data {:?}", data.clone());
    // println!("data_new {:?}", data_new.clone()); // .replace(", \"", ",\"")

    let mut hasher = Sha512::new();
    hasher.update(data_new);
    let result = hasher.finalize();
    println!("{:?}", Sha512::digest(result.clone()));
    if sha512_authed_checksum != hex::encode(result) {
        return Err("Incoming data does not match checksum in JWT packet.".to_string());
    }

    if let Some(body) = body {
        if body_sha512_authed_checksum.is_empty() {
            return Err(
                "Body was provided - however, jwt.body_checksum is null.".to_string(),
            );
        }

        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(&body.as_bytes()).unwrap().as_bytes());
        let result = hasher.finalize();
        println!("body {:?}", Sha512::digest(result.clone()));

        if body_sha512_authed_checksum != hex::encode(result) {
            return Err(
                "Incoming body data does not match checksum in JWT packet.".to_string(),
            );
        }
    }

    Ok(true)
}

async fn verify_jwt(jwt_string: &str, public_key: &str) -> Result<Value, String> {
    let public_key_pem = format!(
        "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----\n",
        public_key
    );

    println!("JWT {}", jwt_string);
    println!("public_key {}", public_key_pem);

    if is_null_or_whitespace(jwt_string) {
        return Err("jwt is null.".to_string());
    }
    if is_null_or_whitespace(public_key) {
        return Err("public_key is null or whitespace".to_string());
    }

    let output = decode(&jwt_string, &public_key_pem, Algorithm::ES512, &ValidationOptions::default());

    match output {
        Ok((jwt_header, jwt_payload)) => {
            return Ok(jwt_payload)
        }
        Err(err) => {
            println!("{:?}", err);
            return Err("Unacceptable JWT.".to_string())
        }
    }
}

pub fn GenerateKeyPair() -> Keypair {
    // Create a new Elliptic Curve group using the P-521 curve (NIST curve)
    let ec_group = EcGroup::from_curve_name(Nid::SECP521R1).expect("Failed to create EC group");

    // Generate a new private key
    let private_key = EcKey::generate(&ec_group).expect("Failed to generate private key");

    // Get private key in PEM format
    let private_key_pem = private_key
        .private_key_to_pem()
        .expect("Failed to convert private key to PEM format");

    // Get public key in PEM format
    let public_key_pem = private_key
        .public_key_to_pem()
        .expect("Failed to convert public key to PEM format");

    // // Print private and public keys
    // println!("ES512 Private Key:");
    // println!("{}", String::from_utf8_lossy(&private_key_pem));
    // println!("ES512 Public Key:");
    // println!("{}", String::from_utf8_lossy(&public_key_pem));

    let keys = Keypair {
        private_key: String::from_utf8_lossy(&private_key_pem).to_string(),
        public_key: String::from_utf8_lossy(&public_key_pem).to_string()
    };
    return keys;
}

pub async fn onboard_new_device(public_key: &str) -> Result<DeviceDetails, Box<dyn Error>> {
    if public_key.trim().is_empty() {
        return Err("public_key is null or whitespace.".into());
    }

    let device_id = generate_random_id();

    // let public_key_import = import_elliptic_public_key(&format!("-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----", public_key)).await?;
    // if public_key_import.usages()[0] != "verify" || public_key_import.usages().len() != 1 {
    //     return Err("Public key usages array must be [\"sign\", \"verify\"]".into());
    // }
    // if public_key_import.algorithm().name() != "ECDSA" {
    //     return Err("Algorithm name must be ECDSA".into());
    // }
    // // if public_key_import.algorithm().named_curve() != "P-521" {
    // //     return Err("namedCurve must be P-521.".into());
    // // }
    // if public_key_import.key_type() != "public" {
    //     return Err("Key type MUST be public key. It is extremely insecure to surrender your private authentication key. You should consider the provided RSA key compromised, please generate a new key.".into());
    // }

    Ok(DeviceDetails { ok: true, device_id })
}