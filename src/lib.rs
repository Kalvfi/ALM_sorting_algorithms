use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;
use std::str::FromStr;

pub fn read_lines<T, P>(filename: P) -> Vec<T> where T: FromStr, P: AsRef<Path> {
    let file = File::open(filename).expect("Could not open file");
    let buf = io::BufReader::new(file);

    let mut items = Vec::with_capacity(10_000_000);

    for line in buf.lines() {
        match line {
            Ok(s) => {
                match s.parse::<T>() {
                    Ok(value) => items.push(value),
                    Err(_) => {}
                }
            }
            Err(e) => println!("Error reading line: {}", e),
        }
    }

    items
}
