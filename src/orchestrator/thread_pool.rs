use std::sync::atomic::{AtomicUsize, Ordering};
use crossbeam::channel::{self, Sender, Receiver};

//TODO: add documentation of box, dyn, fnOnce, Send and static
pub struct ThreadPool {
    sender: channel::Sender<Box<dyn FnOnce() + Send + 'static>>,
    _handles: Vec<std::thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = channel::unbounded();
        let mut handles = Vec::with_capacity(size);

        for id in 0..size {
            let rx: Receiver<Box<dyn FnOnce() + Send + 'static>> = receiver.clone();
            handles.push(std::thread::spawn(move || {
                while let Ok(task) = rx.recv() {
                    println!("Worker {} is running", id);
                    task()
                }
            }))
        }

        ThreadPool {
            sender,
            _handles: handles,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}
