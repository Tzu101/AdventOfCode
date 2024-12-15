const CRATE: char = 'O';
const ROBOT: char = '@';
const EMPTY: char = '.';

fn move_object(map: &mut Vec<Vec<char>>, pos_x: usize, pos_y: usize, move_x: i32, move_y: i32) -> bool {
    let new_x = (pos_x as i32 + move_x) as usize;
    let new_y = (pos_y as i32 + move_y) as usize;

    if map[new_y][new_x] == EMPTY {
        map[new_y][new_x] = map[pos_y][pos_x];
        map[pos_y][pos_x] = EMPTY;
        return true;
    }
    else if map[new_y][new_x] == CRATE {
        if move_object(map, new_x, new_y, move_x, move_y) {
            map[new_y][new_x] = map[pos_y][pos_x];
            map[pos_y][pos_x] = EMPTY;
            return true;
        }
        else {
            return false;
        }
    }

    false
}

#[allow(dead_code)]
pub fn part1() -> String {
    let input = aoc::to_char("input/day15_example.txt");

    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut map = Vec::<Vec<char>>::new();
    let mut actions = Vec::<char>::new();
    let mut at_actions = false;
    for mut line in input {
        if line.is_empty() {
            at_actions = true;
            continue;
        }

        if at_actions {
            actions.append(&mut line);
        }
        else {
            if let Some(index) = line.iter().position(|x| x == &ROBOT) {
                pos_y = map.len();
                pos_x = index;
            }
            map.push(line);
        }
    }

    for action in actions {
        match action {
            '<' => {
                if move_object(&mut map, pos_x, pos_y, -1, 0) {
                    pos_x -= 1;
                }
            }
            '>' => {
                if move_object(&mut map, pos_x, pos_y, 1, 0) {
                    pos_x += 1;
                }
            }
            '^' => {
                if move_object(&mut map, pos_x, pos_y, 0, -1) {
                    pos_y -= 1;
                }
            }
            'v' => {
                if move_object(&mut map, pos_x, pos_y, 0, 1) {
                    pos_y += 1;
                }
            }
            _ => {
                panic!("Wrong action: {action}");
            }
        }

        /*println!("Move {action}");
        for row in &map {
            for char in row {
                print!("{}", char);
            }
            println!();
        }*/
    }

    let mut gps_sum = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == CRATE {
                gps_sum += 100 * row + col;
            }
        }
    }

    gps_sum.to_string()
}

const WALL: char = '#';
const LEFT_CRATE: char = '[';
const RIGHT_CRATE: char = ']';

fn move_crate_hor(map: &mut Vec<Vec<char>>, crate_start: usize, pos_y: usize, move_x: i32) -> bool {
    let crate_end = (crate_start as i32 + move_x) as usize;
    let move_to = (crate_end as i32 + move_x) as usize;

    if map[pos_y][move_to] == EMPTY {
        map[pos_y][move_to] = map[pos_y][crate_end];
        map[pos_y][crate_end] = map[pos_y][crate_start];
        map[pos_y][crate_start] = EMPTY;
        return true;
    }
    else if map[pos_y][move_to] == LEFT_CRATE || map[pos_y][move_to] == RIGHT_CRATE {
        if move_crate_hor(map, move_to, pos_y, move_x) {
            map[pos_y][move_to] = map[pos_y][crate_end];
            map[pos_y][crate_end] = map[pos_y][crate_start];
            map[pos_y][crate_start] = EMPTY;
            return true;
        }
        else {
            return false;
        }
    }

    false
}

fn move_robot_hor(map: &mut Vec<Vec<char>>, pos_x: usize, pos_y: usize, move_x: i32) -> bool {
    let new_x = (pos_x as i32 + move_x) as usize;

    if map[pos_y][new_x] == EMPTY {
        map[pos_y][new_x] = map[pos_y][pos_x];
        map[pos_y][pos_x] = EMPTY;
        return true;
    }
    else if map[pos_y][new_x] == LEFT_CRATE || map[pos_y][new_x] == RIGHT_CRATE {
        if move_crate_hor(map, new_x, pos_y, move_x) {
            map[pos_y][new_x] = map[pos_y][pos_x];
            map[pos_y][pos_x] = EMPTY;
            return true;
        }
        else {
            return false;
        }
    }

    false
}

