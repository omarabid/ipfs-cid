// ipfs-cid is a tool to generate the CID of a file.
// The CLI is simple and only takes a file path as an argument and prints the CID to STDOUT.
pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ipfs-cid <file>");
        std::process::exit(1);
    }
    let path = &args[1];
    let file = std::fs::read(path).expect("Failed to read file");
    let cid = ipfs_cid::generate_cid_v0(&file);
    match cid {
        Ok(cid) => println!("{}", cid),
        Err(e) => eprintln!("Failed to generate CID: {}", e),
    }
}
