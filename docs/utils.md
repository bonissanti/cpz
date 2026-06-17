# Module: `utils`

The `utils` module is the overarching location for project-wide enumerations, numerical thresholds, standalone helper functions, and crucially, **hardware detection logic**.

---

## Sub-modules

| File | Responsibility |
|---|---|
| `enums.rs` | Global state and domain types |
| `constants.rs` | Compile-time numerical tunables |
| `utils.rs` | Utility functions, notably disk-type detection |

---

## `enums.rs` — Global Types

### Core Enums

| Enum | Variants | Purpose |
|---|---|---|
| `StorageKind` | `SSD`, `NVMe`, `HDD`, `DEFAULT` | Informs the size of the Thread Pool and Chunk capacities |
| `State` | `Running`, `Stopped`, `Cancelled` | Held within `ControlState` to coordinate pause/resume/cancel |
| `CopyError` | `Cancelled` | Generic wrapper for domain-specific execution halts |

---

## `constants.rs` — Numerical Tunables

```rust
pub const HDD_MINIMUM_CHUNKSIZE: u64 = 3 * 64_000_000;  // 192 MB
pub const SSD_MINIMUM_CHUNKSIZE: u64 = 3 * 8_000_000;   // 24 MB
pub const NVME_MINIMUM_CHUNKSIZE: u64 = 3 * 2_000_000;  // 6 MB
```

These constants define the size thresholds at which single-threaded operations become pooled, chunked operations. They vary by `StorageKind` because an NVMe can handle micro-parallelism, while a mechanical HDD requires huge sequential buffers to maintain throughput.

---

## `utils.rs` — Hardware Detection

### What it does

`Utils::detect_what_kind_of_device_is()` attempts to deduce the underlying disk technology running the OS's root partition (`/`).

### How it works on Linux

1. **Read Mounts:** Scans `/proc/mounts` to find the physical block device mapped to `/`.
2. **Name Heuristics:**
    - If the device name starts with `nvme`, return `StorageKind::NVMe`.
    - If it starts with `mmcblk` (eMMC / SD Cards), return `StorageKind::HDD` (low random I/O).
3. **Sysfs Inspection:** If the device is standard SCSI/SATA (`sdX`, `hdX`, `vdX`), it requires deeper inspection.
    - It reads `/sys/block/<device>/queue/rotational`.
    - `0` means Solid State → `StorageKind::SSD`
    - `1` means Mechanical → `StorageKind::HDD`

This low-level check empowers the entire `orchestrator` and `io` layer to dynamically tune itself to the host's actual hardware without asking the user.

---

## Data flow in `utils`

```text
  Utils::detect_what_kind_of_device_is()
          │
          ├─ "nvme..." ──→ NVMe
          │
          ├─ "mmcblk..." ──→ HDD
          │
          └─ "sdX" ──→ /sys/block/sdX/queue/rotational
                                │
                                ├─ 0 ──→ SSD
                                └─ 1 ──→ HDD
```