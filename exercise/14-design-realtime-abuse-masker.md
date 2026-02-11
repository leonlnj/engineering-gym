# Exercise 14 - Real-Time Content Moderation (WebSocket)

Implement high-performance content masking for WebSocket messages using Trie-based dictionary lookups in Rust.

**Objectives**:
1. Build WebSocket server/client using tokio-tungstenite
2. Load abuse dictionary into memory-efficient Trie at startup
3. Implement concurrent message handling (one task per connection)
4. Mask prohibited terms in real-time using sliding window search

## Problem Statement

Implement a high-performance masking layer for a WebSocket server. When a client broadcasts a message containing prohibited terms, the server must redact these terms (e.g., badword → *******) before broadcasting the payload to other connected peers.

## Context

### Architectural Design

#### Option 1: Tokenize and lookup

- Load all abuse in a dictionary 
- Tokenize incoming message
- Check each word if it is an abuse
- If yes: mask and update
- If no: copy the token

Pros:
- Easy to implement using standard libraries.

Cons:
- String allocation for each token increases memory pressure. 
- Logic Gaps: Fails to catch multi-word phrases or "embedded" abuse (e.g., "wordwithbadwordinside") unless  complex regex or sliding windows are used.

### Option 2: Trie

- Store abuses in Trie
- Traverse Trie data structure

Pros:
- Efficiency: Search time is independent of the dictionary size; it depends only on the length of the input message.

Cons:
- State Management: Since standard databases (SQL/NoSQL) don't offer native Trie structures, the Trie must be built in-memory on the application node.
- Latency: Avoid external API calls per message; the Trie must remain local to the WebSocket process to maintain real-time constraints.

Efficient but no database offer Trie datastructure. Building an external service to hold the abuse is has bad performance if every message require a HTTP call.
Hence the Trie will be stored and init from an external storage by the websocket server as the optimal approach.

## Setup

### Implementation Overview

The system follows a Producer-Consumer pattern over WebSockets, optimized for low latency and high throughput.

1. Initialization Phase: On boot, the server reads the prohibited terms list from local storage. Every term is converted to lowercase before being compiled into a Succinct Trie (using LOUDS). This ensures the "source of truth" is normalized.

2. Shared State: The Trie is wrapped in an Arc (Atomic Reference Counter). This allows the server to spawn a unique asynchronous task for every connected client, with each task having read-only access to the same memory-resident Trie.

3. The Hot Path: For every incoming message, the server performs a sliding-window search. It uses common_prefix_search at each character index to find the longest prohibited match.

4. Client-Side Loop: The client utilizes a split-stream approach to handle bi-directional IO. While the client converts its own input to lowercase for consistency in this exercise, the server is designed to handle mixed-case input regardless.

### Running the Application

In `exercise/resources/ex-14`, execute the server and client code in separate terminals:

```sh
cargo run --bin server
cargo run --bin client
```

## Test

Testing confirms that the Trie identifies prohibited terms regardless of user casing or word boundaries.

```sh
# Partial match (no mask)
badwor      # Output: [Server]: badwor
# Exact match
badword     # Output: [Server]: *******
# Case-insensitivity & Substring matching
bardwoRDx   # Output: [Server]: *******x
# Case-insensitivity & Multiple words
badwordjerk # Output: [Server]: **********
```

## Cleanup

Stop the server and client processes (Ctrl+C in both terminals).
