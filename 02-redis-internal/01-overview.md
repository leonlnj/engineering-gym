# Redis Overview

Redis (Remote Dictionary Server) is an open-source, in-memory, key-value data store. It is widely used as a database, cache, message broker, and streaming engine.

Because it keeps all its data in RAM rather than writing every change to a slow disk, it delivers sub-millisecond response times, frequently handling hundreds of thousands of operations per second.

---

## 1. Core Architecture Principles

*   **In-Memory Storage:** Everything lives in primary memory (RAM) for speed. 
*   **Single-Threaded Core:** Redis handles execution commands using a single event loop thread. This design choice eliminates the overhead of CPU context switching, locking, and race conditions, keeping operations atomic and predictable.
*   **Key-Value Structure:** At its foundation, Redis is a giant dictionary where keys (always strings) map to various rich data structures.

---

## 2. Supported Data Structures

Unlike traditional key-value stores that only allow you to map strings to strings, Redis is a **data structures server**.

| Data Type | Description | Common Use Case |
| :--- | :--- | :--- |
| **Strings** | Binary-safe text, serialized JSON, or integers (up to 512MB). | Session tokens, HTML caching, rate-limiting counters. |
| **Lists** | Lists of strings sorted by insertion order (linked lists). | Simple queues, recent activity feeds. |
| **Sets** | Unordered collections of unique strings. | Tracking unique visitors, tagging systems, intersection/union operations. |
| **Sorted Sets (ZSets)** | Unique strings mapped to a floating-point score, ordered automatically. | Gaming leaderboards, priority queues, secondary indexes. |
| **Hashes** | Maps of field-value pairs (essentially objects). | Storing user profiles or complex entity states. |
| **Bitmaps / HyperLogLogs** | Space-efficient structures for bit manipulation or cardinality estimation. | Tracking daily active users (DAU), estimating millions of unique views. |
| **Streams** | Append-only log structures built for real-time messaging. | Event-driven microservices, activity logging. |

## 3. Primary Use Cases

### A. Caching
The most common use case. By storing frequently accessed database queries, API responses, or user sessions in Redis, applications drastically reduce latency and protect backend databases from heavy read loads. It supports features like **Time-To-Live (TTL)** to automatically expire stale data.

### B. Session Management
In distributed web architectures, storing user sessions in a central Redis instance ensures that any server behind a load balancer can instantly access the user's state without needing sticky sessions.

### C. Rate Limiting
Because commands like `INCR` (increment) and `EXPIRE` are lightning-fast and atomic, Redis is the industry standard for enforcing rate limits on APIs to protect infrastructure from abuse.

### D. Pub/Sub and Message Queues
Redis includes built-in Publish/Subscribe capabilities, allowing services to broadcast messages to channels. Combined with Lists or Streams, it can handle light-to-medium messaging queue patterns.

---

## 4. Performance vs. Persistence

While Redis runs in memory, it is not completely volatile. It offers two main persistence mechanisms to save data to disk in case of a server crash or restart:

1.  **RDB (Redis Database Backup):** Point-in-time snapshots of the dataset taken at specified intervals (e.g., every 5 minutes). It is highly performant for restarts but risks losing data since the last snapshot.
2.  **AOF (Append Only File):** Logs every write operation received by the server. This file is replayed upon startup to reconstruct the original state. It minimizes data loss but results in a larger file and slightly slower write performance.

Many production environments use a combination of both RDB and AOF to balance speed and durability.

---

## 5. Scaling and High Availability

*   **Redis Sentinel:** A monitoring and failover management system. If a primary Redis node goes down, Sentinel automatically detects the failure and promotes a replica node to primary.
*   **Redis Cluster:** A distributed implementation that automatically shards data across multiple Redis nodes using a concept called hash slots. This allows horizontal scaling of memory and throughput across dozens or hundreds of servers.