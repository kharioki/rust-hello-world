use std::collections::HashMap;

// Hashmap - is a collection of key-value pairs

fn main() {
    // create a hashmap
    let mut scores = HashMap::new();

    // Add values
    scores.insert(String::from("Rust"), 87);
    scores.insert(String::from("Dart"), 90);
    scores.insert(String::from("Solidity"), 80);
    scores.insert(String::from("Javascript"), 95);
    scores.insert(String::from("Android"), 100);

    // find length of hashmap
    println!("Total languages tested: {}", scores.len());

    // get value of a key
    let rust_score = scores.get(&String::from("Rust"));
    println!("Rust score: {}", rust_score.unwrap());

    // get value with match provider - if key is not found, return None
    match scores.get("Javascript") {
        Some(score) => println!("Javascript score: {}", score),
        None => println!("No Javascript score"),
    }

    // removing a value
    scores.remove("Android");

    // loop through hashmap
    for (key, value) in &scores {
        println!("{}: {}%", key, value);
    }

    // check for value
    println!("Did you study c++?: {}", scores.contains_key("C++"));
}
