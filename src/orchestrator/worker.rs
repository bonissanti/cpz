use crate::orchestrator::job::Job;

pub struct Worker {

}

impl Worker {
    pub(crate) fn single_thread(job: Job, use_chunks: bool)
    {
    }

    pub fn pooled(job: Job, use_chunks: bool)
    {
    }
}
