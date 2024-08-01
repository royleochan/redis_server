pub mod resp;
pub mod store;
pub mod thread_pool;

use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use bytes::{BufMut, BytesMut};
use log::{error, info};

use resp::data::RESPDataType;
use resp::deserializer::RespDeserializer;
use resp::serializer::RespSerializer;
use store::Store;

pub fn handle_connection(mut stream: TcpStream, store: &mut Store) {
    info!("Handling new connection.");

    let command: BytesMut = get_command(&stream);

    let resp_deserializer = RespDeserializer::default();
    let resp_result = resp_deserializer.deserialize(&command, 0);

    if let Ok(value) = resp_result {
        if let Some((_, resp_data_type)) = value {
            let response = handle_resp_command(resp_data_type, store);
            if response.len() > 0 {
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
    } else {
        error!("Error occured while deserializing command.");
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

fn handle_resp_command(resp_command: RESPDataType, store: &mut Store) -> String {
    if let RESPDataType::Array(resp_data_types) = resp_command {
        info!("Handling command {:?}", resp_data_types);
        let first = resp_data_types.get(0);
        match first.unwrap() {
            RESPDataType::BulkString(first_command) => {
                match String::from_utf8(first_command.to_vec()).unwrap().as_str() {
                    "config" | "CONFIG" => handle_config(),
                    "ping" | "PING" => handle_ping(),
                    "echo" | "ECHO" => handle_echo(resp_data_types),
                    "set" | "SET" => handle_set(resp_data_types, store),
                    "get" | "GET" => handle_get(resp_data_types, store),
                    _ => return handle_default(),
                }
            }
            _ => handle_error("First element in command should be a bulk string."),
        }
    } else {
        handle_error("Command should be an array.")
    }
}

fn handle_error(error_str: &str) -> String {
    let resp_serializer: RespSerializer = RespSerializer::default();
    return resp_serializer.serialize_error(error_str);
}

fn handle_default() -> String {
    return handle_error("Unimplemented command.");
}

fn handle_config() -> String {
    return String::from("*2\r\n$4\r\nsave\r\n$0\r\n\r\n");
}

fn handle_ping() -> String {
    let resp_serializer: RespSerializer = RespSerializer::default();
    return resp_serializer.serialize_ss("pong");
}

fn handle_echo(resp_data_types: Vec<RESPDataType>) -> String {
    let resp_serializer: RespSerializer = RespSerializer::default();
    if let Some(msg) = resp_data_types.get(1) {
        if let RESPDataType::BulkString(return_msg) = msg {
            return resp_serializer
                .serialize_ss(String::from_utf8(return_msg.to_vec()).unwrap().as_str());
        } else {
            return handle_error("Echo should be followed by a string.");
        }
    }
    return handle_error("Missing 'message' argument.");
}

fn handle_set(resp_data_types: Vec<RESPDataType>, store: &mut Store) -> String {
    let resp_serializer: RespSerializer = RespSerializer::default();
    if resp_data_types.get(1).is_none() {
        return handle_error("Missing 'key' argument.");
    }
    if resp_data_types.get(2).is_none() {
        return handle_error("Missing 'value' argument.");
    }
    let key_resp = resp_data_types.get(1).unwrap();
    let val_resp = resp_data_types.get(2).unwrap();
    match (key_resp, val_resp) {
        (RESPDataType::BulkString(key), RESPDataType::BulkString(val)) => {
            store.set_key_val(key.clone(), val.clone());
            return resp_serializer.serialize_ss("OK");
        }
        _ => return handle_error("Set should be followed by 2 bulk strings."),
    }
}

fn handle_get(resp_data_types: Vec<RESPDataType>, store: &mut Store) -> String {
    let resp_serializer: RespSerializer = RespSerializer::default();
    if let Some(key_resp) = resp_data_types.get(1) {
        if let RESPDataType::BulkString(key) = key_resp {
            let value = store.get_from_key_val_store(key.clone());
            if let Some(result) = value {
                return resp_serializer
                    .serialize_ss(String::from_utf8(result.to_vec()).unwrap().as_str());
            }
            return resp_serializer.serialize_nil();
        } else {
            return handle_error("Echo should be followed by a string.");
        }
    }
    return handle_error("Missing 'key' argument.");
}
