use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const INPUT_FILE_NAME: &str = "./input.txt";

fn main() {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;

    let lines = lines_from_file(INPUT_FILE_NAME).expect("Could not load lines");
    for line in lines {
        let mut pieces = line.split_whitespace();
        let instruction = pieces.next();
        let units = pieces
            .next()
            .map(|s| s.parse::<i32>().unwrap())
            .unwrap_or(0);
        match instruction {
            Some("forward") => {
                horizontal += units;
                depth += aim * units;
            }
            Some("down") => aim += units,
            Some("up") => aim -= units,
            _ => println!("Bad instruction [{:?}]", instruction),
        }
    }

    println!("Total depth changes: [{}]", horizontal * depth);
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
