fn simulate_fish(days: usize) -> u64 {
    let fish_types = aoc::to_lines("input/day6.txt")[0].split(",").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let mut fish = vec![0u64; 9];
    for fish_type in fish_types {
        fish[fish_type as usize] += 1;
    }

    for _ in 0..days {
        fish.rotate_left(1);
        fish[6] += fish[8];
    }

    fish.iter().sum::<u64>()
}

#[allow(dead_code)]
pub fn part1() -> String {
    simulate_fish(80).to_string()
}

#[allow(dead_code)]
pub fn part2() -> String {
    simulate_fish(256).to_string()
}