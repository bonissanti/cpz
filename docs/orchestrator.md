# Module: `orchestrator`

The `orchestrator` module is the **core execution engine** of the CPZ project. It is responsible for parsing user instructions into actionable tasks, scheduling these tasks, and managing their execution concurrently while reporting progress and responding to lifecycle commands (pause/cancel).

---

## Sub-modules

| File | Responsibility |
|---|---|
| `job.rs` | Converts a parsed CLI command (`CpData`) into distinct file operations (`Job`s) |
| `worker.rs` | Houses the specific routines (`single_thread` vs `pooled`) for executing a `Job` |
| `control/` | Orchestrates the overall execution state (Running, Stopped, Cancelled) |
| `progress/` | Tracks the state of the system in real-time |
| `thread_pool.rs` | A custom, storage-aware Thread Pool implementation |

---

## `job.rs` — Job Creation

### Core types

```rust
pub struct Job {
    pub id:             Uuid,
    pub src:            PathBuf,
    pub dest:           PathBuf,
    pub flags:          Flags,
    pub size:           u64,
    pub checksum:       bool,
    pub needs_chunking: bool,
    pub storage_kind:   StorageKind,
}
```

### What it does

`Job::create_job(cp_data: &CpData) -> Vec<Job>` is responsible for generating work units. 

| Operation | Action |
|---|---|
| **Single** | `Job::create_single_job` canonicalizes the paths and yields exactly one `Job` |
| **Recursive** | `Job::create_multi_jobs` walks the directory using `WalkDir` and yields a flat `Vec<Job>` of individual files |

### Chunking Check

Each file evaluates whether it requires "chunking" (breaking it into smaller byte ranges for parallel processing) through `define_if_chunk_is_needed(file_size)`. This check is dynamically sized against thresholds based on the underlying `StorageKind` (HDD vs SSD vs NVMe).

---

## `control/control_state.rs` — Lifecycle Control

### Core types

```rust
pub struct ControlState {
    pub state: Mutex<State>,
    pub cv:    Condvar,
}
```

### Why a `Condvar`?

Instead of burning CPU cycles with `loop { if state == Paused { thread::yield_now() } }`, a `Condvar` (Condition Variable) allows worker threads to comfortably sleep. When the user signals a pause, threads lock the `Mutex<State>`, see `State::Stopped`, and wait on the `Condvar`. When resumed, a single signal wakes all sleeping threads.

---

## `progress/progress_tracker.rs` — Progress Tracking

### Core types

```rust
pub struct ProgressTracker {
    pub bytes_coped:     AtomicU64,
    pub bytes_total:     AtomicU64,
    pub files_completed: AtomicU32,
    pub files_total:     AtomicU32,
}
```

The progress module uses highly efficient `AtomicU64` and `AtomicU32` types to allow multiple threads to concurrently increment bytes without locking a heavy `Mutex`. This prevents I/O from bottlenecking on telemetry.

---

## `thread_pool.rs` — Storage-Aware Concurrency

Instead of spawning a new OS thread for every single file—which exhausts memory and OS scheduler time—a fixed number of threads sit in a loop, waiting for tasks via an MPMC (Multi-Producer, Multi-Consumer) channel.

### `StorageKind` scaling

`ThreadPool::get_threadpool_by_storage_kind` sizes the pool dynamically:

| Storage Kind | Pool Size | Reason |
|---|---|---|
| `HDD` | 2 | Mechanical drives suffer extreme thrashing under heavy concurrent I/O |
| `SSD` | 4 | Solid State Drives handle random I/O better but are limited by controller saturation |
| `NVMe` | Max CPUs | Attached directly to PCIe; huge parallel queue depths handle CPU-bound I/O |

### The Task Queue (`Box<dyn FnOnce() + Send + 'static>`)

The type sent down the `crossbeam::channel` perfectly illustrates Rust's safe concurrency model:

- `FnOnce()`: A closure that can be called exactly once (consuming its state).
- `dyn`: Dynamic dispatch; a trait object allowing closures of different internal memory layouts to exist in the same queue.
- `Box`: Allocates the closure on the heap so its size is known at compile-time.
- `Send`: Guarantees the closure and its captured variables are safe to move to another thread.
- `'static`: Ensures the closure doesn't hold references to local stack variables that might be dropped before the thread executes it.

### Graceful Shutdown (`Drop`)

When the `ThreadPool` goes out of scope:
1. `drop(self.sender.take())` closes the channel.
2. Worker threads receive an `Err` on `rx.recv()` and break their infinite loops.
3. The main thread calls `.join()` on all handles, ensuring all actively running closures finish.

---

## Data flow in `orchestrator`

```text
  Job::create_job(CpData)
          │
          ▼
    [Job, Job, Job]
          │
          ▼
  io::strategy::determine_strategy()
          │
          ▼
   ThreadPool::execute(|| {
       // Check ControlState
       // Update ProgressTracker
       // Execute copy
   })
```