use std::fs;
use super::enums::StorageKind;

const SYSFS_PATH: &str = "/sys/block/sda/queue/rotational"; //TODO: this could be nvme0n1 as well, change later

pub struct Utils;

impl Utils {
    pub fn detect_what_kind_of_device_is() -> StorageKind {
        Self::detect_device_from_paths("/proc/mounts", SYSFS_PATH)
    }

    pub(crate) fn detect_device_from_paths(mounts_path: &str, sysfs_path: &str) -> StorageKind {
        let Ok(mounts) = fs::read_to_string(mounts_path) else {
            return StorageKind::DEFAULT;
        };

        for line in mounts.lines() {
            let mut parts = line.split_whitespace();
            let Some(dev_path) = parts.next() else { continue; };
            let Some(mountpoint) = parts.next() else { continue; };

            if mountpoint != "/" {
                continue;
            }

            let Some(dev) = dev_path.strip_prefix("/dev/") else { continue; };

            if dev.starts_with("nvme") {
                return StorageKind::NVME;
            }
            else if dev.starts_with("mmcblk") {
                return StorageKind::HDD;
            }
            else if dev.starts_with("sd") || dev.starts_with("hd") || dev.starts_with("vd") {
                return Self::detect_rotational_device_from_path(sysfs_path);
            }
        }
        StorageKind::DEFAULT
    }

    pub(crate) fn detect_rotational_device_from_path(sysfs_path: &str) -> StorageKind {
        let Ok(storage_kind) = fs::read_to_string(sysfs_path) else {
            return StorageKind::DEFAULT;
        };

        if storage_kind.trim() == "0" {
            StorageKind::SSD
        } else {
            StorageKind::HDD
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::env;

    fn create_fixture_file(name: &str, content: &str) -> String {
        let mut path = env::temp_dir();
        path.push(name);
        let mut file = File::create(&path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        path.to_str().unwrap().to_string()
    }

    #[test]
    fn detects_nvme_device_correctly() {
        let mounts_path = create_fixture_file("mounts_nvme", "/dev/nvme0n1p1 /mnt ext4 rw,relatime 0 0\n");
        let sysfs_path = create_fixture_file("sysfs_nvme", "0\n");
        
        let result = Utils::detect_device_from_paths(&mounts_path, &sysfs_path);
        assert_eq!(result, StorageKind::NVME);
    }

    #[test]
    fn detects_mmcblk_as_hdd_correctly() {
        let mounts_path = create_fixture_file("mounts_mmcblk", "/dev/mmcblk0p1 /boot vfat rw,relatime 0 0\n");
        let sysfs_path = create_fixture_file("sysfs_mmcblk", "1\n");
        
        let result = Utils::detect_device_from_paths(&mounts_path, &sysfs_path);
        assert_eq!(result, StorageKind::HDD);
    }

    #[test]
    fn detects_sd_device_as_ssd_when_rotational_is_zero() {
        let mounts_path = create_fixture_file("mounts_sd_ssd", "/dev/sda1 /data ext4 rw,relatime 0 0\n");
        let sysfs_path = create_fixture_file("sysfs_sd_ssd", "0\n");
        
        let result = Utils::detect_device_from_paths(&mounts_path, &sysfs_path);
        assert_eq!(result, StorageKind::SSD);
    }

    #[test]
    fn detects_sd_device_as_hdd_when_rotational_is_one() {
        let mounts_path = create_fixture_file("mounts_sd_hdd", "/dev/sdb1 /backup ext4 rw,relatime 0 0\n");
        let sysfs_path = create_fixture_file("sysfs_sd_hdd", "1\n");
        
        let result = Utils::detect_device_from_paths(&mounts_path, &sysfs_path);
        assert_eq!(result, StorageKind::HDD);
    }

    #[test]
    fn returns_default_when_device_is_unknown() {
        let mounts_path = create_fixture_file("mounts_unknown", "/dev/mapper/cryptroot / ext4 rw,relatime 0 0\ntmpfs /run tmpfs rw 0 0\n");
        let sysfs_path = create_fixture_file("sysfs_unknown", "1\n");
        
        let result = Utils::detect_device_from_paths(&mounts_path, &sysfs_path);
        assert_eq!(result, StorageKind::DEFAULT);
    }
}
