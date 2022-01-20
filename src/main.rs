/*
  Enum - enums are types that can have a limited number of possible values.
*/
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/*
  constants - constants are variables that are declared in global scope and can't be changed.
  all constant names should be in uppercase and should have _ between words.
*/
const MAX_NUMBER: u8 = 20;

/*
  Tuples - tuples are a way to group together multiple values of different types into a single compound value.
*/

fn main() {
    let player_direction: Direction = Direction::Up;

    // match player_direction {
    //     Direction::Up => println!("Player is moving up"),
    //     Direction::Down => println!("Player is moving down"),
    //     Direction::Left => println!("Player is moving left"),
    //     Direction::Right => println!("Player is moving right"),
    // }

    for n in 1..MAX_NUMBER {
        println!("{}", n);
    }

    // using tuples
    let tup1 = (35, 45, 55, 65);
    let tup2 = (35, "Hello", 45, true);
    // tuples can store tuples as well
    let tup3 = (35, "Hello", (45, true));

    println!("{}", tup1.3);
    println!("{}", tup2.1);
    println!("{}", tup3.2 .1);

    // destructuring tuples
    let (x, y, z) = tup3;
    println!("{}", x);
    println!("{}", y);
    println!("{}", z.0);
}
