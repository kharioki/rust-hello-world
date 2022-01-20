fn main() {
    let mut n = 0;

    loop {
        n += 1;

        // skip even numbers
        if n % 2 == 0 {
            continue;
        }

        // stop loop at 100
        if n > 100 {
            break;
        }
        println!("n is {}", n);
    }
}
