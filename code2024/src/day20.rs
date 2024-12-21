use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque, HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
    score: u64,
    cheat: usize,
    cheat_start: Option<(usize, usize)>,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point {
            x,
            y,
            score: 0,
            cheat: CHEAT_STEPS,
            cheat_start: None
        }
    }

    fn from(point: &Point, dx: i64, dy: i64) -> Point {
        let new_x = point.x as i64 + dx;
        let new_y = point.y as i64 + dy;

        Point {
            x: new_x as usize,
            y: new_y as usize,
            score: point.score + 1,
            cheat: point.cheat,
            cheat_start: point.cheat_start,
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const EMPTY: char = '.';
const START: char = 'S';
const END: char = 'E';
const INPUT: &str = "input/day20_example.txt";

const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const CHEAT_STEPS: usize = 1;

fn point_in_bounds(point: &Point, map: &Vec<Vec<char>>) -> bool {
    point.x < map[0].len() && point.y < map.len()
}

fn find_path(start: Point, end: Point, map: &Vec<Vec<char>>) -> u64 {
    let mut path_points = BinaryHeap::new();
    path_points.push(start);

    let mut min_scores = vec![vec![u64::MAX; map[0].len()]; map.len()];
    while let Some(next_point) = path_points.pop() {
        if next_point.score > min_scores[next_point.y][next_point.x] {
            continue;
        }
        if next_point.x == end.x && next_point.y == end.y {
            continue;
        }

        for direction in DIRECTIONS {
            let new_point = Point::from(&next_point, direction.0, direction.1);
            if point_in_bounds(&new_point, &map) &&
                map[new_point.y][new_point.x] == EMPTY &&
                min_scores[new_point.y][new_point.x] > new_point.score {

                min_scores[new_point.y][new_point.x] = new_point.score;
                path_points.push(new_point);
            }
        }
    }

    min_scores[end.y][end.x]
}

fn find_cheats(start: Point, end: Point, map: &Vec<Vec<char>>, best_non_cheat: u64) -> Vec<u64> {
    let mut path_points = VecDeque::new();
    path_points.push_back(start);

    let mut temp_map = map.clone();

    let mut visited: HashMap<Option<(usize, usize)>, HashSet<(usize, usize)>> = HashMap::new();
    visited.insert(None, HashSet::new());
    let mut min_cheats = Vec::new();
    while let Some(next_point) = path_points.pop_front() {
        if next_point.score >= best_non_cheat {
            continue;
        }
        if next_point.x == end.x && next_point.y == end.y {
            min_cheats.push(next_point.score);
            temp_map[next_point.cheat_start.unwrap().1][next_point.cheat_start.unwrap().0] = 'X';
            //println!("Found cheat at {:?}", next_point.cheat_start);
            continue;
        }

        if next_point.x == 9 && next_point.y == 7 && next_point.cheat_start.is_none() {
            println!("{:?}", next_point);
        }

        for direction in DIRECTIONS {
            let mut new_point = Point::from(&next_point, direction.0, direction.1);
            if !point_in_bounds(&new_point, &map) {
                continue;
            }

            let mut add_point = false;
            if map[new_point.y][new_point.x] == EMPTY {
                add_point = true;
            }
            else if new_point.cheat > 0 {
                if new_point.cheat_start.is_none() {
                    new_point.cheat_start = Some((next_point.x, next_point.y));
                    visited.insert(Some((next_point.x, next_point.y)), HashSet::new());
                }
                new_point.cheat -= 1;
                add_point = true;
            }

            if add_point && !visited.get(&new_point.cheat_start).unwrap().contains(&(new_point.x, new_point.y)) {
                visited.get_mut(&new_point.cheat_start).unwrap().insert((new_point.x, new_point.y));
                path_points.push_back(new_point);
            }
        }
    }

    for row in temp_map {
        for c in row {
            print!("{} ", c);
        }
        println!();
    }

    min_cheats
}

#[allow(dead_code)]
pub fn part1() -> String {
    let mut race_map = aoc::to_char(INPUT);
    let mut start_point = None;
    let mut end_point = None;
    for row in 0..race_map.len() {
        for col in 0..race_map[row].len() {
            if race_map[row][col] == START {
                start_point = Some(Point::new(col, row));
                race_map[row][col] = EMPTY;
            }
            if race_map[row][col] == END {
                end_point = Some(Point::new(col, row));
                race_map[row][col] = EMPTY;
            }
        }
    }

    let race_map = race_map;
    let start_point = start_point.unwrap();
    let end_point = end_point.unwrap();

    let mut path_points = BinaryHeap::new();
    path_points.push(start_point);

    let best_score = find_path(start_point, end_point, &race_map);

    let min_cheats = find_cheats(start_point, end_point, &race_map, best_score);
    println!("{min_cheats:?}, {}", min_cheats.len());

    best_score.to_string()
}



#[allow(dead_code)]
pub fn part2() -> String {
    // Point (usize, usize) -> (x, y)
    let mut race_map = aoc::to_char("input.txt");
    let mut start_point = None;
    let mut end_point = None;
    for row in 0..race_map.len() {
        for col in 0..race_map[row].len() {
            if race_map[row][col] == 'S' {
                start_point = Some((col, row));
                race_map[row][col] = '.';
            }
            if race_map[row][col] == 'E' {
                end_point = Some((col, row));
                race_map[row][col] = '.';
            }
        }
    }

    let race_map = race_map;
    let mut score_map = vec![vec![u64::MAX; race_map[0].len()]; race_map.len()];
    let start_point = start_point.unwrap();

    score_map[start_point.1][start_point.0] = 0;

    let mut path_points = VecDeque::new();
    path_points.push_back(start_point);
    while let Some(next_point) = path_points.pop_front() {
        if next_point.0 > 0 && race_map[next_point.1][next_point.0 - 1] != '#' && score_map[next_point.1][next_point.0 - 1] > score_map[next_point.1][next_point.0] {
            score_map[next_point.1][next_point.0 - 1] = score_map[next_point.1][next_point.0] + 1;
            path_points.push_back((next_point.0 - 1, next_point.1));
        }
        if next_point.0 < race_map[0].len() - 1 && race_map[next_point.1][next_point.0 + 1] != '#' && score_map[next_point.1][next_point.0 + 1] > score_map[next_point.1][next_point.0] {
            score_map[next_point.1][next_point.0 + 1] = score_map[next_point.1][next_point.0] + 1;
            path_points.push_back((next_point.0 + 1, next_point.1));
        }
        if next_point.1 > 0 && race_map[next_point.1 - 1][next_point.0] != '#' && score_map[next_point.1 - 1][next_point.0] > score_map[next_point.1][next_point.0] {
            score_map[next_point.1 - 1][next_point.0] = score_map[next_point.1][next_point.0] + 1;
            path_points.push_back((next_point.0, next_point.1 - 1));
        }
        if next_point.1 < race_map.len() - 1 && race_map[next_point.1 + 1][next_point.0] != '#' && score_map[next_point.1 + 1][next_point.0] > score_map[next_point.1][next_point.0] {
            score_map[next_point.1 + 1][next_point.0] = score_map[next_point.1][next_point.0] + 1;
            path_points.push_back((next_point.0, next_point.1 + 1));
        }
    }

    let mut total_cheats = 0;

    for row in 0..score_map.len() {
        for col in 0..score_map[row].len() {
            let score = score_map[row][col];
            if score == WALL {
                continue;
            }

            let pos = (col, row);
            let mut rep = HashSet::new();
            let mut goal = HashSet::new();
            total_cheats += rec_cheat(0, pos, &mut rep, &mut goal, &score_map, score, Cheat::Can);
        }
    }

    total_cheats.to_string()
}

#[derive(Clone, Copy, Debug)]
enum Cheat {
    Can,
    Will,
    Is,
    Cannot
}

const SCORE_THRESHOLD: u64 = 49;
const CHEAT_THRESHOLD: u64 = 20;

const WALL: u64 = u64::MAX;

// I broke something to this no longer works. Example returns 721 instead of 285 at 49, 20
fn rec_cheat(cheat: u64, pos: (usize, usize), rep: &mut HashSet<(usize, usize)>, goal: &mut HashSet<(usize, usize)>, scores: &Vec<Vec<u64>>, origin: u64, mut cheating: Cheat) -> u64 {
    let mut total_score = 0;
    let mut add_tile = false;
    let mut segment_fault = false;
    let mut add_cheat_tile = false;
    match cheating {
        Cheat::Can => {
            if scores[pos.1][pos.0] == WALL {
                cheating = Cheat::Is;
            }
        }
        Cheat::Will => {
            if scores[pos.1][pos.0] == WALL {
                cheating = Cheat::Is;
            }
        }
        Cheat::Is => {
            if scores[pos.1][pos.0] != WALL {
                cheating = Cheat::Cannot;
                add_tile = true;
            }
        }
        Cheat::Cannot => {
            if scores[pos.1][pos.0] == WALL {
                segment_fault = true;
            }
            else {
                add_tile = true;
            }
        }
    }

    if add_tile {
        let current_score = scores[pos.1][pos.0];

        let improves_score = current_score > origin + cheat;
        if improves_score {
            let unique_goal = !goal.contains(&pos);

            let saved_score = current_score - origin - cheat;
            let within_threshold = saved_score >= SCORE_THRESHOLD;

            if unique_goal && improves_score && within_threshold {
                goal.insert(pos);
                total_score += 1;
            }
        }
    }

    if cheat == CHEAT_THRESHOLD || segment_fault {
        return total_score;
    }

    rep.insert(pos);

    if add_cheat_tile {
        if pos.0 > 0 && !rep.contains(&(pos.0 - 1, pos.1)) {
            total_score += rec_cheat(cheat + 1, (pos.0 - 1, pos.1), rep, goal, scores, origin, Cheat::Will);
        }
        if pos.0 < scores[0].len() - 1 && !rep.contains(&(pos.0 + 1, pos.1)) {
            total_score += rec_cheat(cheat + 1, (pos.0 + 1, pos.1), rep, goal, scores, origin, Cheat::Will);
        }
        if pos.1 > 0 && !rep.contains(&(pos.0, pos.1 - 1)) {
            total_score += rec_cheat(cheat + 1, (pos.0, pos.1 - 1), rep, goal, scores, origin, Cheat::Will);
        }
        if pos.1 < scores.len() - 1 && !rep.contains(&(pos.0, pos.1 + 1)) {
            total_score += rec_cheat(cheat + 1, (pos.0, pos.1 + 1), rep, goal, scores, origin, Cheat::Will);
        }
    }

    if pos.0 > 0 && !rep.contains(&(pos.0 - 1, pos.1)) {
        total_score += rec_cheat(cheat + 1, (pos.0 - 1, pos.1), rep, goal, scores, origin, cheating);
    }
    if pos.0 < scores[0].len() - 1 && !rep.contains(&(pos.0 + 1, pos.1)) {
        total_score += rec_cheat(cheat + 1, (pos.0 + 1, pos.1), rep, goal, scores, origin, cheating);
    }
    if pos.1 > 0 && !rep.contains(&(pos.0, pos.1 - 1)) {
        total_score += rec_cheat(cheat + 1, (pos.0, pos.1 - 1), rep, goal, scores, origin, cheating);
    }
    if pos.1 < scores.len() - 1 && !rep.contains(&(pos.0, pos.1 + 1)) {
        total_score += rec_cheat(cheat + 1, (pos.0, pos.1 + 1), rep, goal, scores, origin, cheating);
    }

    rep.remove(&pos);

    total_score
}
