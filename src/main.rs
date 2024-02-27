use std::env;
use std::fs;
use std::process;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let pattern = &args[1];
    let filename: &String = &args[2];

    match fs::read_to_string(filename) {
        Ok(contents) => {
            // search the string of the file data
            for line in contents.lines() {
                if line.contains(pattern) {
                    println!("{}", line);
                }
            }
        }
        Err(e) => {
            println!("Error reading from {}: {}", filename, e);
        }
    }
}
