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

enum Instructions {
    Do,
    Dont,
    Mult(i32, i32)
}

fn memory_instructions(memory: &String) -> Vec<Instructions> {
    let mul_re = Regex::new(r"(mul\((([0-9]{1,3}),([0-9]{1,3}))\))|(don't)|(do)").unwrap();
    mul_re.captures_iter(memory).map(|caps| {
        let instruction = caps.get(0).unwrap().as_str();
        match instruction {
            "do" => Instructions::Do,
            "don't" => Instructions::Dont,
            _ => {
                let num1 = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let num2 = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
                Instructions::Mult(num1, num2)
            }
        }
    }).collect()
}

#[allow(dead_code)]
pub fn part2() -> String {
    let mut score = 0;
    let memory = &aoc::to_lines("input/day3.txt");

    let mut do_mult = true;
    for memory_line in memory {
        for instruction in memory_instructions(memory_line) {
            match instruction {
                Instructions::Do => do_mult = true,
                Instructions::Dont => do_mult = false,
                Instructions::Mult(x, y) => if do_mult { score += x * y },
            }
        }
    }

    score.to_string()
}