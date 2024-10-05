use hades_auth::static_auth_sign;
use serde_json::{json, Value};
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio::sync::{mpsc, Mutex, watch};
use tokio::select;

use std::sync::Arc;

use crate::structs::credential::Credential;
use crate::CHANNEL;

use super::object::merge;

pub async fn websocket_connect_feed_channel(host: &str) {
    let (ws_stream, _) = connect_async(host.clone()).await.expect("Failed to connect");

    println!("WebSocket connection established! [client]");

    let (mut write, mut read) = ws_stream.split();

    let write = Arc::new(Mutex::new(write));
    let write_clone = Arc::clone(&write);

    // Write to websocket server.
    let connection_broadcast_thread = tokio::spawn(async move {
        // loop {
        println!("Write thread started");
        let mut rx = CHANNEL.1.lock().await;
        while let Some(received) = rx.recv().await {
            let mut write = write_clone.lock().await;
            write.send(Message::text(received)).await.expect("Failed to send message");
        }
        // }
    });

    loop {
        println!("LOOP [client]");
        select! {
            // Read from WebSocket
            msg = read.next() => {
                match msg {
                    Some(Ok(message)) => {
                        println!("Received [client]: {}", message);
                    }
                    Some(Err(e)) => {
                        eprintln!("Error receiving message [client]: {}", e);
                        break;
                    }
                    None => {
                        println!("Connection closed [client].");
                        break;
                    }
                }
            }
        }
    }
}

pub async fn websocket_event_builder(event: &str, data: &Value, default_data: &Value, credential: Credential) -> Value {
    let body = merge(&data, default_data);

    let jwt = static_auth_sign(&credential.private_key, body.clone()).await.expect("Failed to generate static_auth");
    
    let websocket_event = json!({
        "body": serde_json::to_string(&body).unwrap(),
        "_hades_websocket": json!({
            "event": event,
            "jwt": jwt
        })
    });

    return websocket_event;
}