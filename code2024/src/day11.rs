use std::collections::HashMap;

#[allow(dead_code)]
pub fn part1() -> String {
    let lines = aoc::to_lines("input/day11.txt");
    let mut stones = lines[0].split_whitespace().map(|s: &str| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    const BLINK_NUM: u64 = 25;
    for blink in 0..BLINK_NUM {
        let mut new_stones = Vec::<u64>::new();
        println!("{blink} is {}", stones.len());
        for stone in &stones {
            if *stone == 0 {
                new_stones.push(1);
            }
            else if stone.to_string().len() % 2 == 0 {
                let stone_string = stone.to_string();
                let split_stones = stone_string.split_at(stone_string.len() / 2);
                new_stones.push(split_stones.0.parse::<u64>().unwrap());
                new_stones.push(split_stones.1.parse::<u64>().unwrap());
            }
            else {
                new_stones.push(stone * 2024);
            }
        }
        stones = new_stones;
    }

    stones.len().to_string()
}

fn stone_sum(stone: u64, blink: usize, memory: &mut HashMap<(u64, usize), u64>) -> u64 {
    if blink == 0 {
        return 1;
    }
    if memory.contains_key(&(stone, blink)) {
        return *memory.get(&(stone, blink)).unwrap();
    }

    if stone == 0 {
        let sum = stone_sum(1, blink - 1, memory);
        if !memory.contains_key(&(stone, blink)) {
            memory.insert((stone, blink), sum);
        }
        sum
    } else if stone.to_string().len() % 2 == 0 {
        let stone_string = stone.to_string();
        let split_stones = stone_string.split_at(stone_string.len() / 2);
        let sum = stone_sum(split_stones.0.parse::<u64>().unwrap(), blink - 1, memory) + stone_sum(split_stones.1.parse::<u64>().unwrap(), blink - 1, memory);
        if !memory.contains_key(&(stone, blink)) {
            memory.insert((stone, blink), sum);
        }
        sum
    } else {
        let sum = stone_sum(stone * 2024, blink - 1, memory);
        if !memory.contains_key(&(stone, blink)) {
            memory.insert((stone, blink), sum);
        }
        sum
    }
}

#[allow(dead_code)]
pub fn part2() -> String {
    let initial_stones = Vec::from([3935565, 31753, 437818, 7697, 5, 38, 0, 123]);

    const BLINK_NUM: usize = 75;
    let mut total_stones = 0;
    let mut stone_memory: HashMap<(u64, usize), u64> = HashMap::new();
    for stone in &initial_stones {
        total_stones += stone_sum(*stone, BLINK_NUM, &mut stone_memory);
    }

    total_stones.to_string()
}