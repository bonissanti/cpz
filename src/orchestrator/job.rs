use std::fs;
use std::path::PathBuf;
use uuid::Uuid;
use crate::cli::bitflags::Flags;
use crate::cli::cp_data::CpData;

pub struct Job {
    pub id:         Uuid,
    pub src:        PathBuf,
    pub dest:       PathBuf,
    pub flags:      Flags,
    pub size:       u64,
    pub checksum:   bool
}

impl Job {
    pub fn create_job(cp_data: &CpData)
    {
        let mut jobs: Vec<Job> = Vec::new();

        for src in &cp_data.sources {
            if cp_data.flags.contains(Flags::RECURSIVE) {
                print!("Recursively copying directory: {}", src);
            }
            else {
                let new_job: Job = Self::create_single_job(src, &cp_data.destination, cp_data.flags);
                jobs.push(new_job);
            }
        }
    }

    fn create_single_job(src: &String, dest: &String, flags: Flags) -> Job
    {
        let src_relative_path = PathBuf::from(src);
        let src_absolute_path = fs::canonicalize(src_relative_path).unwrap();

        let dest_relative_path = PathBuf::from(dest);
        let dest_absolute_path = fs::canonicalize(dest_relative_path).unwrap();

        Job {
            id: Uuid::new_v4(),
            src: src_absolute_path,
            dest: dest_absolute_path,
            flags,
            size: 0,
            checksum: false
        }
    }

    fn create_multi_job(src: &String, dest: &String, flags: Flags)
    {

    }
}