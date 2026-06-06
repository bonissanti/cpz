use std::cmp::PartialEq;
use std::fs::DirEntry;
use crate::utils::enums::StorageKind;
use crate::utils::utils::Utils;

const DEFAULT_MINIMUM_CHUNKSIZE: u64 = 3 * 64000000;
const HDD_MINIMUM_CHUNKSIZE: u64 = 3 * 64000000;
const SSD_MINIMUM_CHUNKSIZE: u64 = 3 * 8000000;
const NVME_MINIMUM_CHUNKSIZE: u64 = 3 * 2000000;


pub struct ChunkRange {
    pub offset: u64,
    pub length: u64,
}

impl ChunkRange {
    pub fn split(file_size: u64, storage_kind: &StorageKind)
    {
    }

    pub fn get_chunksize(entry: DirEntry) -> u64
    {
        let storage_kind: StorageKind = Utils::detect_what_kind_of_device_is();//TODO: put it in conditionals
        
        if storage_kind == StorageKind::SSD {
            return SSD_MINIMUM_CHUNKSIZE;
        }

        else if storage_kind == StorageKind::HDD {
            return HDD_MINIMUM_CHUNKSIZE;
        }

        else if storage_kind == StorageKind::NVME {
            return NVME_MINIMUM_CHUNKSIZE;
        }

        return DEFAULT_MINIMUM_CHUNKSIZE
    }
}