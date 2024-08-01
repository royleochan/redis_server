use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use log::{error, info};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(num_threads: usize) -> Self {
        assert!(num_threads > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(num_threads);

        for id in 0..num_threads {
            workers.push(Worker::new(id, Arc::clone(&receiver)).unwrap())
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, String> {
        let builder = thread::Builder::new();
        let result = builder.spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            info!("Worker {id} got a job; executing.");

            job();
        });
        match result {
            Ok(thread) => Ok(Worker { id, thread }),
            Err(e) => {
                error!("Thread failed: {:?}", e);
                Err("Error spawning thread".to_string())
            }
        }
    }
}
