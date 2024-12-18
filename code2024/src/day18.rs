use std::cmp::Ordering;
use std::collections::BinaryHeap;

const GRID_SIZE: usize = 71;  // 7 or 71
const BYTES: usize = 1024;  // 12 or 1024
const INPUT: &str = "input/day18.txt";

#[derive(Copy, Clone, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    score: i32
}

impl Point {
    fn from(path: &Point, dx: i32, dy: i32) -> Point {
        let new_x = path.x as i32 + dx;
        let new_y = path.y as i32 + dy;

        Point {
            x: new_x as usize,
            y: new_y as usize,
            score: path.score + 1,
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

#[allow(dead_code)]
pub fn part1() -> String {
    let mut memory_space = [[0; GRID_SIZE]; GRID_SIZE];

    let falling_bytes = aoc::to_lines(INPUT);
    for byte in 0..BYTES {
        let byte_pos = falling_bytes[byte].split(",").collect::<Vec<&str>>();
        memory_space[byte_pos[1].parse::<usize>().unwrap()][byte_pos[0].parse::<usize>().unwrap()] = 1;
    }

    let end_point = Point { x: GRID_SIZE - 1, y: GRID_SIZE - 1, score: 0 };
    let mut path_points = BinaryHeap::new();
    let start_path = Point {
        x: 0,
        y: 0,
        score: 0,
    };
    path_points.push(start_path);

    let mut dist = vec![[999999; GRID_SIZE]; GRID_SIZE];

    while let Some(next_path) = path_points.pop() {
        if next_path.score > dist[next_path.y][next_path.x] { continue; }


        if next_path.x == end_point.x && next_path.y == end_point.y {
            return next_path.score.to_string();
        }

        if next_path.x > 0 && memory_space[next_path.y][next_path.x - 1] == 0 {
            let new_path = Point::from(&next_path, -1, 0);
            if dist[next_path.y][next_path.x - 1] > new_path.score {
                dist[next_path.y][next_path.x - 1] = new_path.score;
                path_points.push(new_path);
            }
        }
        if next_path.x < GRID_SIZE - 1 && memory_space[next_path.y][next_path.x + 1] == 0 {
            let new_path = Point::from(&next_path, 1, 0);
            if dist[next_path.y][next_path.x + 1] > new_path.score {
                dist[next_path.y][next_path.x + 1] = new_path.score;
                path_points.push(new_path);
            }
        }
        if next_path.y > 0 && memory_space[next_path.y - 1][next_path.x] == 0 {
            let new_path = Point::from(&next_path, 0, -1);
            if dist[next_path.y - 1][next_path.x] > new_path.score {
                dist[next_path.y - 1][next_path.x] = new_path.score;
                path_points.push(new_path);
            }
        }
        if next_path.y < GRID_SIZE - 1 && memory_space[next_path.y + 1][next_path.x] == 0 {
            let new_path = Point::from(&next_path, 0, 1);
            if dist[next_path.y + 1][next_path.x] > new_path.score {
                dist[next_path.y + 1][next_path.x] = new_path.score;
                path_points.push(new_path);
            }
        }
    }

    0.to_string()
}

fn has_path(memory_space: &[[i32; GRID_SIZE]; GRID_SIZE]) -> bool {
    let end_point = Point { x: GRID_SIZE - 1, y: GRID_SIZE - 1, score: 0 };
    let mut path_points = BinaryHeap::new();
    let start_path = Point {
        x: 0,
        y: 0,
        score: 0,
    };
    path_points.push(start_path);

    let mut dist = vec![[999999; GRID_SIZE]; GRID_SIZE];

    while let Some(next_path) = path_points.pop() {
        if next_path.score > dist[next_path.y][next_path.x] { continue; }


        if next_path.x == end_point.x && next_path.y == end_point.y {
            return true;
        }

        if next_path.x > 0 && memory_space[next_path.y][next_path.x - 1] == 0 {
            let new_path = Point::from(&next_path, -1, 0);
            if dist[next_path.y][next_path.x - 1] > new_path.score {
                dist[next_path.y][next_path.x - 1] = new_path.score;
                path_points.push(new_path);
            }
        }
        if next_path.x < GRID_SIZE - 1 && memory_space[next_path.y][next_path.x + 1] == 0 {
            let new_path = Point::from(&next_path, 1, 0);
            if dist[next_path.y][next_path.x + 1] > new_path.score {
                dist[next_path.y][next_path.x + 1] = new_path.score;
                path_points.push(new_path);
            }
        }
        if next_path.y > 0 && memory_space[next_path.y - 1][next_path.x] == 0 {
            let new_path = Point::from(&next_path, 0, -1);
            if dist[next_path.y - 1][next_path.x] > new_path.score {
                dist[next_path.y - 1][next_path.x] = new_path.score;
                path_points.push(new_path);
            }
        }
        if next_path.y < GRID_SIZE - 1 && memory_space[next_path.y + 1][next_path.x] == 0 {
            let new_path = Point::from(&next_path, 0, 1);
            if dist[next_path.y + 1][next_path.x] > new_path.score {
                dist[next_path.y + 1][next_path.x] = new_path.score;
                path_points.push(new_path);
            }
        }
    }

    false
}

#[allow(dead_code)]
pub fn part2() -> String {
    let mut memory_space = [[0; GRID_SIZE]; GRID_SIZE];

    let falling_bytes = aoc::to_lines(INPUT);
    for byte in 0..BYTES {
        let byte_pos = falling_bytes[byte].split(",").collect::<Vec<&str>>();
        memory_space[byte_pos[1].parse::<usize>().unwrap()][byte_pos[0].parse::<usize>().unwrap()] = 1;
    }

    for byte in BYTES..falling_bytes.len() {
        let byte_pos = falling_bytes[byte].split(",").collect::<Vec<&str>>();
        memory_space[byte_pos[1].parse::<usize>().unwrap()][byte_pos[0].parse::<usize>().unwrap()] = 1;

        println!("Byte: {byte}");
        if !has_path(&memory_space) {
            println!("No path found: {byte_pos:?}");
            break;
        }
    }

    0.to_string()
}