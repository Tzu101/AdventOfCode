use regex::Regex;

fn multiplication_instructions(memory: &String) -> Vec<(i32, i32)> {
    let mul_re = Regex::new(r"(mul\((([0-9]{1,3}),([0-9]{1,3}))\))").unwrap();
    mul_re.captures_iter(memory).map(|caps| {
        let (_, [_, _, num1, num2]) = caps.extract();
        (num1.parse::<i32>().unwrap(), num2.parse::<i32>().unwrap())
    }).collect()
}

#[allow(dead_code)]
pub fn part1() -> String {
    let mut score = 0;
    let memory = &aoc::to_lines("input/day3.txt");
    for memory_line in memory {
        for nums in multiplication_instructions(memory_line) {
            score += nums.0 * nums.1;
        }
    }
    score.to_string()
}

#[allow(dead_code)]
pub fn part2() -> String {
    String::from("Part 2")
}