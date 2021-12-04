use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const INPUT_FILE_NAME: &str = "./input.txt";

fn main() {
    let lines = lines_from_file(INPUT_FILE_NAME).expect("Could not load lines");
    let num_bits = lines[0].len();
    let median: f32 = lines.len() as f32 / 2.0;

    let mut bit_sums: Vec<u32> = vec![0; num_bits];
    for line in lines {
        let line_bytes = line.as_bytes();
        for bit_i in 0..bit_sums.len() {
            bit_sums[bit_i] += line_bytes[bit_i] as u32 - '0' as u32
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    let base: i32 = 2;
    for bit_i in 0..bit_sums.len() {
        let bit_value = base.pow(bit_sums.len() as u32 - bit_i as u32 - 1);
        if bit_sums[bit_i] as f32 >= median {
            gamma += bit_value;
        } else {
            epsilon += bit_value;
        }
    }

    println!("{}", gamma * epsilon);
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
