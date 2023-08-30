use ipfs_cid::{generate_cid, generate_cid_v0};

fn main() {
    let bytes_slice = b"hello world";

    let cid = generate_cid(bytes_slice).unwrap();
    println!("Cid Object:\n{:?}", cid);

    let cid_hash = generate_cid_v0(bytes_slice).unwrap();
    println!("Cid V0:\n{}", cid_hash);
}
