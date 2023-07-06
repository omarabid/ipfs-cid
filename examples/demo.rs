use ipfs_cid::{generate_cid, generate_cid_hash};

fn main() {
    let bytes_slice = b"hello world";

    let cid = generate_cid(bytes_slice).unwrap();
    println!("{:?}", cid);

    let cid_hash = generate_cid_hash(bytes_slice).unwrap();
    println!("{}", cid_hash);
}
