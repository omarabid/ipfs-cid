mod error;

use crate::error::{IpfscidError, Result};
use cid::Cid;

/// Generate a CID from a byte slice
pub fn generate_cid(bytes: &[u8]) -> Result<Cid> {
    // create a UnixFS file
    let mut unixfs_file = ipfs_unixfs::file::adder::FileAdder::default();

    // Get the chunch size
    let chunk_size = unixfs_file.size_hint();

    // Split the provided bytes into chunks
    let chunks = bytes.chunks(chunk_size);

    // Push the chunks into the UnixFS file
    for chunk in chunks {
        let _ = unixfs_file.push(chunk);
    }

    // Finalize the UnixFS file
    let unixfs_iterator = unixfs_file.finish();

    // Get the last element of the iterator
    let last_chunk =
        unixfs_iterator.last().ok_or(IpfscidError::NoLastChunk)?;

    Ok(last_chunk.0)
}

/// Generate a CID V0 from a byte slice
pub fn generate_cid_v0(bytes: &[u8]) -> Result<String> {
    let cid = generate_cid(bytes)?;

    Ok(cid.to_string())
}
