use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque, HashSet};
use crate::day20::State::{CannotCheat, Cheating};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    CanCheat,
    Cheating(usize),
    CannotCheat
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
    score: u64,
    state: State,
    id: (usize, usize)
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point {
            x,
            y,
            score: 0,
            state: State::CanCheat,
            id: (0, 0)
        }
    }

    fn from(point: &Point, dx: i64, dy: i64) -> Point {
        let new_x = point.x as i64 + dx;
        let new_y = point.y as i64 + dy;

        Point {
            x: new_x as usize,
            y: new_y as usize,
            score: point.score + 1,
            state: State::CanCheat,
            id: point.id
        }
    }

    fn new_cheat(point: &Point) -> Point {
        Point {
            x: point.x,
            y: point.y,
            score: point.score,
            state: Cheating(CHEAT_STEPS - 1),
            id: (point.x, point.y)
        }
    }

    fn from_cheat(point: &Point, dx: i64, dy: i64) -> Point {
        let new_x = point.x as i64 + dx;
        let new_y = point.y as i64 + dy;
        if let Cheating(cheat_steps) = point.state {
            if cheat_steps == 0 {
                panic!("Cheat steps cannot be 0");
            }

            Point {
                x: new_x as usize,
                y: new_y as usize,
                score: point.score + 1,
                state: Cheating(cheat_steps - 1),
                id: point.id
            }
        }
        else {
            panic!("Invalid from cheat")
        }
    }

    fn from_no_cheat(point: &Point, dx: i64, dy: i64) -> Point {
        let new_x = point.x as i64 + dx;
        let new_y = point.y as i64 + dy;

        Point {
            x: new_x as usize,
            y: new_y as usize,
            score: point.score + 1,
            state: CannotCheat,
            id: point.id
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

    let mut visited_cheating = vec![vec![[false; CHEAT_STEPS]; map[0].len()]; map.len()];
    let mut visited_can_cheat = vec![vec![false; map[0].len()]; map.len()];
    let mut visited_cannot_cheat = vec![vec![HashSet::new(); map[0].len()]; map.len()];
    let mut unique_scores = HashSet::new();
    let mut min_cheats = Vec::new();
    while let Some(next_point) = path_points.pop_front() {
        if next_point.score >= best_non_cheat {
            continue;
        }
        if next_point.x == end.x && next_point.y == end.y {
            unique_scores.insert(next_point.score);
            min_cheats.push(next_point.score);
            println!("{:?}, {}", next_point, unique_scores.len());
            continue;
        }

        match next_point.state {
            State::CanCheat => {
                let new_cheat = Point::new_cheat(&next_point);
                path_points.push_back(new_cheat);

                for direction in DIRECTIONS {
                    let new_point = Point::from(&next_point, direction.0, direction.1);
                    if point_in_bounds(&new_point, &map) && map[new_point.y][new_point.x] == EMPTY {
                        if !visited_can_cheat[new_point.y][new_point.x] {
                            visited_can_cheat[new_point.y][new_point.x] = true;
                            path_points.push_back(new_point);
                        }
                    }
                }
            }
            State::Cheating(cheat_steps) => {
                for direction in DIRECTIONS {
                    let new_point = if cheat_steps == 0 {
                        Point::from_no_cheat(&next_point, direction.0, direction.1)
                    }
                    else {
                        Point::from_cheat(&next_point, direction.0, direction.1)
                    };

                    if point_in_bounds(&new_point, &map) && !visited_cheating[new_point.y][new_point.x][cheat_steps] {
                        visited_cheating[new_point.y][new_point.x][cheat_steps] = true;
                        path_points.push_back(new_point);
                    }
                }
            }
            State::CannotCheat => {
                for direction in DIRECTIONS {
                    let new_point = Point::from_no_cheat(&next_point, direction.0, direction.1);
                    if point_in_bounds(&new_point, &map) && map[new_point.y][new_point.x] == EMPTY {
                        if !visited_cannot_cheat[new_point.y][new_point.x].contains(&new_point.id) {
                            visited_cannot_cheat[new_point.y][new_point.x].insert(new_point.id);
                            path_points.push_back(new_point);
                        }
                    }
                }
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