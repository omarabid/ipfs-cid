use ipfs_cid::{
    convert_cid_v0_to_uint256, convert_cid_v1_to_uint256,
    convert_uint256_to_cid_v0, convert_uint256_to_cid_v1, generate_cid,
    generate_cid_v0, generate_cid_v1,
};

fn main() {
    // let bytes_slice = b"hello world";

    // let cid = generate_cid(bytes_slice).unwrap();
    // println!("Cid Object:\n{:?}", cid);

    // let cid_hash = generate_cid_v0(bytes_slice).unwrap();
    // println!("Cid V0:\n{}", cid_hash);

    // let bytes_slice = b"@hello world";

    // let cid = generate_cid(bytes_slice).unwrap();
    // println!("Cid Object:\n{:?}", cid);

    // let cid_hash = generate_cid_v0(bytes_slice).unwrap();
    // println!("Cid V0:\n{}", cid_hash);

    // let cid_1_hash = generate_cid_v1(bytes_slice).unwrap();
    // println!("Cid V1:\n{}", cid_1_hash);
    //
    //
    let cid_v0 = "QmRjNqS3NsEGt8XUirq4F414tGwLx91EJmeZNmAJgSoTBP";
    let int256: String =
        convert_cid_v0_to_uint256(cid_v0.to_string()).unwrap();
    dbg!(int256);

    let cid_v1 = "bafybeibsmv4jadj5sxd5yd3kbap6h5nwidv67rkixn7bfxrcsf4up4334i";
    let int256: String =
        convert_cid_v1_to_uint256(cid_v1.to_string()).unwrap();
    dbg!(int256);

    let uint256 = "102441705020106921381567678423113190717728123273146714193141001409376032417074";
    let cid_v0: String =
        convert_uint256_to_cid_v0(uint256.to_string()).unwrap();
    let cid_v1: String =
        convert_uint256_to_cid_v1(uint256.to_string()).unwrap();
    dbg!(cid_v0);
    dbg!(cid_v1);
}
