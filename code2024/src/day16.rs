//const WALL: char = '#';
const EMPTY: char = '.';

const MOVE_PENALTY: usize = 1;
const ROTATE_PENALTY: usize = 1000;

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    None
}
#[derive(Debug)]
struct PathPoint {
    x: usize,
    y: usize,
    score: usize,
    direction: Direction,
    path: Vec<(usize, usize)>
}

impl PartialEq for PathPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.direction == other.direction
    }
}
impl Eq for PathPoint {}

fn new_position(point: &PathPoint, direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Left => {
            (point.x - 1, point.y)
        }
        Direction::Right => {
            (point.x + 1, point.y)
        }
        Direction::Up => {
            (point.x, point.y - 1)
        }
        Direction::Down => {
            (point.x, point.y + 1)
        }
        Direction::None => {
            panic!("Invalid direction")
        }
    }
}

fn add_point(map: &Vec<Vec<char>>, path_points: &mut Vec<PathPoint>, visited_points: &mut Vec<PathPoint>, origin: &PathPoint, direction: Direction) {
    let (new_x, new_y) = new_position(&origin, &direction);

    if map[new_y][new_x] == EMPTY {
        let mut new_path = origin.path.clone();
        new_path.push((origin.x, origin.y));
        let new_point = PathPoint {
            x: new_x,
            y: new_y,
            score: if origin.direction == direction { origin.score + MOVE_PENALTY } else { origin.score + MOVE_PENALTY + ROTATE_PENALTY },
            direction,
            path: new_path
        };

        if let Some(index) = visited_points.iter().position(|c| c == &new_point) {
            if visited_points[index].score < new_point.score {
                return;
            }
        }

        if let Some(index) = path_points.iter().position(|c| c == &new_point) {
            if path_points[index].score > new_point.score {
                path_points[index] = new_point;
            }
        }
        else {
            path_points.push(new_point);
        }
    }
}

#[allow(dead_code)]
pub fn part1() -> String {
    let mut map = aoc::to_char("input/day16_example.txt");
    let mut path_points: Vec<PathPoint> = Vec::new();
    let mut end_point: PathPoint = PathPoint {x: 0, y: 0, score: 0, direction: Direction::None, path: vec![]};
    for row in 0..map.len() {
        if let Some(index) = map[row].iter().position(|c| c == &'S') {
            path_points.push(PathPoint {x: index, y: row, score: 0, direction: Direction::Right, path: vec![]});
            map[row][index] = EMPTY;
        }
        if let Some(index) = map[row].iter().position(|c| c == &'E')  {
            end_point = PathPoint {x: index, y: row, score: 0, direction: Direction::None, path: vec![]};
            map[row][index] = EMPTY;
        }
    }

    let mut visited_points: Vec<PathPoint> = Vec::new();
    while path_points.len() > 0 {
        let next_point = path_points.pop().unwrap();

        if next_point.x == end_point.x && next_point.y == end_point.y {
            return next_point.score.to_string();
        }

        match next_point.direction {
            Direction::Left => {
                add_point(&map, &mut path_points, &mut visited_points, &next_point, Direction::Left);
                add_point(&map, &mut path_points, &mut visited_points, &next_point, Direction::Up);
                add_point(&map, &mut path_points, &mut visited_points, &next_point, Direction::Down);
            }
            Direction::Right => {
                add_point(&map, &mut path_points, &mut visited_points, &next_point, Direction::Right);
                add_point(&map, &mut path_points, &mut visited_points, &next_point, Direction::Up);
                add_point(&map, &mut path_points, &mut visited_points, &next_point, Direction::Down);
            }
            Direction::Up => {
                add_point(&map, &mut path_points, &mut visited_points, &next_point, Direction::Left);
                add_point(&map, &mut path_points, &mut visited_points, &next_point, Direction::Right);
                add_point(&map, &mut path_points, &mut visited_points, &next_point, Direction::Up);
            }
            Direction::Down => {
                add_point(&map, &mut path_points, &mut visited_points, &next_point, Direction::Left);
                add_point(&map, &mut path_points, &mut visited_points, &next_point, Direction::Right);
                add_point(&map, &mut path_points, &mut visited_points, &next_point, Direction::Down);
            }
            Direction::None => {
                panic!("Invalid direction")
            }
        }

        visited_points.push(next_point);
        path_points.sort_by_key(|p| p.score);
        path_points.reverse();
    }

    (-1).to_string()
}

use std::collections::HashSet;

