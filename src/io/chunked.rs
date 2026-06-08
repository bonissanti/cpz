use std::cmp::PartialEq;
use std::fs::{DirEntry, File};
use std::os::unix::fs::FileExt;
use crate::utils::constants::{DEFAULT_MINIMUM_CHUNKSIZE, HDD_MINIMUM_CHUNKSIZE, NVME_MINIMUM_CHUNKSIZE, SSD_MINIMUM_CHUNKSIZE};
use crate::utils::enums::StorageKind;
use crate::utils::utils::Utils;

pub struct ChunkRange {
    pub offset: u64,
    pub length: u64,
}

impl ChunkRange {
    pub fn split_file_into_chunks(file_size: u64) -> Vec<ChunkRange>
    {
        let mut chunks: Vec<ChunkRange> = Vec::new();
        let mut offset: u64 = 0;
        let chunksize: u64 = Self::get_chunksize();

        while offset < file_size {
            let remaining = file_size - offset;
            let length = chunksize.min(remaining);
            chunks.push(ChunkRange { offset, length });
            offset += length;
        }
        chunks
    }

    pub fn get_chunksize() -> u64
    {
        let storage_kind: StorageKind = Utils::detect_what_kind_of_device_is();

        match storage_kind {
            StorageKind::SSD  => SSD_MINIMUM_CHUNKSIZE,
            StorageKind::HDD  => HDD_MINIMUM_CHUNKSIZE,
            StorageKind::NVME => NVME_MINIMUM_CHUNKSIZE,
            _ => DEFAULT_MINIMUM_CHUNKSIZE
        }
    }

    pub fn read_chunk(file: &File, chunk: &ChunkRange, buf: &mut [u8]) -> std::io::Result<usize> {
        file.read_at(buf, chunk.offset)
    }

    pub fn write_chunk(file: &File, chunk: &ChunkRange, buf: &[u8]) -> std::io::Result<usize> {
        file.write_at(buf, chunk.offset)
    }
}