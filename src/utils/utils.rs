use std::fs;
use std::fs::read_to_string;
use crate::utils::enums::StorageKind;

const SYSFS_PATH: &str = "/sys/block/sda/queue/rotational"; //TODO: this could be nvme0n1 as well, change later

pub struct Utils;

impl Utils {
    pub fn detect_what_kind_of_device_is() -> StorageKind
    {
        let mounts = fs::read_to_string("/proc/mounts").ok();

        for line in mounts.unwrap().lines()
        {
            let mut parts = line.split_whitespace();
            let dev_path = parts.next().unwrap();
            let mountpoint = parts.next().unwrap();

            if mountpoint == "/" {
                continue;
            }

            let dev = dev_path.strip_prefix("/dev/").unwrap();

            if dev.starts_with("nvme") {
                return StorageKind::NVME;
            }

            else if dev.starts_with("mmcblk") {
                return StorageKind::HDD;
            }

            else if dev.starts_with("sd") || dev.starts_with("hd") || dev.starts_with("vd") {
                return Self::detect_rotational_device();
            }
        }
        return StorageKind::DEFAULT;
    }

    fn detect_rotational_device() -> StorageKind
    {
        let storage_kind: String = read_to_string(SYSFS_PATH).unwrap();

        if storage_kind == "0\n" {
            StorageKind::SSD
        }
        else {
            StorageKind::HDD
        }
    }
}