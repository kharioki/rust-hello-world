use std::io; // Import the io module - this is a standard library module

fn main() {
    let mut input = String::new(); // Create a mutable string variable

    // println!("Enter a number: "); // Print a message to the console

    // match io::stdin().read_line(&mut input) {
    //     // Read the input into the variable
    //     Ok(_) => {
    //         let input: i32 = input.trim().parse().unwrap(); // Convert the input to an integer
    //         println!("You entered: {}", input); // Print the input to the console
    //     }
    //     Err(error) => println!("Error: {}", error), // Print the error to the console
    // }

    println!("Enter a message: ");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim();
            println!("You said: {}", input.to_uppercase());
        }
        Err(error) => println!("Error: {}", error),
    }
}
