/* String methods */

fn main() {
    /* Replace */

    {
        let my_string = String::from("Hey Tony");
        println!("{}", my_string.replace("Tony", "Kharioki"));
    }

    /* Lines - split string on lines */

    {
        let my_string = String::from("Hey Tony\nI am a Rustacean");
        for line in my_string.lines() {
            println!("{}", line);
        }
    }

    /* Split */

    {
        let my_string = String::from("I+Am+A+Rustacean");
        // split and return vector
        let split_string: Vec<&str> = my_string.split("+").collect();

        for word in split_string {
            println!("{}", word);
        }
        // let mut iter = my_string.split("+");
        // println!("{}", iter.next().unwrap());
        // println!("{}", iter.next().unwrap());
        // println!("{}", iter.next().unwrap());
        // println!("{}", iter.next().unwrap());
    }

    /* Trim */

    {
        let my_string = String::from("  Hey Tony \n\r ");
        println!("{}", my_string.trim());
    }

    /* Chars */

    {
        let my_string = String::from("Hey Tony");
        // using loop
        for c in my_string.chars() {
            println!("{}", c);
        }
        // get character with match operator
        match my_string.chars().nth(6) {
            Some(c) => println!("{}", c),
            None => println!("No character found"),
        }
    }
}
