#[derive(Clone)]
struct Position {
    x: i32,
    y: i32,
}

enum Direction {
    Left,
    Right,
    Up,
    Down
}

fn get_next_position(position: &Position, direction: &Direction) -> Position {
    let mut next_position = position.clone();
    //let mut next_position = Position { x: position.x, y: position.y };
    match direction {
        Direction::Up => {
            next_position.y -= 1;
        }
        Direction::Right => {
            next_position.x += 1;
        }
        Direction::Down => {
            next_position.y += 1;
        }
        Direction::Left => {
            next_position.x -= 1;
        }
    }
    next_position
}

fn is_position_in_map(position: &Position, map: &Vec<Vec<i32>>) -> bool {
    position.x >= 0 && position.x < map[0].len() as i32 &&
        position.y >= 0 && position.y < map.len() as i32
}

fn is_next_obstacle(position: &Position, map: &Vec<Vec<i32>>) -> bool {
    if is_position_in_map(position, map) {
        return map[position.y as usize][position.x as usize] == 1;
    }
    false
}

#[allow(dead_code)]
pub fn part1() -> String {
    let map = aoc::to_lines("input/day6.txt");
    let mut guard_map: Vec<Vec<i32>> = Vec::new();
    let mut guard_position = Position{ x: 0, y: 0};
    let mut guard_facing = Direction::Up;
    for (row, line) in map.iter().enumerate() {
        guard_map.push(line.chars().enumerate().map(|(col, tile)| {
            return if tile == '#' {
                1
            } else if tile == '.' {
                0
            } else {
                guard_position = Position { x: col as i32, y: row as i32 };
                0
            }
        }).collect());
    }

    let mut unique_positions = vec![0u32; guard_map.len() * guard_map[0].len()];
    while is_position_in_map(&guard_position, &guard_map) {
        unique_positions[(guard_position.y * (guard_map.len() as i32) + guard_position.x) as usize] = 1;
        let mut next_position = get_next_position(&guard_position, &guard_facing);

        if is_next_obstacle(&next_position, &guard_map) {
            match guard_facing {
                Direction::Up => {
                    guard_facing = Direction::Right;
                }
                Direction::Right => {
                    guard_facing = Direction::Down;
                }
                Direction::Down => {
                    guard_facing = Direction::Left;
                }
                Direction::Left => {
                    guard_facing = Direction::Up;
                }
            }
            next_position = get_next_position(&guard_position, &guard_facing);
        }
        guard_position = next_position;
    }

    unique_positions.iter().sum::<u32>().to_string()
}

#[allow(dead_code)]
pub fn part2() -> String {
    2.to_string()
}