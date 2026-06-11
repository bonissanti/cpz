use crate::utils::enums::StorageKind;
use crossbeam::channel::{self, Receiver};
use std::sync::atomic::{AtomicUsize, Ordering};

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
                    task()
                }
            }))
        }

        ThreadPool {
            sender: Option::from(sender),
            _handles: Option::from(handles),
        }
    }

    pub fn worker_count(&self) -> usize {
        self._handles.as_ref().map_or(0, |h| h.len())
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }

    pub fn get_threadpool_by_storage_kind(storage_kind: StorageKind) -> ThreadPool
    {
        if storage_kind == StorageKind::HDD {
            return ThreadPool::new(2)
        }
        else if storage_kind == StorageKind::SSD {
            return ThreadPool::new(4)
        }
        else if storage_kind == StorageKind::NVME {
            let cpus_available = std::thread::available_parallelism()
                .map(|x| x.get())
                .unwrap_or(4);

            return ThreadPool::new(cpus_available)
        }
        return ThreadPool::new(2)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn create_thread_pool_with_specific_size(size: usize) -> ThreadPool {
        ThreadPool::new(size)
    }

    #[test]
    fn test_thread_pool_initialization_with_specific_size() {
        let size = 4;
        let thread_pool = create_thread_pool_with_specific_size(size);
        assert_eq!(thread_pool.worker_count(), size);
    }

    #[test]
    fn test_thread_pool_executes_single_task() {
        let task_execution_count = Arc::new(AtomicUsize::new(0));
        let task_execution_count_clone = Arc::clone(&task_execution_count);

        {
            let thread_pool = create_thread_pool_with_specific_size(2);
            thread_pool.execute(move || {
                task_execution_count_clone.fetch_add(1, Ordering::SeqCst);
            });
        }

        assert_eq!(task_execution_count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_thread_pool_executes_multiple_tasks_concurrently() {
        use std::sync::Barrier;

        let pool_size = 4;
        let completed_task_count = Arc::new(AtomicUsize::new(0));
        let active_task_count = Arc::new(AtomicUsize::new(0));
        let peak_active_task_count = Arc::new(AtomicUsize::new(0));
        let all_workers_active_barrier = Arc::new(Barrier::new(pool_size));

        {
            let thread_pool = create_thread_pool_with_specific_size(pool_size);
            for _ in 0..pool_size {
                let completed_task_count = Arc::clone(&completed_task_count);
                let active_task_count = Arc::clone(&active_task_count);
                let peak_active_task_count = Arc::clone(&peak_active_task_count);
                let all_workers_active_barrier = Arc::clone(&all_workers_active_barrier);
                thread_pool.execute(move || {
                    let current_active = active_task_count.fetch_add(1, Ordering::SeqCst) + 1;
                    peak_active_task_count.fetch_max(current_active, Ordering::SeqCst);
                    all_workers_active_barrier.wait();
                    active_task_count.fetch_sub(1, Ordering::SeqCst);
                    completed_task_count.fetch_add(1, Ordering::SeqCst);
                });
            }
        }

        assert_eq!(completed_task_count.load(Ordering::SeqCst), pool_size);
        assert!(peak_active_task_count.load(Ordering::SeqCst) >= pool_size);
    }

    #[test]
    fn test_thread_pool_executes_more_tasks_than_workers() {
        let pool_size = 4;
        let total_tasks = 20;
        let completed_task_count = Arc::new(AtomicUsize::new(0));

        {
            let thread_pool = create_thread_pool_with_specific_size(pool_size);
            for _ in 0..total_tasks {
                let completed_task_count = Arc::clone(&completed_task_count);
                thread_pool.execute(move || {
                    completed_task_count.fetch_add(1, Ordering::SeqCst);
                });
            }
        }

        assert_eq!(completed_task_count.load(Ordering::SeqCst), total_tasks);
    }
}