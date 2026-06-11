# Module: `cli`

The `cli` module is the **entry point of user interaction**. It is responsible for parsing raw command-line arguments, representing the user's intent as typed data structures, and validating that intent before any file operation begins.

---

## Sub-modules

| File | Responsibility |
|---|---|
| `parser.rs` | Tokenises `argv`, separates flags from file paths |
| `bitflags.rs` | Declares the `Flags` bitmask |
| `cp_data.rs` | Defines the `CpData` transfer object |
| `validation.rs` | Pre-flight checks on sources and destination |

---

## `parser.rs` — Argument Parsing

### What it does

`parser_args(args, flags) -> CpData` iterates over every `String` in `argv` (already stripped of `argv[0]` by `main`).

- Arguments that start with `-` are treated as flag bundles (e.g. `-rv` → recursive + verbose).
- Everything else is accumulated as a file path.
- The **last** path is the destination; all others are sources.

### How flags work

Each recognised character is mapped to a `Flags` bit via `check_flag`:

| Character | Flag inserted |
|---|---|
| `r` | `RECURSIVE` |
| `i` | `NO_DEREFERENCE` |
| `f` | `FORCE` |
| `u` | `UPDATE` |
| `v` | `VERBOSE` |

Unrecognised characters cause an immediate `eprintln!` + `process::exit(1)` — no partial state is returned.

---

## `bitflags.rs` — `Flags` Bitmask

### What it does

Uses the [`bitflags`](https://docs.rs/bitflags) crate to declare a `u8` bitmask:

```rust
pub struct Flags: u8 {
    const RECURSIVE      = 0b00001;  // -r : recurse into directories
    const NO_DEREFERENCE = 0b00010;  // -i : do not follow symlinks
    const FORCE          = 0b00100;  // -f : overwrite without prompting
    const UPDATE         = 0b01000;  // -u : skip if destination is newer
    const VERBOSE        = 0b10000;  // -v : print each copied file
}
```

### Why a bitmask?

- A single `u8` can carry all five boolean flags at once.
- Checking any combination is O(1): `flags.contains(Flags::RECURSIVE | Flags::VERBOSE)`.
- It is `Copy`, so it can be passed by value to every `Job` without heap allocation or cloning.

---

## `cp_data.rs` — `CpData` Transfer Object

```rust
pub struct CpData {
    pub flags:       Flags,
    pub sources:     Vec<String>,
    pub destination: String,
}
```

`CpData` is the structured form of the raw `argv` slice. It is produced once by `parser_args` and consumed by `validation` and `Job::create_job`. It holds no file handles — it only stores intent.

---

## `validation.rs` — Pre-flight Checks

### What it does

`validation(cp_data: &CpData) -> bool` runs a series of guards before any I/O begins. Returning `false` stops execution immediately in `main`.

### Guards on **directories** (`validation_folder`)

| Check | Reason |
|---|---|
| Source is a directory but destination is a file | Cannot overwrite a non-directory with a directory |
| Source is a directory but `-r` not given | Copying a directory requires explicit opt-in |
| Source equals destination (canonical paths) | Would copy into itself |
| Destination is a sub-directory of source | Would create an infinite copy loop |

### Guards on **files** (`validation_file`)

| Check | Reason |
|---|---|
| Source and destination resolve to the same inode | No-op copy is an error |
| Source does not exist | Fast-fail before any thread is spawned |
| Multiple sources but destination is not a directory | Ambiguous target |
| Source not readable | Permission error |
| Destination not writable | Permission error |

Path equality is tested with `fs::canonicalize`, which resolves symlinks and `..` segments before comparison — avoiding false negatives from path-string differences.

---

## Data flow in `cli`

```
argv (Vec<String>)
      │
      ▼
  parser_args()
      │  produces
      ▼
   CpData { flags, sources, destination }
      │
      ▼
  validation()  ──── false ──→  process exits
      │ true
      ▼
  orchestrator::job::Job::create_job()
```
