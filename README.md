# qcow-rs

[![docs.rs/qcow](https://docs.rs/qcow/badge.svg)](https://docs.rs/qcow)
[![crates.io](https://img.shields.io/crates/v/qcow.svg)](https://crates.io/crates/qcow)
![100% documented](https://img.shields.io/badge/docs-100%25-blueviolet)
![MIT Licensed](https://img.shields.io/github/license/panda-re/qcow-rs)

A Rust library for working with qcow images.

## Example

```rust
// open qcow
let qcow = qcow::open(PATH).unwrap();

// print out list of snapshots in the qcow
for snapshot in qcow.snapshots() {
    println!(
        "Snapshot {:?}: {:?} (size = {})",
        snapshot.unique_id,
        snapshot.name,
        snapshot.vm_state_size
    );
}

// create a reader for accessing the virtual hard disk
let mut file = BufReader::new(File::open(PATH)?);
let qcow2 = qcow.unwrap_qcow2();
let mut reader = qcow2.reader(&mut file);

// read the first 10 bytes of the virtual hard disk
let mut buf = [0; 10];
reader.read_exact(&mut buf)?;
```

## Features

* Parse qcow files
* Full qcow version 1 support
  * Support for parsing the header and some associated data
* Full qcow version 2-3 support
  * Header parsing, including extra version 3 header data
  * Header extension parsing, allowing you to use addition data they provide
  * Lookup table (L1 and L2) parsing, only loading L2 tables on demand
  * Snapshot parsing, including snapshot L1 lookup tables
  * Support for reading the contents of the virtual disk
    * Includes compression support (for both zlib and zstd)
    * Cluster lookup caching, backtracking on cache miss
    * Allows arbitrary seeking within the guest
