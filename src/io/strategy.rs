use std::fs::read_to_string;
use crate::orchestrator::thread_pool::ThreadPool;

const LIMIT_FILE_SIZE: u64 = 2000000000;
const SYSFS_PATH: &str = "/sys/block/sda/queue/rotational";

pub struct Strategy {
    pub pool:           ThreadPool,
    pub use_chunks:     bool,
    pub storage_kind:   StorageKind,
    pub jobs:           Vec<crate::orchestrator::job::Job>
}

pub enum StorageKind {
    SSD,
    HDD
}

impl Strategy {
    pub fn determine_strategy(jobs: Vec<crate::orchestrator::job::Job>) -> crate::orchestrator::thread_pool::ThreadPool
    {
        let storage_kind: StorageKind = Self::detect_storage_kind();
        let total_size: u64 = jobs.iter().map(|j| j.size).sum();

        if jobs.len() == 1 && total_size < LIMIT_FILE_SIZE {
            Self::single_thread(false);
        }
        else if jobs.len() == 1 && total_size > LIMIT_FILE_SIZE {
            Self::single_thread(true);
        }

        else if jobs.len() > 1 && total_size < LIMIT_FILE_SIZE {
            Self::pooled(3);
        }

        else if jobs.len() > 1 && total_size > LIMIT_FILE_SIZE {
            Self::pooled(6);
        }
        return crate::orchestrator::thread_pool::ThreadPool::new(0);
    }

    fn single_thread(with_chunk: bool)
    {

    }

    fn pooled(thread_pool_size: usize)
    {

    }

    fn detect_storage_kind() -> StorageKind
    {
        let storage_kind: String = read_to_string(SYSFS_PATH).unwrap();

        if storage_kind == "0" {
            StorageKind::SSD
        }
        else {
            StorageKind::HDD
        }
    }
}