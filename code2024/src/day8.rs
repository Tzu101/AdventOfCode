fn is_position_in_map(row: i32, col: i32, map: &Vec<Vec<char>>) -> bool {
    col >= 0 && col < map[0].len() as i32 && row >= 0 && row < map.len() as i32
}

#[allow(dead_code)]
pub fn part1() -> String {
    let antenna_map = aoc::to_char("input/day8_example.txt");
    let mut anti_antenna_map = antenna_map.clone();

    for row in 0..antenna_map.len() {
        for col in 0..antenna_map[0].len() {
            let source_char = antenna_map[row][col];
            if source_char == '.' {
                continue;
            }

            for target_row in 0..antenna_map.len() {
                for target_col in 0..antenna_map[0].len() {
                    let target_char = antenna_map[target_row][target_col];
                    if target_char != source_char || (row == target_row && col == target_col) {
                        continue;
                    }


                    let anti_row = 2 * (target_row as i32) - (row as i32);
                    let anti_col = 2 * (target_col as i32) - (col as i32);
                    if is_position_in_map(anti_row, anti_col, &antenna_map) {
                        anti_antenna_map[anti_row as usize][anti_col as usize] = '#';
                    }
                }
            }
        }
    }

    anti_antenna_map.iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c == '#')
        .count()
        .to_string()
}

#[allow(dead_code)]
pub fn part2() -> String {
    let antenna_map = aoc::to_char("input/day8.txt");
    let mut anti_antenna_map = antenna_map.clone();

    for row in 0..antenna_map.len() {
        for col in 0..antenna_map[0].len() {
            let source_char = antenna_map[row][col];
            if source_char == '.' {
                continue;
            }

            for target_row in 0..antenna_map.len() {
                for target_col in 0..antenna_map[0].len() {
                    let target_char = antenna_map[target_row][target_col];
                    if target_char != source_char || (row == target_row && col == target_col) {
                        continue;
                    }
                    anti_antenna_map[target_row][target_col] = '#';

                    let mut row_diff = target_row as i32 - row as i32;
                    let mut col_diff = target_col as i32 - col as i32;
                    loop {
                        let anti_row = row as i32 + row_diff;
                        let anti_col = col as i32 + col_diff;
                        if is_position_in_map(anti_row, anti_col, &antenna_map) {
                            anti_antenna_map[anti_row as usize][anti_col as usize] = '#';
                            row_diff += target_row as i32 - row as i32;
                            col_diff += target_col as i32 - col as i32;
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }
    }

    anti_antenna_map.iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c == '#')
        .count()
        .to_string()
}
