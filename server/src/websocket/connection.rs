use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex, watch};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message, tungstenite::protocol::CloseFrame};
use futures_util::{StreamExt, SinkExt};
use std::sync::Arc;

pub async fn handle_connection(stream: TcpStream) {
    // Upgrade the TCP stream to a WebSocket connection
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    println!("New WebSocket connection established [server]");

    let (mut write, mut read) = ws_stream.split();

    let write = Arc::new(Mutex::new(write));
    let write2_clone = Arc::clone(&write);
    
    loop {
        // TODO: Set an actual condition here. This "break;" will cause the connection to hang, but the actual disconnect packet doesn't work for some reason.
        if (1 == 2) {
            // write.send(Message::Close(None)).await.expect("Failed to disconnect client gracefully.");
            //// write.close().await.expect("Failed to close websocket.");
            break;
        }

        if let Some(msg) = read.next().await {
            match msg {
                Ok(message) => {
                    println!("Received [server]: {}", message);
                    let mut write = write2_clone.lock().await;

                    // TODO: an actual condition needs to be set here. 
                    // write.send(message).await.expect("Failed to send message");
                }
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                }
            }
        }
    }

    println!("WebSocket connection closed [server]");
}