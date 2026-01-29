# Exercise 11 - Designing E-commerce product listing

For a small shop of 100 items, design a system in which the shop owner can

- add a new product
- update/delete existing product
- list all products on the website
- customers should be able to access catalog quickly

Task

- Design DB schema
- Write backend API
- Setup DB replication
- Read API from replica

## Design

### Architecture 

Classic 3 tier architecture: Client -> Server -> DB

```mermaid
graph TD
    subgraph Client_Layer [Client]
        C1[Customer Browser]
        C2[Admin Dashboard]
    end

    LB[Load Balancer]

    subgraph Service_Layer [Service]
        API[Backend REST API]
    end

    subgraph Cache_Layer [Cache]
        Redis[(Redis/In-Memory Cache)]
    end

    subgraph Data_Layer [Data]
        DB_P[(Primary DB - Read/Write)]
        DB_R[(Read Replica)]
    end

    %% Flow
    C1 & C2 --> LB
    LB --> API
    API <--> Redis
    
    %% Admin Writes
    API -- "Write/Update/Delete" --> DB_P
    
    %% Customer Reads
    Redis -- "Cache Miss" --> DB_R
    API -- "Read Request" --> Redis
    
    %% Replication
    DB_P -- "Asynchronous Replication" --> DB_R
```

#### Storage

- Small, only 100 rows
- Structured data -> RDB
- Read replicas to handle read

#### Services

- One frontend for client, another for admin and a backend
- REST HTTP
- Scalable
- Frontend by load balancer

#### Cache

- Cache layer before DB 

