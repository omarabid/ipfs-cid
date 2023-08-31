mod error;

use crate::error::{IpfscidError, Result};
use cid::multihash::Code;
use cid::multihash::MultihashDigest;
use cid::Cid;

pub fn generate_cid(bytes: &[u8]) -> Result<Cid> {
    let h = Code::Sha2_256.digest(bytes);

    // create a UnixFS file
    let mut unixfs_file = ipfs_unixfs::file::adder::FileAdder::default();

    // Get the chunck size
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

    let h2 = Code::Sha2_256.digest(&last_chunk.1);

    let cid = Cid::new_v0(h2).unwrap();

    let data = cid.to_bytes();
    let out = Cid::try_from(data).unwrap();

    Ok(cid)
}

/// Generate a CID V0 from a byte slice
pub fn generate_cid_v0(bytes: &[u8]) -> Result<String> {
    let cida = generate_cid(bytes)?;

    Ok(cida.to_string())
}

/// Generate a CID V1 from a byte slice
pub fn generate_cid_v1(bytes: &[u8]) -> Result<String> {
    let cida = generate_cid(bytes)?;

    // convert cidv0 to cidv1
    let cida1 = Cid::new_v1(cida.codec(), *cida.hash());

    Ok(cida1.to_string())
}

pub fn convert_cid_v0_to_uint256(cid_0: String) -> Result<String> {
    let cid = Cid::try_from(cid_0).unwrap();

    let digest = cid.hash().digest();

    let uint256 = num_bigint::BigUint::from_bytes_le(digest);

    dbg!(uint256);

    Ok(String::from("to be implemented"))
}

pub fn convert_cid_v1_to_uint256(cid_1: String) -> Result<String> {
    let cid = Cid::try_from(cid_1).unwrap();

    let digest = cid.hash().digest();

    let uint256 = num_bigint::BigUint::from_bytes_le(digest);

    dbg!(uint256);
    Ok(String::from("to be implemented"))
}

pub fn convert_uint256_to_cid_v0(uint256: String) -> Result<String> {
    let bigint = num_bigint::BigUint::parse_bytes(uint256.as_bytes(), 10);

    // convert bigint to bytes
    let mut bytes = bigint.unwrap().to_bytes_le();

    // the bytes are the digest already. We just need to convert it to a CID
    // but we need to add the multicodec
    bytes.insert(0, 0x12);
    bytes.insert(1, 0x20);
    let multihash = cid::multihash::Multihash::from_bytes(&bytes).unwrap();
    let cid = cid::Cid::new_v0(multihash).unwrap();
    dbg!(cid);
    Ok(cid.to_string())
}

pub fn convert_uint256_to_cid_v1(uint256: String) -> Result<String> {
    Ok(String::from("to be implemented"))
}
