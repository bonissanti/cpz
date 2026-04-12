use crate::cli::bitflags::Flags;

pub struct CpData {
    pub flags: Flags,
    pub sources: Vec<String>,
    pub destination: String,
}
