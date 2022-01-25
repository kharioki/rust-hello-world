use std::process::Command;

fn main() {
    // python print.py
    let mut cmd = Command::new("python");
    cmd.arg("print.py");

    // execute the command
    match cmd.output() {
        Ok(out) => {
            // out here is a vector of bytes. we need to convert it to a string
            println!("Output: {}", String::from_utf8_lossy(&out.stdout));
        }
        Err(e) => {
            println!("Failed to execute process: {}", e);
        }
    }
}
