use futures_util::{SinkExt, StreamExt};
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

#[tokio::main]
async fn main() {
    let url = Url::parse("ws://127.0.0.1:8080").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Successfully connected to server.");
    println!("Type messages below and press Enter to send:");

    // 1. SPLIT STREAM: Decouple 'sending' from 'receiving' to allow full-duplex chat.
    let (mut write, mut read) = ws_stream.split();

    // 2. BACKGROUND RECEIVER: Listen for masked messages from the server.
    let receive_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(text) = msg {
                println!("\n[Server]: {}", text);
            }
        }
    });

    // 2. Main loop for reading terminal input
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    // 3. ASYNC STDIN: Non-blocking terminal input.
    while let Ok(Some(line)) = reader.next_line().await {
        if line.trim().is_empty() { continue; }
        
        // Send the typed line to the server
        if write.send(Message::Text(line)).await.is_err() {
            break;
        }
    }

    // Wait for the receive task to finish if the connection closes
    let _ = receive_task.await;
}