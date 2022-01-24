/*
    Arrays:
    Arrays are a collection of values of the same type.
    Arrays are zero-indexed.
*/

fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // Accessing an element
    // numbers[0]; // 1
    // numbers[4]; // 5

    // looping through an array
    for number in numbers.iter() {
        println!("{}", number);
    }

    // looping through an array with indices
    for i in 0..numbers.len() {
        println!("{}", numbers[i]);
    }
}
