// structs - structs are used to group data together to form a single unit
// structs are always prefixed with the keyword struct
// structs are used to define a data type
struct Color {
    red: u8, // u8: 0-255
    green: u8,
    blue: u8,
}

fn main() {
    let mut bg = Color {
        red: 255,
        green: 70,
        blue: 15,
    };

    bg.blue = 0;

    println!("{}, {}, {}", bg.red, bg.green, bg.blue);
}
