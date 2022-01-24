use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for arguments in args.iter() {
        println!("{}", arguments);
    }

    // you can also print with indices
    println!("{}", args[1]);
}
