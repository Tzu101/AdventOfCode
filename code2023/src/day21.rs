#[allow(dead_code)]
pub fn part1() -> String {
    let mut blank_map = aoc::to_char("input/day21.txt");
    let mut current_map = blank_map.clone();

    for line in &mut blank_map {
        for tile in line {
            if *tile == 'S' {
                *tile = '.';
            }
        }
    }

    const PLOT: char = '.';
    let possible_step = 'S';

    for _ in 0..64 {
        let mut day_map = blank_map.clone();

        for row in 0..current_map.len() {
            for col in 0..current_map[row].len() {

                if current_map[row][col] != possible_step {
                    continue;
                }

                if row > 0 && current_map[row - 1][col] == PLOT {
                    day_map[row - 1][col] = possible_step;
                }
                if col > 0 && current_map[row][col - 1] == PLOT {
                    day_map[row][col - 1] = possible_step;
                }
                if row < current_map.len() - 1 && current_map[row + 1][col] == PLOT {
                    day_map[row + 1][col] = possible_step;
                }
                if col < current_map[row].len() - 1 && current_map[row][col + 1] == PLOT {
                    day_map[row][col + 1] = possible_step;
                }
            }
        }
        current_map = day_map;
    }

    let count = current_map.iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c == 'S')
        .count();

    count.to_string()
}

#[allow(dead_code)]
pub fn part2() -> String {
    2.to_string()
}