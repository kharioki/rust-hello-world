use std::fs::File;
// use prelude module that will help us perform read or write operations
use std::io::prelude::*;

fn main() {
    let mut file = File::create("output.txt").expect("Failed to create file");
    file.write_all(b"Hello there!!!\n")
        .expect("Failed to write to file");
}
