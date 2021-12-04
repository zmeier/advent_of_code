use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const INPUT_FILE_NAME: &str = "./input.txt";

fn main() {
    let mut output = 0;

    let lines = lines_from_file(INPUT_FILE_NAME).expect("Could not load lines");

    let mut merged_depths = Vec::new();
    for depth_index in 0..lines.len() {
        let depth = lines[depth_index].parse::<i32>().unwrap();
        merged_depths.push(depth);
        for i in 1..3 {
            if (depth_index as i32) - (i as i32) >= 0 {
                merged_depths[depth_index - i] += depth;
            }
        }
    }

    for depth_index in 1..merged_depths.len() {
        let previous_depth = &merged_depths[depth_index - 1];
        let depth = &merged_depths[depth_index];

        if previous_depth < depth {
            output += 1;
        }
    }

    println!("Total depth changes: [{}]", output);
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
