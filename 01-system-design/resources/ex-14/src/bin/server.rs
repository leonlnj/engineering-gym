use std::sync::Arc;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use trie_rs::{Trie, TrieBuilder};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. DATA INGESTION: Load dictionary. In production, this would be an S3 fetch.
    let content = std::fs::read_to_string("abuse.txt").expect("Create abuse.txt");

    // 2. TRIE CONSTRUCTION: Build a memory-efficient prefix tree.
    let mut builder = TrieBuilder::new();
    for line in content.lines() { builder.push(line.to_lowercase()); }
    // Arc allows us to share the Trie across multiple threads without cloning the data.
    let trie: Arc<Trie<u8>> = Arc::new(builder.build());
    
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server live at 127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept().await {
        let trie = Arc::clone(&trie);

        // 3. CONCURRENCY: Spawn a new task for every connection.
        tokio::spawn(async move {
            let Ok(ws) = accept_async(stream).await else { return };
            let (mut write, mut read) = ws.split();

            // 4. STREAM HANDLING: Listen for text messages.
            while let Some(Ok(Message::Text(input))) = read.next().await {
                let masked = mask_text(&input, &trie);
                let _ = write.send(Message::Text(masked)).await;
            }
        });
    }
    Ok(())
}

/// Perform in-place masking of prohibited terms using the Trie.
fn mask_text(input: &str, trie: &Trie<u8>) -> String {
    let mut bytes = input.to_lowercase().as_bytes().to_vec();
    let mut i = 0;

    // Use a sliding index to check every possible start position for a prohibited word.
    while i < bytes.len() {
        // common_prefix_search finds all dictionary entries that match the start of &bytes[i..]
        let match_len = trie.common_prefix_search(&bytes[i..])
            .map(|m: Vec<u8>| m.len())
            .max();  // If multiple matches (e.g., 'bad' and 'badword'), take the longest.

        if let Some(len) = match_len {
            // Overwrite the specific range in the byte vector with '*' characters.
            bytes[i..i + len].fill(b'*');
            i += len;  // Jump index to the end of the masked word.
        } else {
            i += 1;
        }
    }
    // OPTIMIZATION: We skip UTF-8 re-validation because '*' is a valid 1-byte UTF-8 char.
    unsafe { String::from_utf8_unchecked(bytes) }
}