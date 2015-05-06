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

        let buf = &mut buf[..amount_read];
        let response_str = response(buf);
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

fn decode_message(message_buf: &[u8]) -> Message {
    let message_str = str::from_utf8(message_buf).unwrap();
    let message: Message = json::decode(&message_str).unwrap();
    message
}

fn response(message_buf: &[u8]) -> String {
    let message = decode_message(message_buf);

    println!("got message {}", json::encode(&message).unwrap());

    let response_message = Message {
        recipient: "you".to_string(),
        content: format!("greetings from {}", message.recipient)
    };

    json::encode(&response_message).unwrap()
}

