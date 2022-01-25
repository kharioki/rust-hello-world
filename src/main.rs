fn main() {
    // let name = String::from("Tony Kharioki");

    // use match to get character at index
    // println!(
    //     "Character at index 8: {}",
    //     match name.chars().nth(8) {
    //         Some(c) => c.to_string(),
    //         None => "No character found at index 8".to_string(),
    //     }
    // );
    println!(
        "Occupation: {}",
        match get_occupation("Kiki") {
            Some(occ) => occ.to_string(),
            None => "No occupation found".to_string(),
        }
    );
}

// using Option
fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Tony" => Some("Software Developer"),
        "Karioki" => Some("Developer"),
        _ => None,
    }
}
