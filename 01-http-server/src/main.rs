// An HTTP Server
// This contains
// - String interpolation/concat
// - Timestamp
// - Basic HTTP server

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let name = "Basic HTTP Server";
    let message = format!("{name} is starting");
    println!("{message}");

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!(
        "Listening for incoming request on http://127.0.0.1:{}",
        "8000"
    );

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Unable to connect. Error: {}", e);
            }
        }
    }
}

fn handle_client(stream: TcpStream) {
    let result = handle_read(&stream);
    handle_write(&stream, &result);
}

fn handle_read(mut stream: &TcpStream) -> String {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
            format!("{req_str}")
        }
        Err(e) => {
            println!("Unable to read stream: {}", e);
            String::from("Error")
        },
    }
}

fn handle_write(mut stream: &TcpStream, result: &String) {
    
    let time = current_timestamp();

    // string interpolation
    let response =     format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<h1>Hello world from Rust. </h1> 
    <br/><strong>Timestamp</strong>: {time}, <br> <pre>{result}</pre>\r\n");

    // convert string to bytes array
    match stream.write(response.as_bytes()) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn current_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
