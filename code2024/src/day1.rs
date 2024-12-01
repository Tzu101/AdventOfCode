pub fn part1() -> String {
    let lines = aoc::to_lines("input/day1.txt")
    .expect("Error reading file");
    let lines_length = lines.len();

    let mut left_list: Vec<i32> = Vec::with_capacity(lines_length);
    let mut right_list: Vec<i32> = Vec::with_capacity(lines_length);
    for line in lines {
        let numbers = line.split("   ").collect::<Vec<&str>>();
        let left_num = numbers[0].parse::<i32>().expect("Couldnt parse number");
        let right_num = numbers[1].parse::<i32>().expect("Couldnt parse number");

        left_list.push(left_num);
        right_list.push(right_num);
    }
    left_list.sort_unstable();
    right_list.sort_unstable();

    let mut total_error = 0i32;
    for num in 0..lines_length {
        total_error += (left_list[num] - right_list[num]).abs();
    }
    
    total_error.to_string()
}
