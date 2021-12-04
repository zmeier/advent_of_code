use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?)
        .lines()
        .collect()
}
