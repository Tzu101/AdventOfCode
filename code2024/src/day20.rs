use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque, HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
    score: u64,
    cheat: usize
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point {
            x,
            y,
            score: 0,
            cheat: CHEAT_STEPS,
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

const WALL: char = '#';
const EMPTY: char = '.';
const START: char = 'S';
const END: char = 'E';
const INPUT: &str = "input/day20_example.txt";

const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const CHEAT_STEPS: usize = 1;

fn point_in_bounds(point: &Point, map: &Vec<Vec<char>>) -> bool {
    0 <= point.x && point.x < map[0].len() && 0 <= point.y && point.y < map.len()
}

fn find_path(start: Point, end: Point, map: &Vec<Vec<char>>) -> u64 {
    let mut path_points = BinaryHeap::new();
    path_points.push(start);

    let mut min_scores = vec![vec![9999999999; map[0].len()]; map.len()];
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

    let mut visited = vec![vec![[false; CHEAT_STEPS + 1]; map[0].len()]; map.len()];
    let mut min_cheats = Vec::new();
    while let Some(next_point) = path_points.pop_front() {
        if next_point.score > best_non_cheat {
            continue;
        }
        if next_point.x == end.x && next_point.y == end.y {
            min_cheats.push(next_point.score);
            continue;
        }

        for direction in DIRECTIONS {
            let mut new_point = Point::from(&next_point, direction.0, direction.1);
            if !point_in_bounds(&new_point, &map) {
                continue;
            }

            let mut add_point = map[new_point.y][new_point.x] == EMPTY;
            if new_point.cheat > 0 {
                new_point.cheat -= 1;
                add_point = true;
            }

            if add_point && !visited[new_point.y][new_point.x][new_point.cheat] {
                visited[new_point.y][new_point.x][new_point.cheat] = true;
                path_points.push_back(new_point);
            }
        }
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
    2.to_string()
}