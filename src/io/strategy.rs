use std::net::Shutdown::Write;
use crate::io;
use crate::io::chunked::ChunkRange;
use crate::orchestrator::control::control_state::ControlState;
use crate::orchestrator::job::Job;
use crate::orchestrator::thread_pool::ThreadPool;
use crate::utils::enums::{CopyError, StorageKind};
use crate::utils::utils::Utils;
use std::sync::Arc;

const LIMIT_FILE_SIZE: u64 = 2000000000;

pub enum ExecutePlan {
    SingleThread,
    Pooled { pool: ThreadPool }
}

pub struct Strategy {
    pub plan:           ExecutePlan,
    pub storage_kind:   StorageKind,
    pub job:            Vec<Job>,
}


impl Strategy {
    pub fn determine_strategy(jobs: Vec<crate::orchestrator::job::Job>) -> Strategy
    {
        let plan: ExecutePlan;
        let storage_kind: StorageKind = Utils::detect_what_kind_of_device_is();

        if jobs.iter().all(|j| j.needs_chunking == false) && jobs.len() == 1 {
            plan = ExecutePlan::SingleThread;
        }
        else {
            plan = ExecutePlan::Pooled { pool: ThreadPool::get_threadpool_by_storage_kind(storage_kind) };
        }
        return Strategy { plan, storage_kind, job: jobs };
    }

    fn determine_execute_plan(job: &Job) -> ExecutePlan
    {
        if job.needs_chunking == true {
            return ExecutePlan::Pooled { pool: ThreadPool::get_threadpool_by_storage_kind(job.storage_kind) }
        }
        return ExecutePlan::SingleThread;
    }

    pub fn execute(self, ctrl: Arc<ControlState>)
    {
        match self.plan {
            ExecutePlan::SingleThread => {
                io::write::Write::copy_direct(self.job);
            }
            ExecutePlan::Pooled { pool} => {
                let chunks = ChunkRange::split_file_into_chunks(self.job.size);

                for chunk in chunks {
                    if ctrl.is_cancelled() {
                        return;
                    }

                    let ctrl_clone = Arc::clone(&ctrl);
                    let src = self.job.src.clone();
                    let dest = self.job.dest.clone();

                    pool.execute(move || {
                        if ctrl_clone.is_cancelled() {
                            return;
                        }
                        io::write::Write::copy_chunk(src, dest, chunk)

                    })
                }
            }
        }
    }

    fn handle_single_thread(jobs: Vec<Job>, ctrl: Arc<ControlState>) {
        let job = jobs.into_iter().next().unwrap();

        if job.needs_chunking {
            let chunks = ChunkRange::split_file_into_chunks(job.size);

            for chunk in chunks {
                match io::write::Write::copy_chunk(job.src.clone(), job.dest.clone(), chunk, ctrl) {
                    Ok(_) => {}
                    Err(CopyError::Cancelled) => {
                        // handle cancellation
                    }
                    Err(e) => {
                        // handle other errors
                    }
                }
            }
            return
        }

        match io::write::Write::copy_direct(job.src.clone(), job.dest.clone(), ctrl) {
            Ok(_) => {}
            Err(CopyError::Cancelled) => {
                // handle cancellation
            }
            Err(e) => {
                // handle other errors
            }
        }

    }
}