extern crate rand;
use rand::Rng;

fn main() {
    let num = rand::thread_rng().gen_range(1..101); // Generate a random number between 1 and 100
    println!("Random number! {}", num);

    // flip a coin
    let coin = rand::thread_rng().gen_bool(1.0 / 2.0); // Generate a random boolean
    println!("Coin flip! {}", coin);
}
