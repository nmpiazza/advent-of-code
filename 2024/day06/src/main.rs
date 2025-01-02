use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Write},
    thread,
    time::Duration,
};

const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
enum Tile {
    Obstruction,
    Traversible,
    Traversed,
    Invalid,
    // Guard,
    GuardDirectionUp,
    GuardDirectionRight,
    GuardDirectionDown,
    GuardDirectionLeft,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Obstruction => write!(f, "#"),
            Tile::Traversible => write!(f, "."),
            Tile::Traversed => write!(f, "#"),
            Tile::Invalid => write!(f, "$"),
            Tile::GuardDirectionUp => write!(f, "^"),
            Tile::GuardDirectionRight => write!(f, ">"),
            Tile::GuardDirectionDown => write!(f, "v"),
            Tile::GuardDirectionLeft => write!(f, "<"),
        }
    }
}

struct Tiles(Vec<Tile>);

impl fmt::Display for Tiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().for_each(|c| {
            let _ = match c {
                Tile::Obstruction => write!(f, "#"),
                Tile::Traversible => write!(f, "."),
                Tile::Traversed => write!(f, "#"),
                Tile::Invalid => write!(f, "$"),
                Tile::GuardDirectionUp => write!(f, "^"),
                Tile::GuardDirectionRight => write!(f, ">"),
                Tile::GuardDirectionDown => write!(f, "v"),
                Tile::GuardDirectionLeft => write!(f, "<"),
            };
        });
        write!(f, "\n")
    }
}
enum GuardDirection {
    Up,
    Right,
    Down,
    Left,
}

impl fmt::Display for GuardDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuardDirection::Up => write!(f, "^"),
            GuardDirection::Right => write!(f, ">"),
            GuardDirection::Down => write!(f, "v"),
            GuardDirection::Left => write!(f, "<"),
        }
    }
}

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum BoardError {
    FailedParse,
}
struct Board {
    tiles: Vec<Vec<Tile>>,
    guard_position: Point,
    guard_direction: GuardDirection,
}

#[derive(Clone, Debug, PartialEq)]
enum GuardProgress {
    Complete,
    Incomplete,
}

impl Board {
    fn guard_walk_forward(&mut self, p: Point) {
        self.tiles[p.y as usize][p.x as usize] = Tile::Traversed;
        self.guard_position = p;
    }

    fn guard_rotate_clockwise(&mut self) {
        match self.guard_direction {
            GuardDirection::Up => {
                self.guard_direction = GuardDirection::Right;
                self.tiles[self.guard_position.y as usize][self.guard_position.x as usize] =
                    Tile::GuardDirectionRight
            }
            GuardDirection::Right => {
                self.guard_direction = GuardDirection::Down;
                self.tiles[self.guard_position.y as usize][self.guard_position.x as usize] =
                    Tile::GuardDirectionDown
            }
            GuardDirection::Down => {
                self.guard_direction = GuardDirection::Left;
                self.tiles[self.guard_position.y as usize][self.guard_position.x as usize] =
                    Tile::GuardDirectionRight
            }
            GuardDirection::Left => {
                self.guard_direction = GuardDirection::Right;
                self.tiles[self.guard_position.y as usize][self.guard_position.x as usize] =
                    Tile::GuardDirectionRight
            }
        }
    }

    fn next_tile(&self) -> Option<Point> {
        let mut next: Point = self.guard_position.clone();
        // match guard_direction => return math
        match self.guard_direction {
            GuardDirection::Up => next.y -= 1,
            GuardDirection::Right => next.x += 1,
            GuardDirection::Down => next.y += 1,
            GuardDirection::Left => next.x -= 1,
        }

        if (next.x < 0 || next.y < 0)
            || (next.x as usize > self.tiles[0].len() || next.y as usize > self.tiles.len())
        {
            None
        } else {
            Some(next)
        }
    }

    fn guard_progress(&mut self) -> GuardProgress {
        let next_point: Point;
        let next_tile: Tile;

        match self.next_tile() {
            None => return GuardProgress::Complete,
            Some(next) => {
                next_point = next;
                next_tile = self.tiles[next_point.y as usize][next_point.x as usize].clone()
            }
        }

        match next_tile {
            Tile::Traversible | Tile::Traversed => {
                self.guard_walk_forward(next_point);
                GuardProgress::Incomplete
            }
            Tile::Obstruction => {
                self.guard_rotate_clockwise();
                GuardProgress::Incomplete
            }
            _ => todo!(),
        }
    }

    fn distinct_traversed_tiles(&self) -> usize {
        self.tiles
            .iter()
            .flatten()
            .filter(|&p| *p == Tile::Traversed)
            .count()
    }

    fn new(input: &str) -> Result<Board, BoardError> {
        let tiles: Vec<Vec<Tile>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .into_iter()
                    .map(|c: char| match c {
                        '.' => Tile::Traversible,
                        '#' => Tile::Obstruction,
                        '^' => Tile::GuardDirectionUp,
                        '>' => Tile::GuardDirectionRight,
                        'V' => Tile::GuardDirectionDown,
                        '<' => Tile::GuardDirectionLeft,
                        _ => Tile::Invalid,
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Vec<Tile>>>();

        // dbg!(&tiles);
        if tiles.iter().flatten().any(|&x| x == Tile::Invalid) {
            return Err(BoardError::FailedParse);
        }
        Ok(Board {
            tiles: tiles,
            guard_position: Point { x: 0, y: 0 },
            guard_direction: GuardDirection::Up,
        })
    }
    // fn display_tiles(&self) {
    //     self.tiles.iter().for_each(|x| println!("{:?}", x));
    // }
    fn display_tiles(&self, iteration: usize) {
        println!("==========Tiles Iteration: {} ==========", iteration);
        for row in &self.tiles {
            for tile in row {
                print!("{}", tile);
            }
            println!();
        }
    }
}

fn part_1_main(input: &str) -> Option<usize> {
    println!("{}", input);
    let mut board: Board = Board::new(input).expect("board parsing");
    let mut guard_progress: GuardProgress = GuardProgress::Incomplete;
    let mut iteration: usize = 0;
    while guard_progress != GuardProgress::Complete {
        board.display_tiles(iteration);
        board.guard_progress();
        thread::sleep(Duration::from_secs(1));
        iteration += 1;
    }
    Some(board.distinct_traversed_tiles())
}
fn main() {
    println!("PART_1_TEST: {:?}", part_1_main(TEST_INPUT).unwrap());
}
