# Orchestrator Module

The `orchestrator` module is the core execution engine of the CPZ project. It is responsible for parsing user instructions into actionable tasks, scheduling these tasks, and managing their execution concurrently while reporting progress and responding to lifecycle commands (pause/cancel).

## Submodules

### 1. `job`
The `job` module converts a parsed CLI command (`CpData`) into one or more `Job` structs. A `Job` represents a single file to be copied.
- **Single vs. Recursive:** If the user specifies a directory with a recursive flag, `Job::create_multi_jobs` walks the directory using `WalkDir` and generates a flat vector of individual `Job` instances for every file.
- **Dynamic Chunking Needs:** Each job evaluates if its target file requires "chunking" (breaking it into smaller parts for parallel processing) based on the device's storage kind and file size thresholds.

### 2. `worker`
The `worker` module is intended to hold the specific routines for carrying out a `Job`. 
- `single_thread`: Runs the entire job operation sequentially.
- `pooled`: Distributes chunked workloads across the thread pool.

### 3. `control`
The `control` module is responsible for orchestrating the overall execution state (Running, Paused, Cancelled).
- **`ControlState`**: Employs a `Mutex<State>` coupled with a `Condvar` (Condition Variable) to block worker threads efficiently if the operation is paused. It ensures that thread cancellation or pausing operates safely across the concurrent landscape without wasting CPU cycles via busy-waiting.

### 4. `progress`
The `progress` module tracks the state of the system in real-time.
- **`ProgressTracker`**: Uses highly efficient atomic integers (`AtomicU64`, `AtomicU32`) to let multiple threads concurrently increment bytes and files copied without requiring heavy Mutex locks, ensuring the CLI or UI can report statistics accurately without bottlenecking I/O.

---

## The Thread Pool (`thread_pool`)

The Thread Pool is the most complex and critical piece of the orchestrator, warranting a deep dive into Rust's concurrency concepts. 

### Why a Thread Pool?
Instead of spawning a new OS thread for every single file (which can lead to massive overhead and memory exhaustion on large directories), a Thread Pool pre-allocates a fixed number of threads. These threads sit in a loop, waiting for tasks to appear in a queue.

### Storage-Aware Concurrency
The thread pool size is dynamically allocated via `get_threadpool_by_storage_kind` to maximize hardware efficiency without causing bottlenecks:
- **HDD:** 2 threads. Hard drives are highly susceptible to thrashing. Too many concurrent reads/writes force the mechanical head to jump around, completely destroying throughput.
- **SSD:** 4 threads. Solid State Drives handle random I/O much better but still have an internal controller limit.
- **NVMe:** Max CPU cores (`available_parallelism`). NVMe drives are attached directly to the PCIe bus and support massive parallel queue depths, making CPU the actual bottleneck.

### Architecture
The Thread Pool uses the **Message Passing** paradigm via `crossbeam::channel` (a Multi-Producer, Multi-Consumer queue).
1. **The Sender:** The `ThreadPool` holds a `Sender` side of the channel.
2. **The Receiver:** Each spawned thread gets a clone of the `Receiver` side.
3. **The Loop:** Each thread runs an infinite `while let Ok(task) = rx.recv()` loop. When a task is sent, the channel safely wakes up one available thread to execute it.

### Demystifying `Box<dyn FnOnce() + Send + 'static>`
This complex type signature describes the "Task" being sent over the channel.

- **`FnOnce()`:** A trait for closures (anonymous functions) that can be called exactly once. This is perfect for a job task because it executes its payload and consumes any captured variables.
- **`dyn` (Dynamic Dispatch):** Because every closure in Rust has a unique, compiler-generated type, we cannot put them directly into a uniform queue. `dyn` tells the compiler to use "trait objects," resolving the function to call at runtime via a vtable.
- **`Box<...>`:** The size of different closures varies depending on the variables they capture. However, data sent through a channel must have a known size at compile time. `Box` solves this by allocating the closure's data on the *heap* and keeping a fixed-size pointer to it on the *stack*.
- **`Send`:** A marker trait guaranteeing that the closure and all variables it captures are safe to be transferred from the main thread into a worker thread. If you try to pass something not thread-safe (like `Rc`), the compiler will block it.
- **`'static`:** A lifetime bound. It signifies that the closure does not hold references to any short-lived data. Because a worker thread might outlive the scope where the closure was created, passing a standard reference (`&`) could lead to dangling pointers. `'static` forces you to take *ownership* (`move`) of all captured variables or use reference-counted smart pointers (like `Arc`).

### Graceful Shutdown (`Drop`)
When the `ThreadPool` goes out of scope, its `Drop` implementation runs:
1. It `drop(self.sender.take())`. By dropping the sender, the channel is closed.
2. All threads waiting on `rx.recv()` will receive an `Err` and gracefully break out of their loops.
3. The main thread calls `.join()` on all thread handles, ensuring all currently executing tasks finish before the program completely exits.
