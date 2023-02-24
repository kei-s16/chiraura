use std::fs::File;
use std::io::prelude::*;

pub fn read_file(path: String) -> Result<String, String> {
    let mut file_body = String::new();

    let mut file_reader = File::open(path).expect("file not found");
    file_reader
        .read_to_string(&mut file_body)
        .expect("something went wrong reading the file");

    Ok(file_body)
}
