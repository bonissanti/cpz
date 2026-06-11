# Module: `io`

The `io` module is the **low-level I/O layer**. It decides *how* bytes are moved from source to destination, splits large files into parallelisable chunks, and will eventually implement the actual write primitives.

---

## Sub-modules

| File | Responsibility |
|---|---|
| `strategy.rs` | Decides single-thread vs pooled execution and dispatches work |
| `chunked.rs` | Splits a file into byte-range chunks and reads/writes them at a given offset |
| `write.rs` | (Stub) The two actual copy primitives: direct and chunked |

---

## `strategy.rs` — Execution Strategy

### Core types

```rust
pub enum ExecutePlan {
    SingleThread,
    Pooled { pool: ThreadPool },
}

pub struct Strategy {
    pub plan:         ExecutePlan,
    pub storage_kind: StorageKind,
    pub job:          Vec<Job>,
}
```

### `determine_strategy(jobs) -> Strategy`

Inspects the job list and decides which execution plan to use:

| Condition | Plan chosen |
|---|---|
| Exactly one job **and** it does not need chunking | `SingleThread` |
| Any other combination | `Pooled { pool }` |

The pool is created immediately via `ThreadPool::get_threadpool_by_storage_kind(storage_kind)`, so the number of worker threads is already tuned to the detected hardware before any copy begins.

### `execute(self, ctrl: Arc<ControlState>)`

Drives the chosen plan:

- **`SingleThread`** — calls `Write::copy_direct` directly on the job list.
- **`Pooled`** — calls `ChunkRange::split_file_into_chunks(job.size)` to obtain a `Vec<ChunkRange>`, then iterates over every chunk:
  1. Checks `ctrl.is_cancelled()` before submitting the chunk — if cancelled, returns immediately without enqueuing more work.
  2. Clones the `Arc<ControlState>` and the source/destination `PathBuf`s into the closure (necessary because the closure is `'static` and must own its data).
  3. Submits the closure to the pool via `pool.execute(...)`.
  4. Inside the closure, checks `ctrl.is_cancelled()` again — a cancellation that arrived *after* the task was enqueued is still respected.

### Cancellation contract

The double-check (before enqueue + inside closure) ensures:

- No new tasks are enqueued after cancellation.
- Tasks already queued but not yet running are short-circuited as soon as they get a worker thread.

---

## `chunked.rs` — Chunk Splitting

### `ChunkRange`

```rust
pub struct ChunkRange {
    pub offset: u64,  // byte position where this chunk starts in the file
    pub length: u64,  // number of bytes in this chunk
}
```

### `split_file_into_chunks(file_size) -> Vec<ChunkRange>`

Divides a file into contiguous, non-overlapping ranges:

```
offset = 0
while offset < file_size:
    length = min(chunksize, file_size - offset)
    push ChunkRange { offset, length }
    offset += length
```

The last chunk will have `length < chunksize` if the file size is not a multiple of the chunk size — this is handled by `chunksize.min(remaining)`.

### Chunk sizes per storage kind

Chunk sizes are tuned to the sequential-read sweet spot of each storage type:

| Storage | Constant | Value |
|---|---|---|
| HDD | `HDD_MINIMUM_CHUNKSIZE` | 192 MB (3 × 64 MB) |
| SSD | `SSD_MINIMUM_CHUNKSIZE` | 24 MB (3 × 8 MB) |
| NVMe | `NVME_MINIMUM_CHUNKSIZE` | 6 MB (3 × 2 MB) |
| Default | `DEFAULT_MINIMUM_CHUNKSIZE` | 192 MB |

HDDs benefit from large sequential reads because seek latency dominates. NVMe drives have near-zero seek latency so smaller chunks allow higher parallelism without wasting memory.

### `read_chunk` / `write_chunk`

Both use `FileExt::read_at` / `write_at` (Unix `pread`/`pwrite` under the hood):

```rust
pub fn read_chunk(file: &File, chunk: &ChunkRange, buf: &mut [u8]) -> io::Result<usize> {
    file.read_at(buf, chunk.offset)
}
pub fn write_chunk(file: &File, chunk: &ChunkRange, buf: &[u8]) -> io::Result<usize> {
    file.write_at(buf, chunk.offset)
}
```

`pread`/`pwrite` are **positional** system calls: they take the offset explicitly and do **not** move the file's internal cursor. This means multiple threads can safely call `read_chunk` / `write_chunk` on the **same** `File` descriptor concurrently, each with its own non-overlapping `ChunkRange` — no mutex is needed around the file handle.

---

## `write.rs` — Copy Primitives (stub)

```rust
pub fn copy_direct(src: PathBuf, dest: PathBuf, ctrl: Arc<ControlState>) { todo!() }
pub fn copy_chunk (src: PathBuf, dest: PathBuf, chunk: ChunkRange, ctrl: Arc<ControlState>) { todo!() }
```

- `copy_direct` — intended for small files that do not need chunking; will open source and destination and stream bytes in one pass.
- `copy_chunk` — intended to be the closure body dispatched by `Strategy::execute`; opens both files, allocates a buffer of exactly `chunk.length` bytes, calls `read_chunk`, then `write_chunk`.

Both signatures accept `Arc<ControlState>` so they can check for pause/cancel mid-copy.

---

## Data flow in `io`

```
Strategy::determine_strategy(jobs)
      │
      ├─ SingleThread ──→ Write::copy_direct(job)
      │
      └─ Pooled ──→ ChunkRange::split_file_into_chunks(size)
                         │
                         ▼  (one task per chunk)
                   ThreadPool::execute(closure)
                         │
                         ▼
                   Write::copy_chunk(src, dest, chunk, ctrl)
```
