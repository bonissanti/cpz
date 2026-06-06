use std::fs::read_to_string;
use crate::orchestrator::job::Job;
use crate::orchestrator::thread_pool::ThreadPool;
use crate::orchestrator::worker::Worker;
use crate::utils::enums::StorageKind;
use crate::utils::utils::Utils;

const LIMIT_FILE_SIZE: u64 = 2000000000;

pub enum ExecutePlan {
    SingleThread { use_chunks: bool },
    Pooled { pool: ThreadPool, use_chunks: bool }
}

pub struct Strategy {
    pub plan:           ExecutePlan,
    pub storage_kind:   StorageKind,
    pub jobs:           Vec<crate::orchestrator::job::Job>
}


impl Strategy {
    pub fn determine_strategy(jobs: Vec<crate::orchestrator::job::Job>) -> Strategy
    {
        let plan: ExecutePlan;
        let storage_kind: StorageKind = Utils::detect_what_kind_of_device_is();
        let total_size: u64 = jobs.iter().map(|j| j.size).sum();

        if jobs.len() == 1 && total_size < LIMIT_FILE_SIZE {
            plan = ExecutePlan::SingleThread { use_chunks: false };
        }

        else if jobs.len() == 1 && total_size > LIMIT_FILE_SIZE {
            plan = ExecutePlan::SingleThread { use_chunks: true };
        }

        else if jobs.len() > 1 && total_size < LIMIT_FILE_SIZE {
            plan = ExecutePlan::Pooled { pool: ThreadPool::new(3), use_chunks: false };
        }

        else {
            plan = ExecutePlan::Pooled { pool: ThreadPool::new(6), use_chunks: true };
        }
        Strategy { plan, storage_kind, jobs }
    }

    pub fn execute(self)
    {
        match self.plan
        {
            ExecutePlan::SingleThread { use_chunks } => {
                let job: Job = self.jobs.into_iter().next().unwrap();
                Worker::single_thread(job, use_chunks);
            },
            ExecutePlan::Pooled { pool, use_chunks } => {
                for job in self.jobs
                {
                    pool.execute(move || {
                        Worker::pooled(job, use_chunks);
                    })
                }
            }
        }
    }
}