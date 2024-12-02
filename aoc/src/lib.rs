use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

pub fn to_lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
            lines
        }
        Err(err) => {
            panic!("{}", err);
        }
    }
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
