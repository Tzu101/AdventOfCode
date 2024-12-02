use std::collections::HashMap;

#[allow(dead_code)]
pub fn part1() -> String {
    let lines = aoc::to_lines("input/day1.txt");
    let lines_length = lines.len();

    let mut left_list: Vec<i32> = Vec::with_capacity(lines_length);
    let mut right_list: Vec<i32> = Vec::with_capacity(lines_length);
    for line in lines {
        let numbers = line.split_whitespace().collect::<Vec<&str>>();
        let left_num = numbers[0].parse::<i32>().expect("Couldnt parse number");
        let right_num = numbers[1].parse::<i32>().expect("Couldnt parse number");

        left_list.push(left_num);
        right_list.push(right_num);
    }
    left_list.sort_unstable();
    right_list.sort_unstable();

    let mut total_error = 0u32;
    for num in 0..lines_length {
        total_error += left_list[num].abs_diff(right_list[num]);
    }

    total_error.to_string()
}

#[allow(dead_code)]
pub fn part2() -> String {
    let lines = aoc::to_lines("input/day1.txt");
    let lines_length = lines.len();

    let mut left_list: Vec<i32> = Vec::with_capacity(lines_length);
    let mut right_map: HashMap<i32, i32> = HashMap::new();
    for line in lines {
        let numbers = line.split_whitespace().collect::<Vec<&str>>();
        let left_num = numbers[0].parse::<i32>().expect("Couldnt parse number");
        let right_num = numbers[1].parse::<i32>().expect("Couldnt parse number");

        left_list.push(left_num);
        match right_map.get(&right_num) {
            Some(num) => {
                right_map.insert(right_num, num + 1);
            },
            None => {
                right_map.insert(right_num, 1);
            },
        }
    }

    let mut total_similarity = 0;
    for left_num in left_list {
        match right_map.get(&left_num) {
            Some(right_repetition) => total_similarity += left_num * right_repetition,
            None => (),
        }
    }
    
    total_similarity.to_string()
}