use regex::Regex;

#[allow(dead_code)]
pub fn part1() -> String {
    let memory = &aoc::to_string("input/day3.txt");
    let mut_regex: Regex = Regex::new(r"(mul\((([0-9]{1,3}),([0-9]{1,3}))\))").unwrap();
    mut_regex.captures_iter(memory).map(|caps| {
        let (_, [_, _, num1, num2]) = caps.extract();
        num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap()
    }).collect::<Vec<i32>>().iter().sum::<i32>().to_string()
}

#[allow(dead_code)]
pub fn part2() -> String {
    let memory = &aoc::to_string("input/day3.txt");
    let instruction_regex: Regex = Regex::new(r"(mul\((([0-9]{1,3}),([0-9]{1,3}))\))|(don't)|(do)").unwrap();

    let mut do_mul = true;
    instruction_regex.captures_iter(memory).map(|caps| {
        let instruction = caps.get(0).unwrap().as_str();
        match instruction {
            "do" => {
                do_mul = true;
                0
            },
            "don't" => {
                do_mul = false;
                0
            },
            _ => {
                let num1 = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let num2 = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
                if do_mul { num1 * num2 } else { 0 }
            }
        }
    }).collect::<Vec<i32>>().iter().sum::<i32>().to_string()
}