use crate::read_lines;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Sub;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub row: i32,
    pub col: i32,
}

impl Point {
    pub fn new(row: i32, col: i32) -> Self {
        Point { row, col }
    }

    pub fn neighbors(&self) -> Vec<(Point, usize)> {
        let &Point { row, col } = self;
        vec![
            Point::new(row + 1, col),
            Point::new(row - 1, col),
            Point::new(row, col + 1),
            Point::new(row, col - 1),
        ]
        .into_iter()
        .filter(|p| p.row >= 0 && p.col >= 0)
        .map(|p| (p, 1))
        .collect_vec()
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point::new(self.row - rhs.row, self.col - rhs.col)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Tile {
    Wall,
    Path,
    Start,
    Exit,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Wall => write!(f, "#"),
            Tile::Path => write!(f, "."),
            Tile::Start => write!(f, "S"),
            Tile::Exit => write!(f, "E"),
        }
    }
}

pub fn parse_map<T>(filename: &str, tile_transformer: fn(char) -> T) -> HashMap<Point, T> {
    read_lines(filename)
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| (Point::new(row as i32, col as i32), tile_transformer(c)))
                .collect_vec()
        })
        .collect::<HashMap<Point, T>>()
}

pub fn print_map<T>(map: &HashMap<Point, T>)
where
    T: Display,
{
    let max_row = map.keys().map(|p| p.row).max().unwrap();
    let max_col = map.keys().map(|p| p.col).max().unwrap();
    for row in 0..=max_row {
        for col in 0..=max_col {
            print!("{}", map.get(&Point { row, col }).unwrap());
        }
        println!();
    }
    println!();
}

pub fn print_map_with_path<T>(map: &HashMap<Point, T>, path: &Vec<Point>)
where
    T: Display,
{
    let max_row = map.keys().map(|p| p.row).max().unwrap();
    let max_col = map.keys().map(|p| p.col).max().unwrap();
    for row in 0..=max_row {
        for col in 0..=max_col {
            if path.contains(&Point { row, col }) {
                print!("0");
            } else {
                print!("{}", map.get(&Point { row, col }).unwrap());
            }
        }
        println!();
    }
    println!();
}
