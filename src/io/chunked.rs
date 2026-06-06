use crate::io::strategy::StorageKind;

pub struct ChunkRange {
    pub offset: u64,
    pub length: u64
}

impl ChunkRange {
    pub fn split(file_size: u64, storage_kind: &StorageKind)
    {
    }
}