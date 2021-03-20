// main.rs

use common::SOCKET_PATH;
use std::str;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::Read;
use std::thread;

mod common;

fn stream_handler(mut stream: TcpStream) {
  let mut data = [0 as u8; 4]; // using 50 byte buffer

  let mut response = String::new();

  while match stream.read(&mut data) {
    Ok(size) => {
      if size != 0 {
        println!("message size: {}", size);
        let str_val = str::from_utf8(&data[0..size]).unwrap();
        
        response.push_str(&str_val);

        true
      } else {
        println!("Connection closed successfully");

        println!("Phrase: {}", response);
        false
      }
    }
    Err(e) => {
      println!("Error Reading stream: {}", e);
      stream.shutdown(Shutdown::Both).unwrap();
      false
    }
  } {}
}

fn main() {
  let address = SOCKET_PATH;

  let listener = match TcpListener::bind(&address) {
    Err(_) => panic!("failed to bind socket"),
    Ok(listener) => listener,
  };

  println!("Server started on {}, waiting for clients", &address);

  // Iterate over clients, blocks if no client available
  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        println!("New Connection: {}", stream.peer_addr().unwrap());

        thread::spawn(move || {
          // connection succeeded
          stream_handler(stream);
        });
        
      }
      Err(e) => {
        println!("Error: {}", e);
      }
    }
  }
}