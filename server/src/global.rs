use std::process::{Command, Stdio};
use std::env;

use std::fs::{File};
use std::io::Write;

use rand::prelude::*;

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