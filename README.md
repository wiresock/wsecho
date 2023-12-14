# wsecho: Rust WebSocket Echo Application

## Introduction

`wsecho` is a Rust application demonstrating a simple WebSocket communication setup. It features a server that reverses incoming messages and a client that sends and receives these altered messages.

## Key Features

- **Reverse Echo Server**: The server reverses any text message received from the client.
- **Interactive WebSocket Client**: Facilitates sending messages and displays the reversed responses.
- **Single-Client Support**: Designed to handle one client at a time, focusing on simplicity and clarity.

## Usage Instructions

### Server

```bash
cargo run server [address:port]
```

- Initiates the server listening on the specified address and port.

### Client

```bash
cargo run client ws://[address:port]
```

- Connects to the server and allows sending messages to receive reversed echoes.

## Example Interaction

- Server: `cargo run server 127.0.0.1:9001`
- Client: `cargo run client ws://127.0.0.1:9001`
- Client Input: `Hello, server!`
- Server Response: `!revres ,olleH`

## Note

This application is for testing and educational purposes and may require enhancements for production use.