fn line_to_xy(line: &str) -> (i64, i64) {
    let line = line.chars()
        .filter(|c| c.is_ascii_digit() || c.is_whitespace())
        .collect::<String>();
    let line = line.split_whitespace()
        .collect::<Vec<&str>>();
    (line[0].parse::<i64>().unwrap(), line[1].parse::<i64>().unwrap())
}

fn possible_combos(a_move: i64, b_move: i64, result: i64) -> Vec<(i64, i64)> {
    let mut combos = vec![];
    let mut attempts = result / b_move;
    while attempts > 0 {
        let remainder = result - attempts * b_move;
        if remainder % a_move == 0 {
            combos.push((remainder / a_move, attempts));
        }
        attempts -= 1;
    }
    combos
}

#[allow(dead_code)]
pub fn part1() -> String {
    let lines = aoc::to_lines("input/day13.txt");

    let mut total_cost = 0;
    for l in (0..lines.len()).step_by(4) {
        let line_a = line_to_xy(&lines[l]);
        let line_b = line_to_xy(&lines[l+1]);
        let result = line_to_xy(&lines[l+2]);

        let x_combos = possible_combos(line_a.0, line_b.0, result.0);
        let y_combos = possible_combos(line_a.1, line_b.1, result.1);

        'find_combo: for combo in &x_combos {
            for match_combo in &y_combos {
                if combo == match_combo {
                    total_cost += 3 * combo.0 + combo.1;
                    break 'find_combo;
                }
            }
        }
    }

    total_cost.to_string()
}

fn calculate_combo(a_move: (i64, i64), b_move: (i64, i64), result: (i64, i64)) -> (i64, i64) {
    let b_press = (result.1 * a_move.0 - result.0 * a_move.1) / (a_move.0 * b_move.1 - a_move.1 * b_move.0);
    let a_press = (result.1 - b_press * b_move.1) / a_move.1;
    (a_press, b_press)
}

#[allow(dead_code)]
pub fn part2() -> String {
    let lines = aoc::to_lines("input/day13.txt");

    let mut total_cost = 0;
    for l in (0..lines.len()).step_by(4) {
        let line_a = line_to_xy(&lines[l]);
        let line_b = line_to_xy(&lines[l+1]);
        let result = line_to_xy(&lines[l+2]);
        let result = (result.0 + 10000000000000, result.1 + 10000000000000);

        let combo = calculate_combo(line_a, line_b, result);
        if line_a.0 * combo.0 + line_b.0 * combo.1 == result.0 &&
            line_a.1 * combo.0 + line_b.1 * combo.1 == result.1 {
            total_cost += 3 * combo.0 + combo.1;
        }
    }

    total_cost.to_string()
}