use std::collections::{HashSet, HashMap};

const KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['X', '0', 'A'],
];

fn find_key(key: char) -> Pos {
    for row in 0..KEYPAD.len() {
        for col in 0..KEYPAD[row].len() {
            if KEYPAD[row][col] == key {
                return Pos(col as isize, row as isize);
            }
        }
    }

    panic!("Key not found");
}

const CONTROLS: [[char; 3]; 2] = [
    ['X', 'U', 'A'],
    ['L', 'D', 'R'],
];

fn find_control(key: char) -> Pos {
    for row in 0..CONTROLS.len() {
        for col in 0..CONTROLS[row].len() {
            if CONTROLS[row][col] == key {
                return Pos(col as isize, row as isize);
            }
        }
    }

    panic!("Key not found");
}

const INVALID: char = 'X';

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Pos(isize, isize);

const DIRECTIONS: [(Pos, char); 4] = [(Pos(-1, 0), 'L'), (Pos(1, 0), 'R'), (Pos(0, -1), 'U'), (Pos(0, 1), 'D')];

fn find_control_dist(from: char, to: char, pos: Pos, path: String, visited: &mut HashSet<Pos>, costs: &mut HashMap<(String, usize), usize>, depth: usize) {
    if CONTROLS[pos.1 as usize][pos.0 as usize] == INVALID {
        return;
    }
    if CONTROLS[pos.1 as usize][pos.0 as usize] == to {
        let current_cost = if depth == 0 {
            path.len() + 1
        }else {
            let mut instructions = path.chars().collect::<Vec<char>>();
            instructions.push('A');
            controls_cost(&instructions, costs, depth - 1)
        };

        let from_to_key = from.to_string() + &to.to_string();
        if let Some(cost) = costs.get(&(from_to_key.clone(), depth)) {
            if cost <= &current_cost {
                return;
            }
        }
        costs.insert((from_to_key, depth), current_cost);
        return;
    }

    visited.insert(pos);
    for (dir_pos, dir_char) in DIRECTIONS {
        let new_pos = Pos(pos.0 + dir_pos.0, pos.1 + dir_pos.1);
        if !(0 <= new_pos.1 && new_pos.1 < CONTROLS.len() as isize && 0 <= new_pos.0 && new_pos.0 < CONTROLS[0].len() as isize) {
            continue;
        }
        if visited.contains(&new_pos) {
            continue;
        }
        let new_path = path.clone() + &dir_char.to_string();
        find_control_dist(from, to, new_pos, new_path, visited, costs, depth);
    }
    visited.remove(&pos);
}

fn controls_cost(code: &[char], robot_costs: &mut HashMap<(String, usize), usize>, depth: usize) -> usize {
    let mut total_cost = 0;
    for (current, next) in std::iter::once('A').chain(code.iter().copied()).zip(code.iter().copied()) {
        let connection_id = current.to_string() + &next.to_string();

        if let Some(cost) = robot_costs.get(&(connection_id.clone(), depth)) {
            total_cost += cost;
        }
        else {
            find_control_dist(current, next, find_control(current), String::from(""), &mut HashSet::new(), robot_costs, depth);
            let cost = robot_costs.get(&(connection_id, depth)).unwrap();
            total_cost += cost;
        }
    }
    total_cost
}

fn find_keypad_dist(from: char, to: char, pos: Pos, path: String, visited: &mut HashSet<Pos>, costs: &mut HashMap<String, usize>, robot_costs: &mut HashMap<(String, usize), usize>, depth: usize) {
    if KEYPAD[pos.1 as usize][pos.0 as usize] == INVALID {
        return;
    }
    if KEYPAD[pos.1 as usize][pos.0 as usize] == to {
        let mut instructions = path.chars().collect::<Vec<char>>();
        instructions.push('A');
        let current_cost = controls_cost(&instructions, robot_costs, depth);

        let from_to_key = from.to_string() + &to.to_string();
        if let Some(cost) = costs.get(&from_to_key) {
            if cost <= &current_cost {
                return;
            }
        }
        costs.insert(from_to_key, current_cost);
        return;
    }

    visited.insert(pos);
    for (dir_pos, dir_char) in DIRECTIONS {
        let new_pos = Pos(pos.0 + dir_pos.0, pos.1 + dir_pos.1);
        if !(0 <= new_pos.1 && new_pos.1 < KEYPAD.len() as isize && 0 <= new_pos.0 && new_pos.0 < KEYPAD[0].len() as isize) {
            continue;
        }
        if visited.contains(&new_pos) {
            continue;
        }
        let new_path = path.clone() + &dir_char.to_string();
        find_keypad_dist(from, to, new_pos, new_path, visited, costs, robot_costs, depth);
    }
    visited.remove(&pos);
}

fn keypad_cost(code: &[char], robot_costs: &mut HashMap<(String, usize), usize>, depth: usize) -> usize {
    let mut costs = HashMap::new();

    let mut total_cost = 0;
    for (current, next) in std::iter::once('A').chain(code.iter().copied()).zip(code.iter().copied()) {
        find_keypad_dist(current, next, find_key(current), String::from(""), &mut HashSet::new(), &mut costs, robot_costs, depth);
        let connection_id = current.to_string() + &next.to_string();

        let cost = costs.get(&connection_id).unwrap();
        total_cost += cost;
    }

    let code_num: usize = code
        .into_iter()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();

    total_cost * code_num
}

fn keypad_costs(depth: usize) -> usize {
    let codes = aoc::to_char("input/day21.txt");
    let mut robot_costs: HashMap<(String, usize), usize> = HashMap::new();

    let mut code_total = 0;
    for code in codes {
        code_total += keypad_cost(&code, &mut robot_costs, depth);
    }

    code_total
}

#[allow(dead_code)]
pub fn part1() -> String {
    keypad_costs(1).to_string()
}

#[allow(dead_code)]
pub fn part2() -> String {
    keypad_costs(24).to_string()
}