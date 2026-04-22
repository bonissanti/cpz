use std::sync::mpsc::{self, Receiver};
use std::thread;

pub(crate) type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub(crate) fn new(id: usize, reciver: Receiver<Task>) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(task) = reciver.recv() {
                println!("Worker {} is running", id);
                task()
            }
        });
        Worker { id, thread }
    }
}
