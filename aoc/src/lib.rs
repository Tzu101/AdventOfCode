use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

pub fn to_lines(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
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
