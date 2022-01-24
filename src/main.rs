fn main() {
    let mut n = 0;
    let mut m = 1;

    // basic loop
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
        // println!("n is {}", n);
    }

    // while loop
    while m <= 50 {
        // print m if divisible by 3
        if m % 3 == 0 {
            println!("m is {}", m);
        }
        m += 1;
    }

    let numbers = 30..51;

    // for loop
    for x in 1..11 {
        // prints 1 to 10 - 11 is non inclusive
        println!("x is {}", x);
    }

    // for loop with range
    for y in numbers {
        println!("y is {}", y);
    }

    let animals = vec!["dog", "cat", "bird", "fish"];

    // iterate over vector
    for a in animals.iter() {
        println!("animal name is {}", a);
    }

    // iterate and enumerate
    for (index, a) in animals.iter().enumerate() {
        println!("animal name is {} and index is {}", a, index);
    }
}
