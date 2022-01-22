fn main() {
    let mut x = 5;

    // immutable references
    let xr = &x;

    {
        // mutable references
        let xm = &mut x;

        *xm += 1;

        println!("x is {}", *xm);
    }
    println!("x is {}", x);
}
