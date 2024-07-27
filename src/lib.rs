pub mod resp;
pub mod thread_pool;

use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use bytes::{BufMut, BytesMut};

use resp::data::RESPDataType;
use resp::deserializer::RespDeserializer;
use resp::serializer::RespSerializer;

pub fn handle_connection(mut stream: TcpStream) {
    let command: BytesMut = get_command(&stream);

    let resp_deserializer = RespDeserializer::default();
    let resp_result = resp_deserializer.deserialize(&command, 0);

    if let Ok(value) = resp_result {
        if let Some((_, resp_data_type)) = value {
            let response = handle_resp_command(resp_data_type);
            if response.len() > 0 {
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
    }
}

fn get_command(mut stream: &TcpStream) -> BytesMut {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut buffer = [0; 512];
    if let Ok(command_size) = buf_reader.read(&mut buffer) {
        let command: String = String::from_utf8(buffer[..command_size].to_vec()).unwrap();
        let mut bytes_mut = BytesMut::with_capacity(command.len());
        bytes_mut.put(command.as_bytes());
        return bytes_mut;
    } else {
        panic!("Command could not be deserialized.");
    }
}

fn handle_resp_command(resp_command: RESPDataType) -> String {
    if let RESPDataType::Array(resp_data_types) = resp_command {
        let first = resp_data_types.get(0);
        match first.unwrap() {
            RESPDataType::BulkString(first_command) => {
                match String::from_utf8(first_command.to_vec()).unwrap().as_str() {
                    "ping" => handle_ping(),
                    "echo" => handle_echo(resp_data_types),
                    _ => handle_null(),
                }
            }
            _ => {
                panic!("First element in command should be a bulk string.")
            }
        }
    } else {
        panic!("Command should be an array.")
    }
}

fn handle_null() -> String {
    println!("Unimplemented command");
    return String::from("");
}

fn handle_ping() -> String {
    let resp_serializer: RespSerializer = RespSerializer::default();
    return resp_serializer.serialize_ss("pong");
}

fn handle_echo(resp_data_types: Vec<RESPDataType>) -> String {
    let resp_serializer: RespSerializer = RespSerializer::default();
    let msg = resp_data_types.get(1).unwrap();
    if let RESPDataType::BulkString(return_msg) = msg {
        return resp_serializer
            .serialize_ss(String::from_utf8(return_msg.to_vec()).unwrap().as_str());
    } else {
        panic!("Echo should be followed by a string.")
    }
}
