/*
    match operator
    - match is used to match a value against a pattern
    - is the rust equivalent of the switch statement
*/

fn main() {
    let number = 13;

    match number {
        // match against a single value
        1 => println!("one"),
        // match against a range of values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // match against a variable number of values
        13..=19 => println!("A teen"),
        // match against a single value
        // _ is a wildcard
        _ => println!("Ain't special"),
    }

    // matching a string
    let name = "Kiki";

    match name {
        // match against a single value
        "Peter" => println!("Hi Peter"),
        // match against a range of values
        "Paul" | "Mary" | "John" => println!("Hi {}", name),
        // match against a variable number of values
        _ => println!("Hi {}!!!", name),
    }
}
