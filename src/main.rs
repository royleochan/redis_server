use std::thread;
use std::time::Duration;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use redis_server::ThreadPool;

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
    let _buf_reader = BufReader::new(&mut stream);
    let response = "+PONG\r\n";
    thread::sleep(Duration::new(2, 0));
    stream.write_all(response.as_bytes()).unwrap();
}
