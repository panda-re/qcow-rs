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

## Library Features

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

## Command Line Interface

Also present is a cli for interfacing with qcows.

```
qcow-cli 0.1.0
A utility for querying information about qcow files

USAGE:
    qcow <qcow> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <qcow>

SUBCOMMANDS:
    get-file      Output a file within the qcow to stdout
    help          Prints this message or the help of the given subcommand(s)
    info          Output info about the given qcow
    partitions    Display a list of partitions in the qcow image
    tree          Display a tree listing of the contents of the qcow
```

### Install

```
cargo install qcow-cli
```

### Screenshots

![info command](https://raw.githubusercontent.com/panda-re/qcow-rs/master/cli/screenshots/info.png?raw=true)
![tree command](https://raw.githubusercontent.com/panda-re/qcow-rs/master/cli/screenshots/tree.png?raw=true)
![partitions command](https://raw.githubusercontent.com/panda-re/qcow-rs/master/cli/screenshots/partitions.png?raw=true)
![get-file command](https://raw.githubusercontent.com/panda-re/qcow-rs/master/cli/screenshots/get-file.png?raw=true)
