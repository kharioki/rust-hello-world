/* regex */

extern crate regex;

use regex::Regex;

fn main() {
    // let re = Regex::new(r"\w{5}").unwrap() // match a 5 letter word
    let re = Regex::new(r"\w{5}").unwrap();
    let text = "hello";

    println!("Found match? {}", re.is_match(text));

    // regex with capture
    let reg = Regex::new(r"(\w{5})").unwrap();

    let txt = "was";

    // using match to get the capture
    match reg.captures(txt) {
        Some(caps) => println!("{}", caps.get(1).unwrap().as_str()),
        None => println!("No match"),
    }
}
