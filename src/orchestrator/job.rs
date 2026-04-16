use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use walkdir::WalkDir;
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
    pub fn create_job(cp_data: &CpData) -> Vec<Job>
    {
        let mut jobs: Vec<Job> = Vec::new();

        for src in &cp_data.sources {
            if cp_data.flags.contains(Flags::RECURSIVE) {
                let mut new_jobs = Self::create_multi_jobs(src, &cp_data.destination, cp_data.flags);
                jobs.append(&mut new_jobs);
            }
            else {
                let new_job: Job = Self::create_single_job(src, &cp_data.destination, cp_data.flags);
                jobs.push(new_job);
            }
        }

        jobs
    }

    fn create_single_job(src: &String, dest: &String, flags: Flags) -> Job
    {
        let src_relative_path: PathBuf = PathBuf::from(src);
        let src_absolute_path: PathBuf = fs::canonicalize(&src_relative_path).unwrap();
        let size: u64 = fs::metadata(&src_absolute_path).unwrap().len();

        let dest_relative_path = PathBuf::from(dest);
        let dest_absolute_path = fs::canonicalize(dest_relative_path).unwrap();

        let dest_absolute_path = dest_absolute_path.join(src_relative_path.file_name().unwrap());

        Job {
            id: Uuid::new_v4(),
            src: src_absolute_path,
            dest: dest_absolute_path,
            flags,
            size,
            checksum: false
        }
    }

    fn create_multi_jobs(src: &String, dest: &String, flags: Flags) -> Vec<Job>
    {
        let mut jobs: Vec<Job> = Vec::new();

        let src_relative_path: PathBuf = PathBuf::from(src);
        let src_absolute_path: PathBuf = fs::canonicalize(&src_relative_path).unwrap();

        let dest_relative_path = PathBuf::from(dest);
        let dest_absolute_path = fs::canonicalize(dest_relative_path).unwrap();

        for entry in WalkDir::new(&src_absolute_path)
        {
            let entry = entry.unwrap();

            if entry.file_type().is_dir() {
                continue;
            }

            let dest_file_abs = dest_absolute_path.join(&src_relative_path);

            jobs.push(Job {
                id: Uuid::new_v4(),
                src: entry.path().to_path_buf(),
                dest: dest_file_abs,
                flags,
                size: entry.metadata().map(|m| m.len()).unwrap_or(0),
                checksum: false
            })
        }

        jobs
    }
}