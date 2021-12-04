use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const INPUT_FILE_NAME: &str = "./input.txt";

fn main() {
    let mut horizontal = 0;
    let mut depth = 0;

    let lines = lines_from_file(INPUT_FILE_NAME).expect("Could not load lines");
    for line in lines {
        if line.starts_with("forward ") {
            horizontal += &line["forward ".len()..].parse::<i32>().unwrap()
        } else if line.starts_with("down ") {
            depth += &line["down ".len()..].parse::<i32>().unwrap()
        } else if line.starts_with("up ") {
            depth -= &line["up ".len()..].parse::<i32>().unwrap()
        }
    }

    println!("Total depth changes: [{}]", horizontal * depth);
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
