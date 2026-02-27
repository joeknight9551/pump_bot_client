use std::fs;

pub fn read_list(filename: &str, wallet_address: &str) -> bool {
    let contents = fs::read_to_string(filename).expect("Should be able to read the file");
    contents.contains(wallet_address)
}