fn move_crate_ver(map: &mut Vec<Vec<char>>, crate_start: usize, pos_y: usize, move_y: i32) -> bool {
    // crate_start is always [
    // crate_end is always ]
    let crate_end = (crate_start as i32 + 1) as usize;
    let new_y = (pos_y as i32 + move_y) as usize;

    let start_empty = map[new_y][crate_start] == EMPTY;
    let end_empty = map[new_y][crate_end] == EMPTY;

    if start_empty && end_empty {
        map[new_y][crate_start] = map[pos_y][crate_start];
        map[pos_y][crate_start] = EMPTY;
        map[new_y][crate_end] = map[pos_y][crate_end];
        map[pos_y][crate_end] = EMPTY;
        return true;
    }
    else if start_empty && !end_empty {
        if map[new_y][crate_end] == WALL {
            return false;
        }
        if move_crate_ver(map, crate_end, new_y, move_y) {
            map[new_y][crate_start] = map[pos_y][crate_start];
            map[pos_y][crate_start] = EMPTY;
            map[new_y][crate_end] = map[pos_y][crate_end];
            map[pos_y][crate_end] = EMPTY;
            return true;
        }
        else {
            return false;
        }
    }
    else if !start_empty && end_empty {
        if map[new_y][crate_start] == WALL {
            return false;
        }
        if move_crate_ver(map, crate_start - 1, new_y, move_y) {
            map[new_y][crate_start] = map[pos_y][crate_start];
            map[pos_y][crate_start] = EMPTY;
            map[new_y][crate_end] = map[pos_y][crate_end];
            map[pos_y][crate_end] = EMPTY;
            return true;
        }
        else {
            return false;
        }
    }
    else if !start_empty && !end_empty {
        if map[new_y][crate_start] == WALL || map[new_y][crate_end] == WALL {
            return false;
        }

        if map[pos_y][crate_start] == map[new_y][crate_start] {
            if move_crate_ver(map, crate_start, new_y, move_y) {
                map[new_y][crate_start] = map[pos_y][crate_start];
                map[pos_y][crate_start] = EMPTY;
                map[new_y][crate_end] = map[pos_y][crate_end];
                map[pos_y][crate_end] = EMPTY;
                return true;
            }
            else {
                return false;
            }
        }
        else {
            let mut temp_map = map.clone();
            if move_crate_ver(&mut temp_map, crate_start - 1, new_y, move_y) && move_crate_ver(&mut temp_map, crate_end, new_y, move_y) {
                *map = temp_map;
                map[new_y][crate_start] = map[pos_y][crate_start];
                map[pos_y][crate_start] = EMPTY;
                map[new_y][crate_end] = map[pos_y][crate_end];
                map[pos_y][crate_end] = EMPTY;
                return true;
            }
            else {
                return false;
            }
        }
    }

    false
}

fn move_robot_ver(map: &mut Vec<Vec<char>>, pos_x: usize, pos_y: usize, move_y: i32) -> bool {
    let new_y = (pos_y as i32 + move_y) as usize;

    if map[new_y][pos_x] == EMPTY {
        map[new_y][pos_x] = map[pos_y][pos_x];
        map[pos_y][pos_x] = EMPTY;
        return true;
    }
    else if map[new_y][pos_x] == LEFT_CRATE {
        if move_crate_ver(map, pos_x, new_y, move_y) {
            map[new_y][pos_x] = map[pos_y][pos_x];
            map[pos_y][pos_x] = EMPTY;
            return true;
        }
    }
    else if map[new_y][pos_x] == RIGHT_CRATE {
        if move_crate_ver(map, pos_x - 1, new_y, move_y) {
            map[new_y][pos_x] = map[pos_y][pos_x];
            map[pos_y][pos_x] = EMPTY;
            return true;
        }
    }

    false
}

#[allow(dead_code)]
pub fn part2() -> String {
    let input = aoc::to_char("input/day15.txt");

    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut map = Vec::<Vec<char>>::new();
    let mut actions = Vec::<char>::new();
    let mut at_actions = false;
    for mut line in input {
        if line.is_empty() {
            at_actions = true;
            continue;
        }

        if at_actions {
            actions.append(&mut line);
        }
        else {
            let mut map_row = Vec::<char>::new();
            for char in line {
                if char == ROBOT {
                    pos_y = map.len();
                    pos_x = map_row.len();
                    map_row.push(ROBOT);
                    map_row.push(EMPTY);
                }
                else if char == CRATE {
                    map_row.push(LEFT_CRATE);
                    map_row.push(RIGHT_CRATE);
                }
                else {
                    map_row.push(char);
                    map_row.push(char);
                }
            }
            map.push(map_row);
        }
    }

    /*println!("Initial state:");
    for row in &map {
        for char in row {
            print!("{}", char);
        }
        println!();
    }*/

    for action in actions {
        match action {
            '<' => {
                if move_robot_hor(&mut map, pos_x, pos_y, -1) {
                    pos_x -= 1;
                }
            }
            '>' => {
                if move_robot_hor(&mut map, pos_x, pos_y, 1) {
                    pos_x += 1;
                }
            }
            '^' => {
                if move_robot_ver(&mut map, pos_x, pos_y, -1) {
                    pos_y -= 1;
                }
            }
            'v' => {
                if move_robot_ver(&mut map, pos_x, pos_y, 1) {
                    pos_y += 1;
                }
            }
            _ => {
                panic!("Wrong action: {action}");
            }
        }

        /*println!("Move {action}");
        for row in &map {
            for char in row {
                print!("{}", char);
            }
            println!();
        }

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap(); // Wait for user input*/
    }

    let mut gps_sum = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == LEFT_CRATE {
                gps_sum += 100 * row + col;
            }
        }
    }

    gps_sum.to_string()
}