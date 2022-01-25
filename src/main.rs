extern crate reqwest;

fn main() {
    // using match to handle errors
    // match reqwest::blocking::get("https://kharioki.com/") {
    //     Ok(mut response) => {
    //         // check if 200 OK
    //         if response.status() == reqwest::StatusCode::OK {
    //             // read the response body
    //             // let body = response.text().unwrap();
    //             // println!("Response: {}", body);
    //             match response.text() {
    //                 Ok(text) => println!("Response text: {}", text),
    //                 Err(e) => println!("Error: {}", e),
    //             }
    //         } else {
    //             println!("Response was not 200 OK");
    //         }
    //     }
    //     Err(e) => println!("Error: {}", e),
    // }

    let response_text = reqwest::blocking::get("https://kharioki.com/")
        .expect("Failed to get response")
        .text()
        .expect("Failed to get text");
    println!("Response text: {}", response_text);
}
