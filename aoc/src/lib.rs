use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn to_lines(file_path: &str) -> io::Result<Vec<String>> {
    // Open the file in read-only mode
    let file = File::open(file_path)?;
    // Use a BufReader for efficient line-by-line reading
    let reader = BufReader::new(file);
    // Collect lines into a Vec<String>
    reader.lines().collect()
}
