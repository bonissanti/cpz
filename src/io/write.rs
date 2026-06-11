use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::io::chunked::ChunkRange;
use crate::orchestrator::control::control_state::ControlState;
use crate::orchestrator::job::Job;

pub struct Write {

}

impl Write {
    pub fn copy_direct(src: PathBuf, dest: PathBuf, ctrl: Arc<ControlState>) {
        todo!()
    }

    pub fn copy_chunk(src: PathBuf, dest: PathBuf, chunk: ChunkRange, ctrl: Arc<ControlState>) {
        todo!()
    }
}