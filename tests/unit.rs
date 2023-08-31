use ipfs_cid::{
    convert_cid_v0_to_uint256, convert_cid_v1_to_uint256,
    convert_uint256_to_cid_v0, convert_uint256_to_cid_v1, generate_cid_v0,
    generate_cid_v1,
};

#[test]
fn test_generate_v0_from_bytes_file0() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file0").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v0(bytes_slice).unwrap();
    let expected_cid =
        String::from("QmUBnCzebDwZgkXp9ZkHHKQNfaeWn2Dw8p8vNz4GN4jBLa");

    assert_eq!(cid, expected_cid);
}

#[test]
fn test_generate_v0_from_bytes_file1() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file1").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v0(bytes_slice).unwrap();
    let expected_cid =
        String::from("Qmb88GEBKEdbRu7gP9aQB3bewTR3fWLKEsdJs7P6ubArCT");

    assert_eq!(cid, expected_cid);
}

#[test]
fn test_generate_v0_from_bytes_file2() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file2").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v0(bytes_slice).unwrap();
    let expected_cid =
        String::from("QmYGrF2MwAoPLDKkMkmHCWieR2xa2FxbXh1B6jhCXA1anU");

    assert_eq!(cid, expected_cid);
}

#[test]
fn test_generate_v0_from_bytes_file3() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file3").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v0(bytes_slice).unwrap();
    let expected_cid =
        String::from("QmTDq2AoRKR5NSwv7D47AAYQxsZRFP54MWJxJGj5YDJXa6");

    assert_eq!(cid, expected_cid);
}

#[test]
fn test_generate_v0_from_bytes_file4() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file4").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v0(bytes_slice).unwrap();
    let expected_cid =
        String::from("QmeSWm3xv7VWfWgFYoaDxP7nbinaSPqXyMFJQuzLMjUyQD");

    assert_eq!(cid, expected_cid);
}

#[test]
fn test_generate_v1_from_bytes_file0() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file0").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v1(bytes_slice).unwrap();
    let expected_cid = String::from(
        "bafybeicw4bpvhscvz2a7in5bytor4bx7kmp3k2ctkwm4et4ri5dx2ypt64",
    );

    assert_eq!(cid, expected_cid);
}

#[test]
fn test_generate_v1_from_bytes_file1() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file1").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v1(bytes_slice).unwrap();
    let expected_cid = String::from(
        "bafybeif56keohn57vtbnwiboeoldxinsbmuswkpo3ekjjo6m2qj4lmxwyq",
    );

    assert_eq!(cid, expected_cid);
}

#[test]
fn test_generate_v1_from_bytes_file2() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file2").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v1(bytes_slice).unwrap();
    let expected_cid = String::from(
        "bafybeiettolckdkgpy2jzix6ocs2ut7viscalntq2jzgj4u56q4lfmgxae",
    );

    assert_eq!(cid, expected_cid);
}

#[test]
fn test_generate_v1_from_bytes_file3() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file3").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v1(bytes_slice).unwrap();
    let expected_cid = String::from(
        "bafybeicirmdtizyoxvto4layqmahze6rtmurxr6h2hh6iu2qmcuqyloyp4",
    );

    assert_eq!(cid, expected_cid);
}

#[test]
fn test_generate_v1_from_bytes_file4() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file4").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v1(bytes_slice).unwrap();
    let expected_cid = String::from(
        "bafybeihphn6mvbd3ezn6ikmdmtflnrxcybdytyrd734crkomqax3iubhxi",
    );

    assert_eq!(cid, expected_cid);
}

#[test]
fn test_convert_v0_to_uint256_and_back_file0() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file0").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v0(bytes_slice).unwrap();
    // Convert CID to uint256
    let uint256 = convert_cid_v0_to_uint256(cid.clone()).unwrap();
    // Convert uint256 back to CID
    let cid_back = convert_uint256_to_cid_v0(uint256).unwrap();

    assert_eq!(cid, cid_back);
}

#[test]
fn test_convert_v0_to_uint256_and_back_file1() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file1").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v0(bytes_slice).unwrap();
    // Convert CID to uint256
    let uint256 = convert_cid_v0_to_uint256(cid.clone()).unwrap();
    // Convert uint256 back to CID
    let cid_back = convert_uint256_to_cid_v0(uint256).unwrap();

    assert_eq!(cid, cid_back);
}

#[test]
fn test_convert_v0_to_uint256_and_back_file2() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file2").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v0(bytes_slice).unwrap();
    // Convert CID to uint256
    let uint256 = convert_cid_v0_to_uint256(cid.clone()).unwrap();
    // Convert uint256 back to CID
    let cid_back = convert_uint256_to_cid_v0(uint256).unwrap();

    assert_eq!(cid, cid_back);
}

#[test]
fn test_convert_v0_to_uint256_and_back_file3() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file3").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v0(bytes_slice).unwrap();
    // Convert CID to uint256
    let uint256 = convert_cid_v0_to_uint256(cid.clone()).unwrap();
    // Convert uint256 back to CID
    let cid_back = convert_uint256_to_cid_v0(uint256).unwrap();

    assert_eq!(cid, cid_back);
}

#[test]
fn test_convert_v0_to_uint256_and_back_file4() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file4").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v0(bytes_slice).unwrap();
    // Convert CID to uint256
    let uint256 = convert_cid_v0_to_uint256(cid.clone()).unwrap();
    // Convert uint256 back to CID
    let cid_back = convert_uint256_to_cid_v0(uint256).unwrap();

    assert_eq!(cid, cid_back);
}

#[test]
fn test_convert_v1_to_uint256_and_back_file0() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file0").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v1(bytes_slice).unwrap();
    // Convert CID to uint256
    let uint256 = convert_cid_v1_to_uint256(cid.clone()).unwrap();
    // Convert uint256 back to CID
    let cid_back = convert_uint256_to_cid_v1(uint256).unwrap();

    assert_eq!(cid, cid_back);
}

#[test]
fn test_convert_v1_to_uint256_and_back_file1() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file1").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v1(bytes_slice).unwrap();
    // Convert CID to uint256
    let uint256 = convert_cid_v1_to_uint256(cid.clone()).unwrap();
    // Convert uint256 back to CID
    let cid_back = convert_uint256_to_cid_v1(uint256).unwrap();

    assert_eq!(cid, cid_back);
}

#[test]
fn test_convert_v1_to_uint256_and_back_file2() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file2").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v1(bytes_slice).unwrap();
    // Convert CID to uint256
    let uint256 = convert_cid_v1_to_uint256(cid.clone()).unwrap();
    // Convert uint256 back to CID
    let cid_back = convert_uint256_to_cid_v1(uint256).unwrap();

    assert_eq!(cid, cid_back);
}

#[test]
fn test_convert_v1_to_uint256_and_back_file3() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file3").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v1(bytes_slice).unwrap();
    // Convert CID to uint256
    let uint256 = convert_cid_v1_to_uint256(cid.clone()).unwrap();
    // Convert uint256 back to CID
    let cid_back = convert_uint256_to_cid_v1(uint256).unwrap();

    assert_eq!(cid, cid_back);
}

#[test]
fn test_convert_v1_to_uint256_and_back_file4() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file4").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_cid_v1(bytes_slice).unwrap();
    // Convert CID to uint256
    let uint256 = convert_cid_v1_to_uint256(cid.clone()).unwrap();
    // Convert uint256 back to CID
    let cid_back = convert_uint256_to_cid_v1(uint256).unwrap();

    assert_eq!(cid, cid_back);
}
