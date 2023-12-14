use std::{env, io::{self, Error, Write}, net::TcpListener};
use tungstenite::{accept, connect, Message};
use url::Url;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} [server|client] [address:port]", args[0]);
        return;
    }

    let result = match args[1].as_str() {
        "server" => start_server(&args[2]),
        "client" => start_client(&args[2]),
        _ => {
            eprintln!("Invalid mode. Choose 'server' or 'client'.");
            return;
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}

fn start_server(addr: &str) -> Result<(), Error> {
    let server = TcpListener::bind(addr)?;
    println!("WebSocket server listening on: {}", addr);

    for stream in server.incoming() {
        let stream = stream?;
        let mut websocket = match accept(stream) {
            Ok(ws) => ws,
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())),
        };

        loop {
            let msg = match websocket.read() {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("Error reading message: {}", e);
                    break;
                }
            };

            if msg.is_text() {
                let msg_str = msg.to_text().unwrap();
                let response = msg_str.chars().rev().collect::<String>();
                println!("Sending: {}", response);
                match websocket.write(Message::Text(response)) {
                    Ok(_) => {
                        match websocket.flush() {
                            Ok(_) => (),
                            Err(e) => {
                                eprintln!("Error flushing message: {}", e);
                                break;
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Error sending message: {}", e);
                        break;
                    }
                }
            } else if msg.is_close() {
                break;
            }
        }
    }

    Ok(())
}

fn start_client(url: &str) -> Result<(), Error> {
    let parsed_url = match Url::parse(url) {
        Ok(url) => url,
        Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())),
    };

    let (mut socket, _) = match connect(parsed_url) {
        Ok((socket, response)) => (socket, response),
        Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())),
    };

    println!("Connected to the server");

    loop {
        let mut input = String::new();
        println!("Enter message: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        if input == "exit" {
            break;
        }

        match socket.write(Message::Text(input)) {
            Ok(_) => {
                match socket.flush() {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error flushing message: {}", e),
                }
            },
            Err(e) => {
                eprintln!("Error sending message: {}", e);
                continue;
            }
        }

        match socket.read() {
            Ok(msg) => println!("Received: {}", msg),
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                continue;
            }
        }
    }

    Ok(())
}
