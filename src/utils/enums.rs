#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageKind {
    SSD,
    NVME,
    HDD,
    DEFAULT
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Running,
    Stopped,
    Cancelled
}