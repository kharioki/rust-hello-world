/*
    strings:
    The string type is a growable, UTF-8 encoded bit of text.
    It is stored as a sequence of bytes, and is indexed by byte.
*/

fn main() {
    // defining a string
    let mut my_string = String::from("Howdy! I'm a Tony! Nice to meet ya!");

    // printing a string
    println!("{}", my_string);
    // length
    println!("Length: {}", my_string.len());
    // Is empty?
    println!("Is empty? {}", my_string.is_empty());

    // split whitespace
    for token in my_string.split_whitespace() {
        println!("{}", token);
    }

    // check if a string contains a substring
    println!("Contains 'Tony'?: {}", my_string.contains("Tony"));

    // push to a string
    my_string.push_str(" I'm a rustacean!");
    println!("{}", my_string);
}
