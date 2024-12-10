fn is_ascending(current_height: char, new_height: char) -> bool {
    if new_height == '.' {
        return false;
    }
    (new_height as i8 - current_height as i8) == 1
}

fn get_trailhead_unique_score(trail_map: &Vec<Vec<char>>, row: usize, col: usize, height: char, unique_ends: &mut Vec::<bool>) -> u32 {
    if trail_map[row][col] == '9' && !unique_ends[row * trail_map[0].len() + col] {
        unique_ends[row * trail_map[0].len() + col] = true;
        return 1;
    }

    let mut score = 0u32;
    if row > 0 && is_ascending(trail_map[row][col], trail_map[row - 1][col]) {
        score += get_trailhead_unique_score(trail_map, row - 1, col, height, unique_ends);
    }
    if row < trail_map.len() - 1 && is_ascending(trail_map[row][col], trail_map[row + 1][col]) {
        score += get_trailhead_unique_score(trail_map, row + 1, col, height, unique_ends);
    }
    if col > 0 && is_ascending(trail_map[row][col], trail_map[row][col - 1]) {
        score += get_trailhead_unique_score(trail_map, row, col - 1, height, unique_ends);
    }
    if col < trail_map[0].len() - 1 && is_ascending(trail_map[row][col], trail_map[row][col + 1]) {
        score += get_trailhead_unique_score(trail_map, row, col + 1, height, unique_ends);
    }

    score
}

#[allow(dead_code)]
pub fn part1() -> String {
    let trail_map = aoc::to_char("input/day10.txt");

    let mut trailhead_score = 0u32;
    for row in 0..trail_map.len() {
        for col in 0..trail_map[0].len() {
            let trailhead = trail_map[row][col];
            if trailhead != '0' {
                continue;
            }

            let mut unique_ends = vec![false; trail_map.len() * trail_map[0].len()];
            trailhead_score += get_trailhead_unique_score(&trail_map, row, col, '0', &mut unique_ends);
        }
    }

    trailhead_score.to_string()
}

fn get_trailhead_score(trail_map: &Vec<Vec<char>>, row: usize, col: usize, height: char) -> u32 {
    if trail_map[row][col] == '9' {
        return 1;
    }

    let mut score = 0u32;
    if row > 0 && is_ascending(trail_map[row][col], trail_map[row - 1][col]) {
        score += get_trailhead_score(trail_map, row - 1, col, height);
    }
    if row < trail_map.len() - 1 && is_ascending(trail_map[row][col], trail_map[row + 1][col]) {
        score += get_trailhead_score(trail_map, row + 1, col, height);
    }
    if col > 0 && is_ascending(trail_map[row][col], trail_map[row][col - 1]) {
        score += get_trailhead_score(trail_map, row, col - 1, height);
    }
    if col < trail_map[0].len() - 1 && is_ascending(trail_map[row][col], trail_map[row][col + 1]) {
        score += get_trailhead_score(trail_map, row, col + 1, height);
    }

    score
}

#[allow(dead_code)]
pub fn part2() -> String {
    let trail_map = aoc::to_char("input/day10.txt");

    let mut trailhead_score = 0u32;
    for row in 0..trail_map.len() {
        for col in 0..trail_map[0].len() {
            let trailhead = trail_map[row][col];
            if trailhead != '0' {
                continue;
            }

            trailhead_score += get_trailhead_score(&trail_map, row, col, '0');
        }
    }

    trailhead_score.to_string()
}