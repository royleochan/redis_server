use std::net::TcpListener;

use std::sync::{Arc, Mutex};

use redis_server::handle_connection;
use redis_server::store::Store;
use redis_server::thread_pool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let pool = ThreadPool::new(4);
    let state_store = Arc::new(Mutex::new(Store::init()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let state_store = Arc::clone(&state_store);
                pool.execute(move || {
                    let mut store = state_store.lock().unwrap();
                    handle_connection(stream, &mut store);
                })
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
