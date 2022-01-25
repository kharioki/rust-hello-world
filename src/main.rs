extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn main() {
    let json_str = r#"
        {
            "name": "Kharioki Tony",
            "age": 30,
            "phones": [
                "+254 729918514",
                "+254 738754710"
            ]
        }
    "#;

    // let json: JsonValue = serde_json::from_str(json_str).unwrap();
    // println!(
    //     "Please call {} at the number {}",
    //     json["name"], json["phones"][0]
    // );

    let res = serde_json::from_str::<Person>(json_str);

    // if res.is_ok() {
    //     let json: JsonValue = res.unwrap();
    //     println!(
    //         "Please call {} at the number {} or {}",
    //         json["name"].as_str().unwrap(),
    //         json["phones"][0].as_str().unwrap(),
    //         json["phones"][1].as_str().unwrap()
    //     );
    // } else {
    //     println!("{}", res.unwrap_err());
    // }

    if res.is_ok() {
        let person: Person = res.unwrap();
        println!(
            "Please call {} at the number {} or {}",
            person.name, person.phones[0], person.phones[1]
        );
    } else {
        println!("Sorry we could not parse the json");
    }
}
