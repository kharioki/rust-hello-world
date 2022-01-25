/* modules */

mod text {
    fn greet() {
        println!("How are you doing?");
    }

    pub fn print_text() {
        println!("Hello, Kharioki!");
        greet();
    }

    // module inside a module
    pub mod text_2 {
        pub fn print_text() {
            println!("Hello, Tony Stark!");
        }
    }
}

fn main() {
    text::print_text();
    text::text_2::print_text();
}
