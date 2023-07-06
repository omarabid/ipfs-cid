# ipfs-cid

This crates provides a simple function to generate IPFS CIDs (Content
Identifiers) from a slice of bytes.

[What is a CID?](https://docs.ipfs.tech/concepts/content-addressing/#what-is-a-cid)

[![CI](https://github.com/omarabid/ipfs-cid/actions/workflows/ci.yml/badge.svg)](https://github.com/omarabid/ipfs-cid/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/)](https://crates.io/crates/)

## Usage

Two functions are available: `generate_cid` and `generate_cid_hash`. The first
one returns the full CID object as defined in the cid crate (version 0.5.1). The second
one returns the hash only as a `String`.

```rust
    let bytes_vector = std::fs::read("data/file0").unwrap();
    let bytes_slice = bytes_vector.as_slice();

    let cid_hash = generate_cid_hash(bytes_slice).unwrap();
    println!("{}", cid_hash);
```

This should return

```bash
QmUBnCzebDwZgkXp9ZkHHKQNfaeWn2Dw8p8vNz4GN4jBLa
```

The file is accessible from IPFS at the same hash: [QmUBnCzebDwZgkXp9ZkHHKQNfaeWn2Dw8p8vNz4GN4jBLa](https://ipfs.io/ipfs/QmUBnCzebDwZgkXp9ZkHHKQNfaeWn2Dw8p8vNz4GN4jBLa)

### License

This project is licensed under

- MIT license ([LICENSE](LICENSE) or https://opensource.org/licenses/MIT)
