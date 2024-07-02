use std::process::{Command, Stdio};
use std::error::Error;
use std::collections::HashMap;
use std::fs::{File};
use std::io::Write;
use std::env;

use crate::CONFIG_VALUE;
use crate::structs::*;
use crate::tables::*;

use url::Url;
use rand::prelude::*;

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{MysqlPool, prelude::*};

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use hades_auth::authenticate;

pub async fn send_email(email: String, subject: String, message: String) -> Result<bool, Box<dyn Error>> {
    // Set limit on email characters, in-case someone wants to have a laugh. 500 is very generous.
    if (email.len() > 500) {
        return Err("The email provided is over 500 characters.".into());
    }

    let smtp_json = serde_json::to_string(&CONFIG_VALUE["smtp"]).expect("Failed to serialize");
    let smtp: Config_smtp = serde_json::from_str(&smtp_json).expect("Failed to parse");

    // NOTE: We're not stupid, Lettre validates the input here via .parse. It's absolutely vital .parse is here for safety.

    let from = format!("{} <{}>", smtp.from_alias.expect("Missing from_alias"), smtp.from_header.clone().expect("Missing from_header"));
    let mut reply_to = format!("<{}>", smtp.from_header.expect("Missing from_header"));
    let to = format!("<{}>", email);

    if (smtp.reply_to_address.is_none() == false) {
        reply_to = format!("<{}>", smtp.reply_to_address.expect("Missing reply_to_address"));
    }

    // NOTE: IT IS ABSOLUTELY VITAL .PARSE IS HERE, ON ALL INPUTS, FOR SAFETY. Lettre validates the input here via .parse, injection is possible without .parse.
    let mut email_packet = Message::builder()
    .from(from.parse().unwrap())
    .reply_to(reply_to.parse().unwrap())
    .to(to.parse().unwrap())
    .subject(subject)
    .header(ContentType::TEXT_PLAIN)
    .body(String::from(message))
    .unwrap();

    // Check for password and get it.
    let mut password: String = String::new();
    if let Some(val) = env::var(smtp.password_env.expect("Missing password_env")).ok() {
        password = val;
    } else {
        return Err("The environment variable specified in config.smtp.password_env is missing.".into());
    }

    let creds = Credentials::new(smtp.username.expect("Missing username"), password);

    // Open a remote connection to SMTP server
    let mailer = SmtpTransport::relay(&smtp.host.expect("Missing host"))
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email_packet) {
        Ok(_) => Ok(true),
        Err(e) => Err("Could not send email: {e:?}".into()),
    }
}

pub fn generate_random_id() -> String {
    let mut random_string = String::new();
    const CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for _ in 0..CHARACTERS.len() {
        let index = rand::thread_rng().gen_range(0..CHARACTERS.len());
        random_string.push(CHARACTERS.chars().nth(index).unwrap());
    }
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    random_string.truncate(20);
    random_string + &timestamp.to_string()
}

pub fn is_null_or_whitespace(s: String) -> bool {
    match s {
        string if string == "null" || string == "undefined" => true,
        string => string.trim().is_empty(),
    }
}

pub async fn request_authentication(mut db: Connection<Db>, body: Option<String>, params: &Query_string, pathname: &str, use_cropped_body: bool) -> Result<Request_authentication_output, Box<dyn Error>> {
    let mut params_object: HashMap<String, String> = HashMap::new();
    let params_string: String = params.0.clone();
    if !params_string.is_empty() {
        params_object = Url::parse(&format!("http://localhost/?{}", params_string))
        .map(|url| url.query_pairs().into_owned().collect())
        .unwrap_or_default();
    }

    println!("params: {:?}", params_object);
    println!("url: {:?}", &format!("http://localhost/?{}", params_string));

    if (params_object.get("deviceid").is_none()) {
        // throw an error.
    }

    let device_id = match params_object.get("deviceid") {
        Some(id) => id.clone(),
        None => return Err("Missing deviceid parameter".into()), // Handle missing deviceid gracefully
    };

    println!("2 {}", device_id.clone());
    
    if (params_object.get("authenticator_JWT_Token").is_none()) {
        // throw an error.
    }
    let jwt = match params_object.get("authenticator_JWT_Token") {
        Some(id) => id.clone(),
        None => return Err("Missing authenticator_JWT_Token parameter".into()), // Handle missing deviceid gracefully
    };

    println!("3");
    
    let result: Option<Rover_devices> = rover_devices::table
        .filter(rover_devices::id.eq(&device_id))
        .first(&mut db)
        .await
        .optional().expect("Something went wrong querying the DB1.");

    println!("4");

    if (result.is_none()) {
        return Err("Authentication failed [device doesn't exist]".into())
    }

    let device = result.unwrap();

    println!("5");

    let public_key = device.public_key;
    let user_id = device.user_id;

    println!("6");

    authenticate(
        body,
        params_object,
        &jwt,
        &public_key,
        &format!("/api{}", pathname),
        false
    ).await.expect("Authentication failed");

    println!("Auth didn't fail");

    return Ok(Request_authentication_output {
        returned_connection: db,
        device_id: device_id,
        user_id: user_id
    });
}