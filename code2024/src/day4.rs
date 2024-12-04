use std::ops;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

struct Position {
    row: usize,
    col: usize,
}

struct Direction {
    row: i32,
    col: i32,
}

impl Direction {
    const UP: Direction = Direction { row: -1, col: 0 };
    const DOWN: Direction = Direction { row: 1, col: 0 };
    const LEFT: Direction = Direction { row: 0, col: -1 };
    const RIGHT: Direction = Direction { row: 0, col: 1 };

    const UP_LEFT: Direction = Direction { row: -1, col: -1 };
    const UP_RIGHT: Direction = Direction { row: -1, col: 1 };
    const DOWN_LEFT: Direction = Direction { row: 1, col: -1 };
    const DOWN_RIGHT: Direction = Direction { row: 1, col: 1 };

    const DIRECTIONS: &'static [Direction] = &[
        Direction::UP, Direction::DOWN,
        Direction::LEFT, Direction::RIGHT,
        Direction::UP_LEFT, Direction::UP_RIGHT,
        Direction::DOWN_LEFT, Direction::DOWN_RIGHT,
    ];
}

impl ops::Add<&Direction> for &Position {
    type Output = Position;
    fn add(self, _rhs: &Direction) -> Position {
        Position {
            row: ((self.row as i32) + _rhs.row) as usize,
            col: ((self.col as i32) + _rhs.col) as usize,
        }
    }
}

impl ops::Add<&Direction> for Position {
    type Output = Position;
    fn add(self, _rhs: &Direction) -> Position {
        &self + _rhs
    }
}

impl ops::Add<Direction> for &Position {
    type Output = Position;
    fn add(self, _rhs: Direction) -> Position {
        self + &_rhs
    }
}

impl ops::Mul<i32> for &Direction {
    type Output = Direction;
    fn mul(self, _rhs: i32) -> Direction {
        Direction {
            row: self.row * _rhs,
            col: self.col * _rhs,
        }
    }
}

fn find_xmas(puzzle: &Vec<Vec<char>>, position: Position, direction: &Direction, letter: usize) -> bool {
    if letter >= XMAS.len() {
        return false;
    }
    // Match
    if !(puzzle[position.row][position.col] == XMAS[letter]) {
        return false;
    }
    // Found
    if letter == XMAS.len() - 1 {
        return true;
    }

    // Recursive
    find_xmas(puzzle, position + &direction, direction, letter + 1)
}

fn position_in_puzzle(puzzle: &Vec<Vec<char>>, position: Position) -> bool {
    0 <= position.row && position.row < puzzle.len() && 0 <= position.col && position.col < puzzle[0].len()
}

fn find_all_xmas(puzzle: &Vec<Vec<char>>) -> u32 {
    let mut xmases = 0u32;

    for row in 0..puzzle.len() {
        for col in 0..puzzle[row].len() {
            for direction in Direction::DIRECTIONS {
                let first_letter = Position{row, col};
                let last_letter = &first_letter + direction * 3;
                if !position_in_puzzle(puzzle, last_letter) {
                    continue;
                }
                if find_xmas(puzzle, first_letter, direction, 0) {
                    xmases += 1;
                }
            }
        }
    }
    xmases
}

#[allow(dead_code)]
pub fn part1() -> String {
    let puzzle = &aoc::to_char("input/day4.txt");
    find_all_xmas(puzzle).to_string()
}

#[allow(dead_code)]
pub fn part2() -> String {
    //let memory = &aoc::to_string("input/day4_example.txt");
    String::from("2")
}