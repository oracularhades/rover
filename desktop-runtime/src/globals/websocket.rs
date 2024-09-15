use serde_json::{json, Value};
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio::sync::{mpsc, Mutex, watch};
use tokio::select;

use std::sync::Arc;

use crate::CHANNEL;

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

pub fn websocket_event_builder(event: &str, data: &Value, default_data: &Value) -> Value {
    let websocket_event = json!({
        "default_data": default_data,
        "data": data,
        "_system_websocket_event": event
    });

    return websocket_event;
}