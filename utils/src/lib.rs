use array2d::Array2D;
use std::hash::Hash;
use std::{
    fs::read_to_string,
    time::{Duration, SystemTime},
};

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

pub fn read_file(filename: &str) -> String {
    read_to_string(filename).unwrap()
}

pub fn time<F: Fn() -> T, T>(f: F) -> (T, Duration) {
    let start = SystemTime::now();
    let result = f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    (result, duration)
}

pub trait Array2DTools<T: Eq + Hash> {
    fn neighbors(&self, row: usize, col: usize) -> Vec<((usize, usize), &T)>;
    fn orthogonal_neighbors(&self, row: usize, col: usize) -> Vec<((usize, usize), &T)>;
}

impl<T: Eq + Hash> Array2DTools<T> for Array2D<T> {
    fn neighbors(&self, row: usize, col: usize) -> Vec<((usize, usize), &T)> {
        let mut n: Vec<((usize, usize), &T)> = Vec::new();
        for x in -1i32..=1 {
            for y in -1i32..=1 {
                if x == 0 && y == 0 {
                    continue;
                } else {
                    let r = row as i32 + y;
                    let c = col as i32 + x;
                    if c >= 0 && r >= 0 {
                        if let Some(t) = self.get(r as usize, c as usize) {
                            n.push(((r as usize, c as usize), &t));
                        }
                    }
                }
            }
        }
        n
    }

    fn orthogonal_neighbors(&self, row: usize, col: usize) -> Vec<((usize, usize), &T)> {
        let mut n: Vec<((usize, usize), &T)> = Vec::new();
        for (x, y) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let r = row as i32 + y;
            let c = col as i32 + x;
            if c >= 0 && r >= 0 {
                if let Some(t) = self.get(r as usize, c as usize) {
                    n.push(((r as usize, c as usize), &t));
                }
            }
        }
        n
    }
}
