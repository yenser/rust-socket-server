// main.rs
mod common;

use common::{SOCKET_PATH, SOCKET_WINDOW_SIZE};
use std::convert::TryInto;
use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::{fs, thread};

// pub Stream

fn get_u32_from_buf(val: &[u8]) -> [u8; 4] {
  val.try_into().expect("slice with incorrect length")
}

fn save_buffer(buf: &Vec<u8>) {
  fs::create_dir_all("./videos/").unwrap();
  let file_name = format!("./videos/buffer.mp4");
  fs::write(file_name, &buf).expect("Unable to write file");
}

fn stream_handler(mut stream: TcpStream) {
  let mut data = [0 as u8; SOCKET_WINDOW_SIZE]; // using 50 byte buffer

  stream.read(&mut data).unwrap();
  let image_size = u32::from_be_bytes(get_u32_from_buf(&data[0..4]));
  println!("image_size: {}", &image_size);

  let mut buf: Vec<u8> = Vec::new();

  while match stream.read(&mut data) {
    Ok(size) => {
      if size != 0 {

        let total_bytes = buf.len() + size; // size of buffer plus data on stream

        if total_bytes as u32 >= image_size {
          let difference: usize = total_bytes - image_size as usize;
          let index = size - difference; // get index of where image ends

          if index != 0 {
            buf.extend_from_slice(&data[0..index]); // add remaining data to buffer
          }

          save_buffer(&buf);

          buf.clear();

          if difference != 0 { // add suffixed data for next image into new file
            buf.extend_from_slice(&data[index..size]);
          }
        } else {
          buf.extend_from_slice(&data[0..size]);
        }

        true
      } else {
        println!("Socket connection closed successfully");
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

pub fn start() {
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
