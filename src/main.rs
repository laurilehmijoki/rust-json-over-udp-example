use std::net::UdpSocket;
extern crate rustc_serialize;
use rustc_serialize::json;
use std::str;
use std::string::String;

fn main() {
    match main_loop() {
        Err(e) => panic!(e),
        _ => ()
    }
}

fn main_loop() -> Result<(), ErrorMessage> {
    let remote_address = "127.0.0.1:34254";
    let socket = try!(UdpSocket::bind(remote_address).map_err(|e| {
        format!("Could not bind to {}: {}", remote_address, e)
    }));

    loop {
        let mut buf = [0; 1000];
        let (amount_read, sender_address) = try!(socket.recv_from(&mut buf).map_err(|e| {
            format!("Could not receive data {}", e)
        }));

        let response_str = match response(&buf[..amount_read]) {
            Ok(response_str) => response_str,
            Err(e) => {
                println!("Error in building response {}", e);
                continue
            }
        };
        match socket.send_to(response_str.as_bytes(), &sender_address) {
            Ok(_) => {
                println!("Sent message {}", response_str);
                continue
            },
            Err(e) => {
                println!("Error while sending data to socket: {}", e);
                break
            }
        }
    }
    Ok(())
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Message {
    recipient: String,
    content: String
}

fn response(message_buf: &[u8]) -> Result<String, ErrorMessage> {
    println!("got message {}", String::from_utf8_lossy(message_buf));
    let message = try!(decode_message(message_buf));

    let response_message = Message {
        recipient: "you".to_string(),
        content: format!("greetings from {}", message.recipient)
    };

    json::encode(&response_message).map_err(|e| {
        format!("Could not encode response: {}", e)
    })
}

fn decode_message(message_buf: &[u8]) -> Result<Message, ErrorMessage> {
    let message_str = try!(str::from_utf8(message_buf).map_err(|e| {
        format!("Message is not UTF8: {}", e)
    }));

    json::decode(&message_str).map_err(|e| {
       format!("Error in decoding json: {}", e)
    })
}

type ErrorMessage = String;
