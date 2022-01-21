fn main() {
    let x = 10;

    {
        // isolated scope
        let y = 20;
        println!("x: {}, y: {}", x, y);
    }

    // y is out of scope
}
