fn main() {
    print_numbers_to(30);
}

/*
  Functions
*/

fn print_numbers_to(n: u32) {
    for i in 1..n {
        if is_even(i) {
            println!("{} is even!", i);
        } else {
            println!("{} is odd!", i);
        }
    }
}

fn is_even(n: u32) -> bool {
    n % 2 == 0
}
