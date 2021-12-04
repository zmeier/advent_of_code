use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const INPUT_FILE_NAME: &str = "./input.txt";

fn main() {
    let mut output = 0;

    let lines = lines_from_file(INPUT_FILE_NAME).expect("Could not load lines");
    for depth_index in 1..lines.len() {
        let previous_depth = &lines[depth_index - 1].parse::<i32>().unwrap();
        let depth = &lines[depth_index].parse::<i32>().unwrap();

        if previous_depth < depth {
            output += 1;
        }
    }

    println!("Total depth changes: [{}]", output);
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
