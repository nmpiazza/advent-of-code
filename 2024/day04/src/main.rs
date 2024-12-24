use core::fmt;
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

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct BoardOffset {
    one: Point,
    two: Point,
    three: Point,
}

const NORTH_OFFSET: BoardOffset = BoardOffset {
    one: { Point { x: 0, y: -1 } },
    two: { Point { x: 0, y: -2 } },
    three: { Point { x: 0, y: -3 } },
};
const SOUTH_OFFSET: BoardOffset = BoardOffset {
    one: { Point { x: 0, y: 1 } },
    two: { Point { x: 0, y: 2 } },
    three: { Point { x: 0, y: 3 } },
};
const EAST_OFFSET: BoardOffset = BoardOffset {
    one: { Point { x: 1, y: 0 } },
    two: { Point { x: 2, y: 0 } },
    three: { Point { x: 3, y: 0 } },
};
const WEST_OFFSET: BoardOffset = BoardOffset {
    one: { Point { x: -1, y: 0 } },
    two: { Point { x: -2, y: 0 } },
    three: { Point { x: -3, y: 0 } },
};

const NORTHEAST_OFFSET: BoardOffset = BoardOffset {
    one: { Point { x: 1, y: -1 } },
    two: { Point { x: 2, y: -2 } },
    three: { Point { x: 3, y: -3 } },
};

const SOUTHEAST_OFFSET: BoardOffset = BoardOffset {
    one: { Point { x: 1, y: 1 } },
    two: { Point { x: 2, y: 2 } },
    three: { Point { x: 3, y: 3 } },
};

const SOUTHWEST_OFFSET: BoardOffset = BoardOffset {
    one: { Point { x: -1, y: 1 } },
    two: { Point { x: -2, y: 2 } },
    three: { Point { x: -3, y: 3 } },
};

const NORTHWEST_OFFSET: BoardOffset = BoardOffset {
    one: { Point { x: -1, y: -1 } },
    two: { Point { x: -2, y: -2 } },
    three: { Point { x: -3, y: -3 } },
};

#[derive(Debug)]
enum DirCheckError {
    MatchError,
}

impl std::error::Error for DirCheckError {}
impl fmt::Display for DirCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirCheckError::MatchError => write!(f, "No match"),
        }
    }
}

fn dir_check(
    board: &Vec<Vec<char>>,
    x: i32,
    y: i32,
    offset: BoardOffset,
) -> Result<Vec<Point>, DirCheckError> {
    let mut path: Vec<Point> = vec![];
    if y + offset.three.y < 0
        || x + offset.three.x < 0
        || y + offset.three.y >= board.len() as i32
        || x + offset.three.x >= board[0].len() as i32
    {
        return Err(DirCheckError::MatchError);
    }

    if board[(y + offset.one.y) as usize][(x + offset.one.x) as usize] == 'M'
        && board[(y + offset.two.y) as usize][(x + offset.two.x) as usize] == 'A'
        && board[(y + offset.three.y) as usize][(x + offset.three.x) as usize] == 'S'
    {
        //dbg!("here");
        path.push(Point { x: x, y: y });
        path.push(Point {
            x: x + offset.one.x,
            y: y + offset.one.y,
        });
        path.push(Point {
            x: x + offset.two.x,
            y: y + offset.two.y,
        });
        path.push(Point {
            x: x + offset.three.x,
            y: y + offset.three.y,
        });
    }

    Ok(path)
}

fn part_1_main(input: &str) -> usize {
    let mut matches: Vec<Vec<Point>> = vec![];
    let mut board: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        board.push(line.chars().collect());
    }

    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] == 'X' {
                for offset in [
                    NORTH_OFFSET,
                    SOUTH_OFFSET,
                    EAST_OFFSET,
                    WEST_OFFSET,
                    NORTHEAST_OFFSET,
                    SOUTHEAST_OFFSET,
                    SOUTHWEST_OFFSET,
                    NORTHWEST_OFFSET,
                ] {
                    if let Ok(path) = dir_check(&board, x as i32, y as i32, offset) {
                        if !path.is_empty() {
                            matches.push(path);
                        }
                    }
                }
            }
        }
    }

    matches.retain(|x| x.len() > 0);

    matches.len()
}

fn part_2_check(board: &Vec<Vec<char>>, x: i32, y: i32) -> Result<Vec<Point>, DirCheckError> {
    let mut path: Vec<Point> = vec![];

    if x - 1 < 0 || y - 1 < 0 || x + 1 >= board[0].len() as i32 || y + 1 >= board.len() as i32 {
        return Err(DirCheckError::MatchError);
    }

    let nw: char = board[y as usize - 1][x as usize - 1];
    let ne: char = board[y as usize - 1][x as usize + 1];
    let se: char = board[y as usize + 1][x as usize + 1];
    let sw: char = board[y as usize + 1][x as usize - 1];

    if ((nw == 'M' && se == 'S') || (nw == 'S' && se == 'M'))
        && ((ne == 'M' && sw == 'S') || (ne == 'S' && sw == 'M'))
    {
        path.push(Point { x: x, y: y })
    }

    Ok(path)
}
fn part_2_main(input: &str) -> usize {
    let mut matches: Vec<Vec<Point>> = vec![];
    let mut board: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        board.push(line.chars().collect());
    }

    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] == 'A' {
                if let Ok(path) = part_2_check(&board, x as i32, y as i32) {
                    if !path.is_empty() {
                        matches.push(path);
                    }
                }
            }
        }
    }

    matches.retain(|x| x.len() > 0);

    matches.len()
}
fn main() {
    println!("Part 1 Test: {}", part_1_main(PART_1_TEST_INPUT));

    let part_1_input: &str = &read_to_string("./src/input.txt").unwrap();
    println!("Part 1: {}", part_1_main(part_1_input));
    println!("Part 2 Test: {}", part_2_main(PART_1_TEST_INPUT));
    println!("Part 2: {}", part_2_main(part_1_input));
}
