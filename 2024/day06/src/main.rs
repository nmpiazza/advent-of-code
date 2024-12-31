use core::fmt;
use std::error::Error;

#[derive(PartialEq, PartialOrd)]
enum Tile {
    Obstruction,
    Traversible,
    Traversed,
    //GuardDirectionUp,
    //GuardDirectionRight,
    //GuardDirectionDown,
    //GuardDirectionLeft,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Obstruction => write!(f, "#"),
            Tile::Traversible => write!(f, "."),
            Tile::Traversed => write!(f, "#"),
            //Tile::GuardDirectionUp => write!(f, "^"),
            //Tile::GuardDirectionRight => write!(f, ">"),
            //Tile::GuardDirectionDown => write!(f, "v"),
            //Tile::GuardDirectionLeft => write!(f, "<"),
        }
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
struct Board {
    tiles: Vec<Vec<Tile>>,
    guard_position: Point,
    guard_direction: GuardDirection,
}

#[allow(dead_code)]
impl Board {
    fn guard_walk_forward(&mut self, p: Point) {
        self.tiles[p.y as usize][p.x as usize] = Tile::Traversed;
        self.guard_position = p;
    }

    fn guard_rotate_clockwise(&mut self) {
        match self.guard_direction {
            GuardDirection::Up => self.guard_direction = GuardDirection::Right,
            GuardDirection::Right => self.guard_direction = GuardDirection::Down,
            GuardDirection::Down => self.guard_direction = GuardDirection::Left,
            GuardDirection::Left => self.guard_direction = GuardDirection::Right,
        }
    }

    fn next_tile(&mut self) -> Option<Point> {
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

    fn guard_progress(&self) {
        // if next tile out of bounds
        todo!()
    }

    fn distinct_traversed_tiles(&self) -> usize {
        self.tiles
            .iter()
            .flatten()
            .filter(|&p| *p == Tile::Traversed)
            .count()
    }
}
fn main() {
    println!("Hello, world!");
}
