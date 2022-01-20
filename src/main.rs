fn main() {
    let n = 56;

    // it is unnecessary to use parentheses in the `if` keyword
    if n < 30 {
        println!("n is less than 30");
    } else if n == 50 {
        println!("This is cool!");
    } else {
        println!("n is greater than 30, n is {}", n);
    }
}
