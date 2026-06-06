#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageKind {
    SSD,
    NVME,
    HDD,
    DEFAULT
}

