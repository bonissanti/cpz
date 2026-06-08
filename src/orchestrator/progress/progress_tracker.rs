use std::sync::atomic::{AtomicU32, AtomicU64};

pub struct ProgressTracker {
    pub bytes_coped:        AtomicU64,
    pub bytes_total:        AtomicU64,
    pub files_completed:    AtomicU32,
    pub files_total:        AtomicU32,
}
