use ipfs_cid::generate_from_bytes;

#[test]
fn test_generate_from_bytes_file0() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file0").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_from_bytes(bytes_slice).unwrap();
    let expected_cid =
        String::from("QmUBnCzebDwZgkXp9ZkHHKQNfaeWn2Dw8p8vNz4GN4jBLa");

    assert_eq!(cid, expected_cid);
}

#[test]
fn test_generate_from_bytes_file1() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file1").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_from_bytes(bytes_slice).unwrap();
    let expected_cid =
        String::from("Qmb88GEBKEdbRu7gP9aQB3bewTR3fWLKEsdJs7P6ubArCT");

    assert_eq!(cid, expected_cid);
}

#[test]
fn test_generate_from_bytes_file2() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file2").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_from_bytes(bytes_slice).unwrap();
    let expected_cid =
        String::from("QmYGrF2MwAoPLDKkMkmHCWieR2xa2FxbXh1B6jhCXA1anU");

    assert_eq!(cid, expected_cid);
}

#[test]
fn test_generate_from_bytes_file3() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file3").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_from_bytes(bytes_slice).unwrap();
    let expected_cid =
        String::from("QmTDq2AoRKR5NSwv7D47AAYQxsZRFP54MWJxJGj5YDJXa6");

    assert_eq!(cid, expected_cid);
}

#[test]
fn test_generate_from_bytes_file4() {
    // Read file into bytes
    let bytes_vector = std::fs::read("data/file4").unwrap();
    let bytes_slice = bytes_vector.as_slice();
    // Generate CID from bytes
    let cid = generate_from_bytes(bytes_slice).unwrap();
    let expected_cid =
        String::from("QmeSWm3xv7VWfWgFYoaDxP7nbinaSPqXyMFJQuzLMjUyQD");

    assert_eq!(cid, expected_cid);
}
