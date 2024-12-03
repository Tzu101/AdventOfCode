use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::time::Instant;

pub fn to_lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    lines
}
pub fn to_string(file_path: &str) -> String {
    let content: String = fs::read_to_string(file_path).unwrap();
    content
}

pub fn benchmark<F, R>(func: F) -> R
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = func();
    let duration = start.elapsed().as_secs_f64() * 1000.0;
    println!("Execution took {:.2}ms", duration);
    result
}
