use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;

pub fn read_lines<P>(filename: P) -> Vec<String> where P: AsRef<Path> {
    let file = File::open(filename).expect("Could not open file");
    let buf = io::BufReader::new(file);

    let mut words = Vec::with_capacity(10_000_000);

    for line in buf.lines() {
        match line {
            Ok(word) => words.push(word),
            Err(e) => println!("Error reading line: {}", e),
        }
    }

    words
}
