/*
    vectors:
    A vector is a growable array.
*/

fn main() {
    // define a vector
    // let my_vector: Vec<i32> = Vec::new();
    let mut my_vector = vec![1, 2, 3, 4, 5];

    // accessing elements
    println!("{}", my_vector[0]);

    // pushing elements
    my_vector.push(6); // adds to the end of the vector
    println!("{}", my_vector[5]);

    // removing elements
    my_vector.pop(); // removes the last element
    println!("{}", my_vector[4]);

    my_vector.remove(0); // removes the element at the given index
    println!("{}", my_vector[0]);

    // iterating over vectors
    for i in my_vector.iter() {
        println!("{}", i);
    }
}
