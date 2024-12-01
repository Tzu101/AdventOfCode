pub fn part1() -> String {
    let lines = aoc::to_lines("input/sample.txt")
    .expect("Error reading file");

    for line in lines {
        println!("{}", line);
    }
    
    String::from("Solution")
}