/*
    traits:
    A trait is a contract that defines a set of methods that a type must implement.
    A trait is like an interface in Java.
*/

struct Person {
    name: String,
    age: u8,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

fn main() {
    let person = Person {
        name: String::from("Tony"),
        age: 30,
    };

    println!("{}", person.to_string());
}
