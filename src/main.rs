use std::fs::File;
// use prelude module that will help us perform read or write operations
use std::io::prelude::*;

fn main() {
    let mut file = File::open("info.txt").expect("Can't open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Oops!!! Can't read file...");

    println!("File contents:\n\n {}", contents);
}
