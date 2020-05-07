// -----------------------------------------------------------------------------
// Simple HTTP Server                                           /httpr/[main.rs]
// (c) balarabe@protonmail.com                                      License: MIT
// -----------------------------------------------------------------------------

// Compile with warnings:      rustc main.rs
// Compile ignoring warnings:  rustc -D warnings main.rs

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

// _ _
fn main() {
    println!("Server listening on port 900");
    let listener = TcpListener::bind("0.0.0.0:900").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accept connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || { connect(stream) });
            }
            Err(e) => {
                println!("Connection failed with error: {}", e);
            }
        }
    }
    drop(listener)
} //                                                                        main

// _ _
fn connect(mut stream: TcpStream) {
    println!("\n>>>>>>> thread for {}", stream.peer_addr().unwrap());
    let mut data = [0 as u8; 64 * 1024]; // using 64KB byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            let request = String::from_utf8_lossy(&data[0..size]);
            println!("\n>>>>>>> {} {} bytes:\n{}",
                stream.peer_addr().unwrap(), size, request);
            let ar = DOC_NOT_FOUND.as_bytes();
            stream.write(ar).unwrap();
            size > 0 // result
        },
        Err(_) => {
            println!("\n>>>>>>> {} error reading",
                stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false // result
        }
    } {}
} //                                                                     connect

// _ _
const DOC_NOT_FOUND: &str =
"<!DOCTYPE html PUBLIC '-//w3c//dtd xhtml 1.0 transitional//en' \
'http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd'> \
<html xmlns='http://www.w3.org/1999/xhtml'> \
<head><title>E404</title></head> \
<body bgcolor='#ffffff'> \
<div style='position: absolute; left: 55px; width: 350px; \
 top: 55px; height: 85px; background-color: #FF3300; \
 font-family: Arial, Helvetica, sans-serif; \
 font-size: xx-large; font-weight: bolder; \
 color: #FFFFFF;' align='center'> \
404: That page was not found! *</div> \
<div style='position: absolute; left: 55px; width: 350px; \
 top: 150px; font-family: Arial, Helvetica, sans-serif; \
 font-weight: bolder; color: #FF3300;' align='center'> \
* However the server is responding.</div> \
</body></html>";

// end
