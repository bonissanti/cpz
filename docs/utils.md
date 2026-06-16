# Utils Module

The `utils` module is the overarching location for project-wide utilities, constants, data structures, and hardware detection logic.

## Submodules

### 1. `enums`
Provides global enumerations used to represent state and characteristics across the entire codebase.
- **`StorageKind`**: Represents the physical nature of the storage device (`SSD`, `NVMe`, `HDD`, `DEFAULT`). This heavily impacts how the program schedules I/O operations and parallelism.
- **`State`**: Represents the current execution state of the copy process (`Running`, `Stopped`, `Cancelled`), utilized heavily by the orchestrator's control flow.
- **`CopyError`**: A generalized error enum to encapsulate domain-specific failures (like a copy being cancelled by the user).

### 2. `constants`
Holds compile-time numerical constants crucial for tuning the application's performance characteristics.
- **Chunk Sizes (`HDD_MINIMUM_CHUNKSIZE`, `SSD_MINIMUM_CHUNKSIZE`, etc.):** When a file is exceedingly large, transferring it linearly on a single thread is inefficient. These constants define the threshold at which a file should be chunked into smaller blocks and parallelized, optimized per storage medium.

### 3. `utils`
Contains standalone helper functions. The standout feature is the **Hardware Detection Logic**.

#### Hardware Detection (`detect_what_kind_of_device_is`)
To optimize concurrency and chunking, CPZ attempts to deduce the underlying disk technology running the OS's mount points.
1. **Reads `/proc/mounts`**: Scans the active mounts on the Linux system to find the physical block device backing the root `/` partition.
2. **Device Name Parsing**: 
   - `nvme...` is immediately flagged as `NVMe`.
   - `mmcblk...` (SD Cards/eMMC) is flagged as `HDD` due to typical low random I/O performance.
   - `sd...`, `hd...`, `vd...` (SATA/SCSI/Virtual) require further inspection.
3. **Reads `/sys/block/.../queue/rotational`**: For ambiguous devices, it checks the kernel's `sysfs`. If the drive reports `1` (rotational), it's a mechanical HDD. If it reports `0`, it's an SSD.
This logic empowers the orchestrator to instantiate the correct Thread Pool size to prevent hardware thrashing.
