use std::cmp::PartialEq;

#[derive(Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn get_guard() -> (Position, Vec<Vec<i32>>) {
    let map = aoc::to_lines("input/day6.txt");
    let mut guard_map: Vec<Vec<i32>> = Vec::new();
    let mut guard_position = Position{ x: 0, y: 0};
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

    (guard_position, guard_map)
}

fn get_next_position(position: &Position, direction: &Direction) -> Position {
    let mut next_position = position.clone();
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

fn is_obstacle(position: &Position, map: &Vec<Vec<i32>>) -> bool {
    if is_position_in_map(position, map) {
        return map[position.y as usize][position.x as usize] == 1;
    }
    false
}

fn flip_guard(direction: &mut Direction) {
    match direction {
        Direction::Up => {
            *direction = Direction::Right;
        }
        Direction::Right => {
            *direction = Direction::Down;
        }
        Direction::Down => {
            *direction = Direction::Left;
        }
        Direction::Left => {
            *direction = Direction::Up;
        }
    }
}

#[allow(dead_code)]
pub fn part1() -> String {
    let mut guard_facing = Direction::Up;
    let (mut guard_position, guard_map) = get_guard();

    let mut unique_positions = vec![0u32; guard_map.len() * guard_map[0].len()];
    while is_position_in_map(&guard_position, &guard_map) {
        unique_positions[(guard_position.y * (guard_map.len() as i32) + guard_position.x) as usize] = 1;
        let mut next_position = get_next_position(&guard_position, &guard_facing);

        if is_obstacle(&next_position, &guard_map) {
            flip_guard(&mut guard_facing);
            next_position = get_next_position(&guard_position, &guard_facing);
        }
        guard_position = next_position;
    }

    unique_positions.iter().sum::<u32>().to_string()
}

fn does_guard_loop(map: &Vec<Vec<i32>>, position: &Position, direction: &Direction) -> bool {
    let mut guard_position = position.clone();
    let mut guard_facing = direction.clone();

    let mut unique_positions_up = vec![0u32; map.len() * map[0].len()];
    let mut unique_positions_down = vec![0u32; map.len() * map[0].len()];
    let mut unique_positions_left = vec![0u32; map.len() * map[0].len()];
    let mut unique_positions_right = vec![0u32; map.len() * map[0].len()];

    while is_position_in_map(&guard_position, &map) {
        let unique_index = (guard_position.y * (map[0].len() as i32) + guard_position.x) as usize;
        match guard_facing {
            Direction::Left => {
                if unique_positions_left[unique_index] == 1 {
                    return true;
                }
                unique_positions_left[unique_index] = 1;
            }
            Direction::Right => {
                if unique_positions_right[unique_index] == 1 {
                    return true;
                }
                unique_positions_right[unique_index] = 1;
            }
            Direction::Up => {
                if unique_positions_up[unique_index] == 1 {
                    return true;
                }
                unique_positions_up[unique_index] = 1;
            }
            Direction::Down => {
                if unique_positions_down[unique_index] == 1 {
                    return true;
                }
                unique_positions_down[unique_index] = 1;
            }
        }

        let next_position = get_next_position(&guard_position, &guard_facing);
        if is_obstacle(&next_position, &map) {
            flip_guard(&mut guard_facing);
        }
        else {
            guard_position = next_position;
        }
    }
    false
}

#[allow(dead_code)]
pub fn part2() -> String {
    let mut guard_facing = Direction::Up;
    let (mut guard_position, mut guard_map) = get_guard();

    let mut unique_obstacles = 0;
    for row in 0..guard_map.len() {
        for col in 0..guard_map[0].len() {
            if guard_map[row][col] == 1 {
                continue;
            }

            guard_map[row][col] = 1;
            if does_guard_loop(&guard_map, &guard_position, &guard_facing) {
                unique_obstacles += 1;
            }
            guard_map[row][col] = 0;
        }
    }
    unique_obstacles.to_string()

    /*let mut unique_obstacles = vec![0u32; guard_map.len() * guard_map[0].len()];
    while is_position_in_map(&guard_position, &guard_map) {
        let next_position = get_next_position(&guard_position, &guard_facing);

        // Simulate with extra obstacle
        if is_position_in_map(&next_position, &guard_map) && guard_map[next_position.y as usize][next_position.x as usize] != 1 {
            guard_map[next_position.y as usize][next_position.x as usize] = 1;
            if does_guard_loop(&guard_map, &guard_position, &guard_facing) {
                let unique_index = (next_position.y * (guard_map[0].len() as i32) + next_position.x) as usize;
                unique_obstacles[unique_index] = 1;
            }
            else {
            }
            guard_map[next_position.y as usize][next_position.x as usize] = 0;
        }
        else {
        }

        if is_obstacle(&next_position, &guard_map) {
            flip_guard(&mut guard_facing);
        }
        else {
            guard_position = next_position;
        }
    }

    unique_obstacles.iter().sum::<u32>().to_string()*/
}
