# Exercise 7 - Webscoket

Simple exercise running python clients and server locally to communicate via WebSocket, using [SocketIO](https://github.com/miguelgrinberg/python-socketio).

## Concept

1. **WebSocket Basics**
   - WebSockets provide full-duplex, persistent communication between client and server.
   - Unlike HTTP’s request/response model, WebSockets allow either side to send messages independently at any time.

2. **Socket.IO Abstraction**
   - Socket.IO is a library that simplifies real-time communication using WebSockets.
   - It provides event-driven messaging, automatic connection management, and fallback transports if WebSockets are not available.

3. **Event-Driven Communication**
   - Communication is structured around events, e.g., `connect`, `disconnect`, or custom events.
   - Clients can emit events to the server, and the server can emit events to one or multiple clients.
   - This model decouples messaging from raw network protocols, making code more readable and maintainable.

4. **Persistent Connections**
   - WebSocket connections remain open until explicitly closed.
   - Both server and client can continuously send and receive data over the same connection.

5. **Client Identification**
   - Socket.IO provides unique session IDs (SID) for each client.
   - This allows the server to track connected clients, manage broadcasts, and handle disconnects.

6. **Server-Side Asynchronous Handling**
   - Servers can run async tasks alongside handling events.
   - Background or periodic tasks can push data to clients without blocking other connections.

7. **Scalability**
   - Using an async framework (like Python `asyncio`) or an ASGI server allows a Socket.IO server to handle many concurrent connections efficiently.

## Setup

This exercise will use `uv` as the python venv.

```sh
uv venv websocket -p 3.14
source websocket/bin/activate
uv pip install python-socketio uvicorn requests websocket-client
```
Copy the snippet below as `client.py` and `server.py`.

### server.py

```python
"""
WebSocket server using Python Socket.IO and Uvicorn.

- Tracks connected clients via SID → client_id mapping
- Emits server time every 5 seconds
- Broadcasts client join/leave messages
"""

import asyncio, socketio, uvicorn, datetime

sio = socketio.AsyncServer(async_mode="asgi")
app = socketio.ASGIApp(sio)

# Mapping of SID -> client_id for client tracking
clients = {} 

@sio.event
async def connect(sid, environ, auth):    
    """
    Triggered automatically when a client establishes a WebSocket connection.

    Args:
        sid: unique session ID for this connection
        environ: ASGI environment info
        auth: optional client-provided metadata
    """
    client_id = clients[sid] = auth.get("client_id", "unknown")
    print(f"connected: {client_id}")

    # Start server time broadcast on first connection
    if not hasattr(sio, "_ticker_started"):
        sio._server_time_started = True
        sio.start_background_task(server_time)

    # Broadcast join message to all clients
    await sio.emit("broadcast", f"{client_id} joined the channel")

@sio.event
async def disconnect(sid):
    """
    Triggered automatically when a client disconnects.
    """
    client_id = clients.pop(sid, "unknown")
    print(f"disconnected: {client_id}")
    await sio.emit("broadcast", f"{client_id} left the channel")

async def server_time():
    """
    Periodically broadcasts the current server time every 5 seconds.
    """
    while True:
        await asyncio.sleep(5)
        timenow = datetime.datetime.now().strftime("%H:%M:%S")
        await sio.emit("broadcast", f"Server Time: {timenow}")

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
```

### client.py

```python
"""
WebSocket client using Python Socket.IO.

- Connects to the server with a unique client_id
- Listens for 'broadcast' messages from the server
"""

import socketio, uuid

sio = socketio.Client()
client_id = f"client-{str(uuid.uuid4())[:4]}"

@sio.event
def connect():
    """
    Triggered automatically when the client connects to the server.
    Can be used to emit initial messages or metadata.
    """
    msg = f"client {client_id} joined"

@sio.event
def broadcast(msg): 
    """
    Handler for 'broadcast' events sent by the server.
    """
    print(f"Received Broadcast: {msg}")

print(f"Starting up client, id = {client_id}")
sio.connect("http://localhost:8000", auth={"client_id": client_id})
sio.wait()

```

### Test

1. Start the server by running server.py.
2. Run a client (client.py). Observe that it receives the broadcast message announcing its own join.
3. Start a second client. Notice that:
  - The first client receives a broadcast notifying that a new client has joined. 
  - The second client receives its own join message.
4. Disconnect the first client. Observe that:
  - The second client receives a broadcast indicating the first client has left.
  - The second client continue receiving periodic server time updates every 5 seconds.

> This demonstrates real-time, bidirectional communication, client join/leave notifications, and server-initiated events using WebSockets and Socket.IO.

#### Sample Output

```sh
# Client 1
Starting up client, id = client-fca9
Received Broadcast: client-fca9 joined the channel
Received Broadcast: Server Time: 21:49:25
Received Broadcast: Server Time: 21:49:30
Received Broadcast: client-10d4 joined the channel
Received Broadcast: Server Time: 21:49:35
# exit client 2
```

```sh
# Client 2
Received Broadcast: client-10d4 joined the channel
Received Broadcast: Server Time: 21:49:40
Received Broadcast: Server Time: 21:49:45
Received Broadcast: client-fca9 left the channel
Received Broadcast: Server Time: 21:49:50
```

```sh
# Server
connected: client-10d4
disconnected: client-fca9
disconnected: client-10d4
```