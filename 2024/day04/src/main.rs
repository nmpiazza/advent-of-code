use std::thread::sleep_ms;

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

#[allow(dead_code)]
#[derive(Debug)]
struct Solution {
    solution: Vec<char>,
    reversible: bool,
}

#[allow(dead_code)]
impl Solution {
    fn matches(&self, candidate: &Vec<char>) -> bool {
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

#[allow(dead_code)]
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

#[allow(dead_code)]
impl Directions {
    fn new() -> Directions {
        Directions {
            north: false,
            south: false,
            east: false,
            west: false,
            north_east: false,
            south_east: false,
            south_west: false,
            north_west: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

//struct Match(Vec<char>);

#[allow(dead_code)]
#[derive(Debug)]
enum CrosswordError {
    NoMatch,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Crossword {
    board: Vec<Vec<char>>,
    solutions: Vec<Solution>,
}

#[allow(dead_code)]
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

    fn check_solution(&self, position: Point, solution: &Solution) -> Vec<Vec<char>> {
        //dbg!(&position);
        //dbg!(&solution);
        let mut directions: Directions = Directions::new();
        // Bounds check in cardinal directions
        if (position.x as i32) - (solution.solution.len() as i32) > 0 {
            directions.west = true;
        } else if (position.x as i32) + (solution.solution.len() as i32)
            < self.board[0].len() as i32
        {
            directions.east = true;
        } else if (position.y as i32) - (solution.solution.len() as i32) > 0 {
            directions.north = true;
        } else if (position.y as i32) + (solution.solution.len() as i32) < self.board.len() as i32 {
            directions.south = true;
        }

        // AND cardinals to get the intercardinal directions
        directions.north_east = directions.north && directions.east;
        directions.south_east = directions.south && directions.east;
        directions.north_west = directions.north && directions.west;
        directions.south_west = directions.south && directions.west;

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

        //dbg!(&extraction_candidates);
        let mut successful_matches: Vec<Vec<char>> = vec![];

        for candidate in extraction_candidates {
            // TODO implement matches logic with extract and solution.matches()
            let extracted = self.extract(candidate);
            if solution.matches(&extracted) {
                successful_matches.push(extracted)
            }
        }

        successful_matches
    }

    fn extract(&self, points: Vec<Point>) -> Vec<char> {
        //dbg!(&points);
        let mut ret_vec: Vec<char> = vec![];
        for point in points {
            ret_vec.push(self.board[point.y][point.x]);
        }
        ret_vec
    }

    fn solve(&self) -> Vec<Vec<char>> {
        let mut matches: Vec<Vec<char>> = vec![];
        for x in 0..self.board[0].len() {
            //dbg!(x);
            for y in 0..self.board.len() {
                //dbg!(y);
                for solution in &self.solutions {
                    if self.board[y][x] == solution.solution[0]
                        || self.board[y][x] == solution.solution[solution.solution.len() - 1]
                    {
                        matches.extend(self.check_solution(Point { x: x, y: y }, solution));
                    }
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

#[allow(dead_code)]
fn print_board(board: &Vec<Vec<char>>) -> String {
    //board.iter().for_each(|x| println!("{:#?}", x));
    for line in board {
        line.iter().for_each(|x| print!("{}", x));
        println!();
    }
    board
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn check_north(_x: usize, y: usize, len: usize, limit: usize) -> bool {
    y as i32 - len as i32 >= limit as i32
}

fn check_south(_x: usize, y: usize, len: usize, limit: usize) -> bool {
    y as i32 + len as i32 <= limit as i32
}

fn check_east(x: usize, _y: usize, len: usize, limit: usize) -> bool {
    x as i32 + len as i32 <= limit as i32
}

fn check_west(x: usize, _y: usize, len: usize, limit: usize) -> bool {
    x as i32 - len as i32 >= limit as i32
}

fn direction(point: &Point, magnitude: usize, x_step: i32, y_step: i32) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    //dbg!(&points);
    for i in 1..=magnitude {
        //let x_val = point.X as i32 + i as i32 * x_step;
        //let y_val = point.Y as i32 + i as i32 * y_step;
        //dbg!(&x_val);
        //dbg!(&y_val);
        points.push(Point {
            x: (point.x as i32 + i as i32 * x_step) as usize,
            y: (point.y as i32 + i as i32 * y_step) as usize,
        });
        //dbg!(&points);
    }
    points
}

fn push_if_no_duplicates(paths: &mut Vec<Vec<Point>>, candidate: Vec<Point>) {
    //dbg!(&paths);
    //dbg!(&candidate);
    let mut duplicate_found: bool = false;
    for path in paths.iter() {
        // Check forwards and reversed
        if (candidate[0] == path[0] && candidate[candidate.len() - 1] == path[path.len() - 1])
            || (candidate[0] == path[path.len() - 1] && candidate[candidate.len() - 1] == path[0])
        {
            duplicate_found = true;
            break;
        }
    }
    if !duplicate_found {
        paths.push(candidate)
    }
}

fn extract_vec_char(board: &Vec<Vec<char>>, points: &Vec<Point>) -> Vec<char> {
    print_board(board);
    sleep_ms(1);
    dbg!(&points);
    let mut ret_vec: Vec<char> = vec![];
    for point in points {
        ret_vec.push(board[point.y][point.x]);
    }
    ret_vec
}

fn simple_part_1_main(input: &str, solutions: Vec<Vec<char>>) -> usize {
    let mut board: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        board.push(line.chars().collect());
    }
    //dbg!(&board);
    //print_board(&board);
    //dbg!(&solutions);

    let mut paths: Vec<Vec<Point>> = vec![];

    for solution in solutions {
        let magnitude: usize = solution.len();

        for y in 0..board.len() {
            for x in 0..board[0].len() {
                let point: Point = Point { x, y };

                // North
                if check_north(x, y, solution.len(), 0) {
                    let points = direction(&point, magnitude, 0, -1);
                    let extracted = extract_vec_char(&board, &points);
                    if solution.iter().eq(extracted.iter())
                        || solution.iter().rev().eq(extracted.iter())
                    {
                        push_if_no_duplicates(&mut paths, points);
                    }
                }
                // South
                if check_south(x, y, solution.len(), board.len()) {
                    let points = direction(&point, magnitude, 0, 1);
                    let extracted = extract_vec_char(&board, &points);
                    if solution.iter().eq(extracted.iter())
                        || solution.iter().rev().eq(extracted.iter())
                    {
                        push_if_no_duplicates(&mut paths, points);
                    }
                }
                // East
                if check_east(x, y, solution.len(), board[0].len()) {
                    let points = direction(&point, magnitude, 1, 0);
                    let extracted = extract_vec_char(&board, &points);
                    if solution.iter().eq(extracted.iter())
                        || solution.iter().rev().eq(extracted.iter())
                    {
                        push_if_no_duplicates(&mut paths, points);
                    }
                }
                // West
                if check_west(x, y, solution.len(), 0) {
                    let points = direction(&point, magnitude, -1, 0);
                    let extracted = extract_vec_char(&board, &points);
                    if solution.iter().eq(extracted.iter())
                        || solution.iter().rev().eq(extracted.iter())
                    {
                        push_if_no_duplicates(&mut paths, points);
                    }
                }
                // NorthEast
                if check_north(x, y, solution.len(), 0)
                    && check_east(x, y, solution.len(), board[0].len())
                {
                    let points = direction(&point, magnitude, 1, -1);
                    let extracted = extract_vec_char(&board, &points);
                    if solution.iter().eq(extracted.iter())
                        || solution.iter().rev().eq(extracted.iter())
                    {
                        push_if_no_duplicates(&mut paths, points);
                    }
                }
                // SouthEast
                if check_south(x, y, solution.len(), board.len())
                    && check_east(x, y, solution.len(), board[0].len())
                {
                    let points = direction(&point, magnitude, 1, 1);
                    let extracted = extract_vec_char(&board, &points);
                    if solution.iter().eq(extracted.iter())
                        || solution.iter().rev().eq(extracted.iter())
                    {
                        push_if_no_duplicates(&mut paths, points);
                    }
                }
                // SouthWest
                if check_south(x, y, solution.len(), board.len())
                    && check_west(x, y, solution.len(), 0)
                {
                    let points = direction(&point, magnitude, -1, 1);
                    let extracted = extract_vec_char(&board, &points);
                    if solution.iter().eq(extracted.iter())
                        || solution.iter().rev().eq(extracted.iter())
                    {
                        push_if_no_duplicates(&mut paths, points);
                    }
                }
                // NorthWest
                if check_north(x, y, solution.len(), 0) && check_west(x, y, solution.len(), 0) {
                    let points = direction(&point, magnitude, -1, -1);
                    let extracted = extract_vec_char(&board, &points);
                    if solution.iter().eq(extracted.iter())
                        || solution.iter().rev().eq(extracted.iter())
                    {
                        push_if_no_duplicates(&mut paths, points);
                    }
                }
            }
        }
    }

    paths.len()
}

fn main() {
    println!(
        "Part 1 Test: {}",
        simple_part_1_main(PART_1_TEST_INPUT, vec!["XMAS".chars().collect()])
    );
}
