use std::process::{Command, Stdio};
use std::error::Error;
use std::collections::HashMap;

use std::fs::{File};
use std::io::Write;
use url::Url;

use rand::prelude::*;

use crate::structs::*;
use crate::device::*;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{MysqlPool, prelude::*};

use hades_auth::authenticate;

pub async fn send_email(email: String, subject: String, message: String) {
    println!("going");
    let mut file = File::create("/tmp/test.txt").expect("Something went wrong creating the file.");
    file.write_all(format!("Subject: {}\nheaders:\n  From: noreply@paperplane.motionfans.com\n  Subject: {}\n\n{}", subject, subject, message).as_bytes());

    // Create a Command to execute the echo command
    let echo_command = Command::new("cat")
        .arg("/tmp/test.txt")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute echo command");

    // Use output of echo as input for msmtp command
    let mut msmtp_command = Command::new("msmtp")
        .args(&["-f", "noreply@paperplane.motionfans.com", &email])
        .stdin(Stdio::from(echo_command.stdout.expect("Failed to get echo stdout")))
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute msmtp command");

    // Wait for the msmtp command to finish and get its exit status
    let status = msmtp_command
        .wait()
        .expect("Failed to wait for msmtp command");

    if !status.success() {
        eprintln!("Error: Command execution failed");
        // Optionally, you can handle the error further
    } else {
        println!("Command executed successfully");
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
        pathname,
        false
    ).await.expect("Authentication failed");

    println!("Auth didn't fail");

    return Ok(Request_authentication_output {
        returned_connection: db,
        device_id: device_id,
        user_id: user_id
    });
}