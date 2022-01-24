/*
    Arrays:
    Arrays are a collection of values of the same type.
    Arrays are zero-indexed.
*/

fn main() {
    // you can specify the type of the elements in the array and the length - not required
    // let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let numbers = [1, 2, 3, 4, 5];

    // you can also declare an array by passing the value and length
    // let numbers = [2; 400];

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
