// -----------------------------------------------------------------------------
// Simple HTTP Server                                           /httpr/[main.rs]
// (c) balarabe@protonmail.com                                      License: MIT
// -----------------------------------------------------------------------------

// Compile with warnings:      rustc main.rs
// Compile ignoring warnings:  rustc -D warnings main.rs

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

// main entry point of the application
fn main() {
    println!("Server listening on port 900");
    let listener = TcpListener::bind("0.0.0.0:900").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accept connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || { connect(stream) });
            }
            Err(er) => {
                println!("Connection failed with error: {}", er);
            }
        }
    }
    drop(listener)
} //                                                                        main

// spawned in a thread to serve incoming connections
fn connect(mut stream: TcpStream) {
    println!("\n>>>>>>> thread for {}", stream.peer_addr().unwrap());
    let mut data = [0 as u8; 64 * 1024]; // using 64KB byte buffer
    /*while*/ match stream.read(&mut data) {
        Ok(size) => {
            let request = String::from_utf8_lossy(&data[0..size]);
            println!("\n>>>>>>> {} {} bytes:\n{}",
                stream.peer_addr().unwrap(), size, request);
            let header = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();
            let response = DOC_NOT_FOUND.as_bytes();
            stream.write(header).unwrap();
            stream.write(response).unwrap();
            stream.flush().unwrap();
            /*false*/ // result
        },
        Err(_) => {
            println!("\n>>>>>>> {} error reading",
                stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            /*false*/ // result
        }
    } /*{}*/
} //                                                                     connect

// 404 message displayed by the server when a file is not found
// (includes the HTTP header for now)
const DOC_NOT_FOUND: &str =
"<!DOCTYPE html> \
<html lang='en'> \
    <head> \
        <meta charset='utf-8'> \
        <title>E404</title> \
        <style> \
        div.main { \
            align-items: center; \
            display: flex; \
            height: 90vh; \
            justify-content: center; \
            width: 100%; \
        } \
        div.box { \
            background-color: #FF3300; /*orange*/ \
            border-radius: 0.5rem; \
            height: 5rem; \
            padding-top: 1rem; \
            width: 24rem; \
        } \
        .big { \
            font-size: xx-large; \
        } \
        p { \
            color: white; \
            font-family: sans-serif; \
            font-weight: bolder; \
            margin: 0; \
            text-align: center; \
        } \
        </style> \
    </head> \
    <body> \
        <div class='main'> \
            <div class='box'> \
                <p class='big'>404: Page not found! *</p> \
                <p>* However the server is responding.</p> \
            </div> \
        </div> \
    </body> \
</html>";

// end