fn add_point_precise(map: &Vec<Vec<char>>, path_points: &mut Vec<PathPoint>, visited_points: &mut Vec<PathPoint>, origin: &PathPoint, direction: Direction) {
    if origin.direction != direction {
        let new_point = PathPoint {
            x: origin.x,
            y: origin.y,
            score: origin.score + ROTATE_PENALTY,
            direction,
            path: origin.path.clone()
        };

        if let Some(index) = visited_points.iter().position(|c| c == &new_point) {
            if visited_points[index].score < new_point.score {
                return;
            }
        }

        /*if let Some(index) = path_points.iter().position(|c| c == &new_point) {
            if path_points[index].score > new_point.score {
                path_points[index] = new_point;
            }
        }
        else {
            path_points.push(new_point);
        }*/
        path_points.push(new_point);
    }
    else {
        let (new_x, new_y) = new_position(&origin, &direction);

        if map[new_y][new_x] == EMPTY {
            let mut new_path = origin.path.clone();
            new_path.push((origin.x, origin.y));
            let new_point = PathPoint {
                x: new_x,
                y: new_y,
                score: origin.score + MOVE_PENALTY,
                direction,
                path: new_path
            };

            if let Some(index) = visited_points.iter().position(|c| c == &new_point) {
                if visited_points[index].score < new_point.score {
                    return;
                }
            }

            /*if let Some(index) = path_points.iter().position(|c| c == &new_point) {
                if path_points[index].score > new_point.score {
                    path_points[index] = new_point;
                }
            }
            else {
                path_points.push(new_point);
            }*/
            path_points.push(new_point);
        }
    }
}

#[allow(dead_code)]
pub fn part2() -> String {
    let mut map = aoc::to_char("input/day16.txt");
    let mut path_points: Vec<PathPoint> = Vec::new();
    let mut end_point: PathPoint = PathPoint {x: 0, y: 0, score: 0, direction: Direction::None, path: vec![]};
    for row in 0..map.len() {
        if let Some(index) = map[row].iter().position(|c| c == &'S') {
            path_points.push(PathPoint {x: index, y: row, score: 0, direction: Direction::Right, path: vec![]});
            map[row][index] = EMPTY;
        }
        if let Some(index) = map[row].iter().position(|c| c == &'E')  {
            end_point = PathPoint {x: index, y: row, score: 0, direction: Direction::None, path: vec![]};
            map[row][index] = EMPTY;
        }
    }

    let mut visited_points: Vec<PathPoint> = Vec::new();
    let mut watch_locations = HashSet::new();
    let mut min_score = None;
    while path_points.len() > 0 {
        let next_point = path_points.pop().unwrap();
        //println!("{:?}", next_point);

        if next_point.x == end_point.x && next_point.y == end_point.y {
            println!("Found path {} with score {}", next_point.path.len(), next_point.score);
            if min_score.is_none() {
                min_score = Some(next_point.score);
            }
            if next_point.score <= min_score.unwrap() {
                for point in &next_point.path {
                    if !watch_locations.contains(point) {
                        watch_locations.insert(point.clone());
                    }
                }
            }
            continue;
        }

        match next_point.direction {
            Direction::Left => {
                add_point_precise(&map, &mut path_points, &mut visited_points, &next_point, Direction::Left);
                add_point_precise(&map, &mut path_points, &mut visited_points, &next_point, Direction::Up);
                add_point_precise(&map, &mut path_points, &mut visited_points, &next_point, Direction::Down);
            }
            Direction::Right => {
                add_point_precise(&map, &mut path_points, &mut visited_points, &next_point, Direction::Right);
                add_point_precise(&map, &mut path_points, &mut visited_points, &next_point, Direction::Up);
                add_point_precise(&map, &mut path_points, &mut visited_points, &next_point, Direction::Down);
            }
            Direction::Up => {
                add_point_precise(&map, &mut path_points, &mut visited_points, &next_point, Direction::Left);
                add_point_precise(&map, &mut path_points, &mut visited_points, &next_point, Direction::Right);
                add_point_precise(&map, &mut path_points, &mut visited_points, &next_point, Direction::Up);
            }
            Direction::Down => {
                add_point_precise(&map, &mut path_points, &mut visited_points, &next_point, Direction::Left);
                add_point_precise(&map, &mut path_points, &mut visited_points, &next_point, Direction::Right);
                add_point_precise(&map, &mut path_points, &mut visited_points, &next_point, Direction::Down);
            }
            Direction::None => {
                panic!("Invalid direction")
            }
        }

        visited_points.push(next_point);
        path_points.sort_by_key(|p| p.score);
        path_points.reverse();
    }

    for location in &watch_locations {
        map[location.1][location.0] = 'O';
    }

    for row in map {
        for tile in row {
            print!("{} ", tile);
        }
        println!();
    }

    (watch_locations.len() + 1).to_string()
}