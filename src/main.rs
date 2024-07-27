use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    string,
};

use bytes::{BufMut, Bytes, BytesMut};

use redis_server::resp::data::RESPDataType;
use redis_server::resp::deserializer::RespDeserializer;
use redis_server::resp::serializer::RespSerializer;
use redis_server::thread_pool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => pool.execute(|| {
                handle_connection(stream);
            }),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut buffer = [0; 512];
    match buf_reader.read(&mut buffer) {
        Ok(size) => {
            let command: String = String::from_utf8(buffer[..size].to_vec()).unwrap();
            let mut bytes_mut = BytesMut::with_capacity(command.len());
            bytes_mut.put(command.as_bytes());

            let resp_serializer = RespSerializer::default();
            let resp_deserializer = RespDeserializer::default();
            let resp_result = resp_deserializer.deserialize(&bytes_mut, 0);

            if let Ok(value) = resp_result {
                if let Some((_, resp_data_type)) = value {
                    match resp_data_type {
                        RESPDataType::Array(resp_data_types) => {
                            let first = resp_data_types.get(0);

                            match first.unwrap() {
                                RESPDataType::BulkString(string_command) => {
                                    if string_command == "ping" {
                                        let response = resp_serializer.serialize_ss("pong");
                                        stream.write_all(response.as_bytes()).unwrap();
                                    } else if string_command == "echo" {
                                        let msg = resp_data_types.get(1);
                                        match msg.unwrap() {
                                            RESPDataType::BulkString(return_msg) => {
                                                let response = resp_serializer.serialize_ss(
                                                    String::from_utf8(return_msg.to_vec())
                                                        .unwrap()
                                                        .as_str(),
                                                );
                                                stream.write_all(response.as_bytes()).unwrap();
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                _ => println!("To handle"),
                            }
                        }
                        _ => println!("To handle"),
                    }
                }
            }
        }
        Err(e) => {
            println!("error: {}", e);
        }
    };
}
