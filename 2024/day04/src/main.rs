use std::char::ParseCharError;
use std::{error::Error, fs::read_to_string};

const PART_1_TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

//enum ValidChar {
//    X,
//    M,
//    A,
//    S,
//}
//enum ValidCharError {
//    InvalidChar,
//}
//
//impl ValidChar {
//    fn to_char(&self) -> char {
//        match self {
//            ValidChar::X => return 'X',
//            ValidChar::M => return 'M',
//            ValidChar::A => return 'A',
//            ValidChar::S => return 'S',
//        }
//    }
//    fn from_char(c: char) -> Result<ValidChar, crate::ValidCharError> {
//        match c {
//            'X' => Ok(ValidChar::X),
//            'M' => Ok(ValidChar::M),
//            'A' => Ok(ValidChar::A),
//            'S' => Ok(ValidChar::S),
//            //_ => Err("Invalid character"),
//            _ => Err(ValidCharError::InvalidChar),
//        }
//    }
//}

struct Solution {
    solution: Vec<char>,
    reversible: bool,
}

impl Solution {
    fn matches(&self, candidate: Vec<char>) -> bool {
        if self.reversible {
            // return self.solution == candidate || self.solution.reverse().clone() == candidate;
            return self.solution.iter().eq(candidate.iter())
                || self.solution.iter().rev().eq(candidate.iter());
        } else {
            // return self.solution == candidate
            return self.solution.iter().eq(candidate.iter());
        }
    }
}

struct Directions {
    North: bool,
    South: bool,
    East: bool,
    West: bool,
    NorthEast: bool,
    SouthEast: bool,
    SouthWest: bool,
    NorthWest: bool,
}

impl Directions {
    fn new() -> Directions {
        Directions {
            North: true,
            South: true,
            East: true,
            West: true,
            NorthEast: true,
            SouthEast: true,
            SouthWest: true,
            NorthWest: true,
        }
    }
}

#[derive(Clone)]
struct Point {
    X: usize,
    Y: usize,
}

struct Match(Vec<char>);

/*
fn direction(point: Point, X_size: i32, Y_size: i32) -> Vec<Point> {
    let mut X_points: Vec<usize> = vec![];
    let mut Y_points: Vec<usize> = vec![];
    // TODO This doesn't account for zero in one direction
    for x in point.X as i32..(point.X as i32 + X_size) {
        X_points.push(x as usize);
    }

    for y in point.Y as i32..(point.Y as i32 + Y_size) {
        Y_points.push(y as usize)
    }

    X_points
        .iter()
        .zip(Y_points.iter())
        .map(|(x, y)| Point { X: *x, Y: *y })
        .collect()
}
*/

fn direction(point: &Point, magnitude: usize, x_step: i32, y_step: i32) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    for i in 1..=magnitude {
        points.push(Point {
            X: (point.X as i32 + i as i32 * x_step) as usize,
            Y: (point.Y as i32 + i as i32 * y_step) as usize,
        })
    }
    points
}

enum CrosswordError {
    NoMatch,
}

struct Crossword {
    board: Vec<Vec<char>>,
    solutions: Vec<Solution>,
}

impl Crossword {
    fn new(input: &str) -> Crossword {
        let mut board: Vec<Vec<char>> = vec![];
        for line in input.lines() {
            board.push(line.chars().collect());
        }

        Crossword {
            board,
            solutions: vec![],
        }
    }

    fn check_solution(
        &self,
        position: Point,
        solution: Solution,
    ) -> Result<Vec<Vec<Point>>, CrosswordError> {
        let mut directions: Directions = Directions::new();
        // Bounds check in cardinal directions
        if position.X - solution.solution.len() < 0 {
            directions.West = false;
        } else if position.X + solution.solution.len() > self.board[0].len() {
            directions.East = false;
        } else if position.Y - solution.solution.len() < 0 {
            directions.North = false;
        } else if position.Y + solution.solution.len() > self.board.len() {
            directions.South = false;
        }

        // AND cardinals to get the intercardinal directions
        directions.NorthEast = directions.North && directions.East;
        directions.SouthEast = directions.South && directions.East;
        directions.NorthEast = directions.North && directions.East;
        directions.SouthEast = directions.South && directions.East;

        let mut extraction_candidates: Vec<Vec<Point>> = vec![];

        // TODO
        if directions.North {
            extraction_candidates.push(direction(&position, solution.solution.len(), 0, -1))
        }
        if directions.South {
            extraction_candidates.push(direction(&position, solution.solution.len(), 0, 1))
        }
        if directions.East {
            extraction_candidates.push(direction(&position, solution.solution.len(), 1, 0))
        }
        if directions.West {
            extraction_candidates.push(direction(&position, solution.solution.len(), -1, 0))
        }
        if directions.NorthEast {
            extraction_candidates.push(direction(&position, solution.solution.len(), 1, -1))
        }
        if directions.SouthEast {
            extraction_candidates.push(direction(&position, solution.solution.len(), 1, 1))
        }
        if directions.SouthWest {
            extraction_candidates.push(direction(&position, solution.solution.len(), -1, 1))
        }
        if directions.NorthWest {
            extraction_candidates.push(direction(&position, solution.solution.len(), -1, -1))
        }

        if extraction_candidates.len() == 0 {
            return Err(CrosswordError::NoMatch);
        } else {
            return Ok(extraction_candidates);
        }
    }

    fn extract(points: Vec<Point>) -> Match {
        todo!()
    }
    fn solve(&self) -> Vec<Match> {
        // for solution in self.solutions
        //
        todo!()
    }
    fn add_solution(&mut self, solution: Solution) {
        self.solutions.push(solution);
    }
}

fn part1_main(input: &str) -> i32 {
    // Parse input to Crossword board struct
    let mut crossword: Crossword = Crossword::new(input);
    // Add Solutions to board struct
    crossword.add_solution(Solution {
        solution: "XMAS".chars().collect(),
        reversible: true,
    });
    // Solve
    todo!();
    // Count solutions
    todo!();
    0
}

fn main() {
    println!("Part 1 Test: {}", part1_main(PART_1_TEST_INPUT));
}
