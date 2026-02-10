# Exercise 14 - Design realtime abuse masker

This exercise involves deploying a high-concurrency Rust environment. Implement a server-side masking logic where the "Abuse Dictionary" is fetched from S3 during the application’s boot sequence. A local text file will be used instead of s3 for simplicity.

## Problem Statement

Implement a high-performance masking layer for a WebSocket server. When a client broadcasts a message containing prohibited terms, the server must redact these terms (e.g., badword → *******) before broadcasting the payload to other connected peers.

## Design

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

## Implementation Overview

The system follows a Producer-Consumer pattern over WebSockets, optimized for low latency and high throughput.

1. Initialization Phase: On boot, the server reads the prohibited terms list from local storage. Every term is converted to lowercase before being compiled into a Succinct Trie (using LOUDS). This ensures the "source of truth" is normalized.

2. Shared State: The Trie is wrapped in an Arc (Atomic Reference Counter). This allows the server to spawn a unique asynchronous task for every connected client, with each task having read-only access to the same memory-resident Trie.

3. The Hot Path: For every incoming message, the server performs a sliding-window search. It uses common_prefix_search at each character index to find the longest prohibited match.

4. Client-Side Loop: The client utilizes a split-stream approach to handle bi-directional IO. While the client converts its own input to lowercase for consistency in this exercise, the server is designed to handle mixed-case input regardless.

## Setup

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
