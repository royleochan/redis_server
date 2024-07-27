use std::net::TcpListener;

use redis_server::handle_connection;
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
