#![allow(dead_code)]

enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    fn is_weekday(&self) -> bool {
        match self {
            &Day::Saturday | &Day::Sunday => false,
            _ => true,
        }
    }

    // fn is_weekend(&self) -> bool {
    //     match self {
    //         Day::Saturday | Day::Sunday => true,
    //         _ => false,
    //     }
    // }
}

fn main() {
    let d = Day::Monday;
    let s = Day::Saturday;
    println!("Is d a weekday? {}", d.is_weekday());
    println!("Is s a weekday? {}", s.is_weekday());
}
