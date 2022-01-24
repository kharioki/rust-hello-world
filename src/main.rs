/*
    impl:
    impl is a keyword in Rust. It is used to add methods in a struct.
*/

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self - reference to the current instance of the struct

    fn print_description(&self) {
        println!(
            "Rectangle with width {} and height {}",
            self.width, self.height
        );
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let my_rect = Rectangle {
        width: 30,
        height: 50,
    };

    my_rect.print_description();
    println!("Is Rectangle a square? {}", my_rect.is_square());
}
