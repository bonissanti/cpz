use std::sync::atomic::{AtomicUsize, Ordering};
use crossbeam::channel::{self, Sender, Receiver};

//TODO: add documentation of box, dyn, fnOnce, Send, static and algorithm of thread pool
pub struct ThreadPool {
    sender: Option<channel::Sender<Box<dyn FnOnce() + Send + 'static>>>,
    _handles: Option<Vec<std::thread::JoinHandle<()>>>,
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
            sender: Option::from(sender),
            _handles: Option::from(handles),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        if let Some(handles) = self._handles.take() {
            for handle in handles {
                let _ = handle.join();
            }
        }
    }
}