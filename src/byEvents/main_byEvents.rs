use std::io;
use std::process::Command;

pub fn main() {
    println!("Enter Eventname:");

    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let input_str = input_string.trim().to_string();

    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(format!("main{}", input_str))
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);
}
