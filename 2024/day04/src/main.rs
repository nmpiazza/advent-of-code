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

#[derive(Debug)]
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
    north: bool,
    south: bool,
    east: bool,
    west: bool,
    north_east: bool,
    south_east: bool,
    south_west: bool,
    north_west: bool,
}

impl Directions {
    fn new() -> Directions {
        Directions {
            north: true,
            south: true,
            east: true,
            west: true,
            north_east: true,
            south_east: true,
            south_west: true,
            north_west: true,
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

#[derive(Debug)]
enum CrosswordError {
    NoMatch,
}

#[derive(Debug)]
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

    fn check_solution(&self, position: Point, solution: &Solution) -> Vec<Match> {
        let mut directions: Directions = Directions::new();
        // Bounds check in cardinal directions
        if (position.X as i32) - (solution.solution.len() as i32) < 0 {
            directions.west = false;
        } else if (position.X as i32) + (solution.solution.len() as i32)
            > self.board[0].len() as i32
        {
            directions.east = false;
        } else if (position.Y as i32) - (solution.solution.len() as i32) < 0 {
            directions.north = false;
        } else if (position.Y as i32) + (solution.solution.len() as i32) > self.board.len() as i32 {
            directions.south = false;
        }

        // AND cardinals to get the intercardinal directions
        directions.north_east = directions.north && directions.east;
        directions.south_east = directions.south && directions.east;
        directions.north_east = directions.north && directions.east;
        directions.south_east = directions.south && directions.east;

        let mut extraction_candidates: Vec<Vec<Point>> = vec![];

        // TODO
        if directions.north {
            extraction_candidates.push(direction(&position, solution.solution.len(), 0, -1))
        }
        if directions.south {
            extraction_candidates.push(direction(&position, solution.solution.len(), 0, 1))
        }
        if directions.east {
            extraction_candidates.push(direction(&position, solution.solution.len(), 1, 0))
        }
        if directions.west {
            extraction_candidates.push(direction(&position, solution.solution.len(), -1, 0))
        }
        if directions.north_east {
            extraction_candidates.push(direction(&position, solution.solution.len(), 1, -1))
        }
        if directions.south_east {
            extraction_candidates.push(direction(&position, solution.solution.len(), 1, 1))
        }
        if directions.south_west {
            extraction_candidates.push(direction(&position, solution.solution.len(), -1, 1))
        }
        if directions.north_west {
            extraction_candidates.push(direction(&position, solution.solution.len(), -1, -1))
        }

        let successful_matches: Vec<Match> = vec![];

        for candidate in extraction_candidates {
            // TODO implement matches logic with extract and solution.matches()
            todo!("Implement matches logic with extract and solution.matches()");
        }
        /*
        if successful_matches.is_empty() {
            Err(CrosswordError::NoMatch)
        } else {
            Ok(successful_matches)
        }
        */
        // NOTE Changing this to just return a Vec since I'm just extending the result anyway
        successful_matches
    }

    fn extract(points: Vec<Point>) -> Match {
        todo!()
    }

    fn solve(&self) -> Vec<Match> {
        let mut matches: Vec<Match> = vec![];
        for x in 0..self.board[0].len() {
            for y in 0..self.board.len() {
                for solution in &self.solutions {
                    matches.extend(self.check_solution(Point { X: x, Y: y }, solution));
                }
            }
        }

        matches
    }

    fn add_solution(&mut self, solution: Solution) {
        self.solutions.push(solution);
    }
}

fn part1_main(input: &str) -> usize {
    // Parse input to Crossword board struct
    let mut crossword: Crossword = Crossword::new(input);
    // Add Solutions to board struct
    crossword.add_solution(Solution {
        solution: "XMAS".chars().collect(),
        reversible: true,
    });
    //dbg!(&crossword);
    // Solve
    crossword.solve().len()
    // Count solutions
    //0
}

fn main() {
    println!("Part 1 Test: {}", part1_main(PART_1_TEST_INPUT));
}
