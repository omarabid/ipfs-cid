use ipfs_cid::{generate_cid, generate_cid_v0, generate_cid_v1};

fn main() {
    let bytes_slice = b"hello world";

    let cid = generate_cid(bytes_slice).unwrap();
    println!("Cid Object:\n{:?}", cid);

    let cid_hash = generate_cid_v0(bytes_slice).unwrap();
    println!("Cid V0:\n{}", cid_hash);

    let bytes_slice = b"@hello world";

    let cid = generate_cid(bytes_slice).unwrap();
    println!("Cid Object:\n{:?}", cid);

    let cid_hash = generate_cid_v0(bytes_slice).unwrap();
    println!("Cid V0:\n{}", cid_hash);

    let cid_1_hash = generate_cid_v1(bytes_slice).unwrap();
    println!("Cid V1:\n{}", cid_1_hash);
}
