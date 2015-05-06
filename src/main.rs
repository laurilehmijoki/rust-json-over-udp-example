use std::net::UdpSocket;
extern crate rustc_serialize;
use rustc_serialize::json;
use std::str;
use std::string::String;

fn main() {
    let remote_address = "127.0.0.1:34254";
    let socket = match UdpSocket::bind(remote_address) {
        Ok(socket) => socket,
        Err(e) => panic!("Could not bind to {}: {}", remote_address, e)
    };

    loop {
        let mut buf = [0; 1000];
        let (amount_read, sender_address) = match socket.recv_from(&mut buf) {
            Ok(x) => x,
            Err(e) => panic!("Could not receive data {}", e)
        };

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
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Message {
    recipient: String,
    content: String
}

fn response(message_buf: &[u8]) -> Result<String, ErrorMessage> {
    println!("got message {}", String::from_utf8_lossy(message_buf));
    let message = match decode_message(message_buf) {
        Ok(message) => message,
        Err(error_message) => return Err(error_message)
    };

    let response_message = Message {
        recipient: "you".to_string(),
        content: format!("greetings from {}", message.recipient)
    };

    match json::encode(&response_message) {
        Ok(json) => Ok(json),
        Err(e) => Err(format!("Could not encode response: {}", e))
    }
}

fn decode_message(message_buf: &[u8]) -> Result<Message, ErrorMessage> {
    let message_str = match str::from_utf8(message_buf) {
        Ok(utf8) => utf8,
        Err(utf8_error) =>
            return Err(format!("Message is not UTF8: {}", utf8_error))
    };

    let message: Message = match json::decode(&message_str) {
        Ok(decoded) => decoded,
        Err(decoder_error) =>
            return Err(format!("Error in decoding json: {}", decoder_error))
    };

    Ok(message)
}

type ErrorMessage = String;
