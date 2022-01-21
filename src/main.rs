fn main() {
    let mut x = 10;
    {
        // shadow x in this scope
        let x = 20;
    }

    let x = "X is a string";
    println!("x is {}", x);

    let x = true;
    println!("x is {}", x);
}
