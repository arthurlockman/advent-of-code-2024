use array2d::Array2D;
use itertools::Itertools;
use std::cmp::Ordering;
use queues::{IsQueue, Queue};
use utils::{read_lines, time};

#[derive(Debug, Eq, Copy, Clone, Ord, Hash)]
struct Plot {
    row: usize,
    col: usize,
    plant: char,
    visited: bool,
}

impl Plot {
    fn new(row: usize, col: usize, plant: char) -> Plot {
        Plot {
            row,
            col,
            plant,
            visited: false,
        }
    }

    fn perimeter(&self, region: &Vec<Plot>) -> usize {
        4 - self.valid_orthogonal_neighbors(region).len()
    }

    fn is_neighbor(&self, other: &Plot) -> bool {
        self.plant == other.plant
            && (self.row + 1 == other.row && self.col == other.col
                || self.row == other.row && self.col + 1 == other.col
                || self.row as i32 - 1 == other.row as i32 && self.col == other.col
                || self.row == other.row && self.col as i32 - 1 == other.col as i32)
    }

    fn valid_orthogonal_neighbors(&self, region: Vec<Plot>) -> Vec<Plot> {
        let mut n: Vec<Plot> = Vec::new();
        if let Some(&r) = region
            .iter()
            .find(|p| p.plant == self.plant && p.col == self.col + 1 && p.row == self.row)
        {
            n.push(r);
        }
        if self.col as i32 - 1 >= 0 {
            if let Some(&r) = region
                .iter()
                .find(|p| p.plant == self.plant && p.col == self.col - 1 && p.row == self.row)
            {
                n.push(r);
            }
        }
        if let Some(&r) = region
            .iter()
            .find(|p| p.plant == self.plant && p.col == self.col && p.row == self.row + 1)
        {
            n.push(r);
        }
        if self.row as i32 - 1 >= 0 {
            if let Some(&r) = region
                .iter()
                .find(|p| p.plant == self.plant && p.col == self.col && p.row == self.row - 1)
            {
                n.push(r);
            }
        }
        n
    }
}

impl PartialOrd for Plot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.row == other.row {
            return Some(self.col.cmp(&other.col));
        }
        Some(self.row.cmp(&other.row))
    }
}

impl PartialEq for Plot {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col && self.plant == other.plant
    }
}

fn price(lines: Vec<String>) -> usize {
    let plots: Vec<Vec<Plot>> = lines
        .iter()
        .enumerate()
        .map(|line| {
            line.1
                .chars()
                .enumerate()
                .map(|c| Plot::new(line.0, c.0, c.1))
                .collect_vec()
        })
        .collect();
    let mut map = Array2D::from_rows(&plots).unwrap();

    for row in 0..map.row_len() {
        for col in 0..map.column_len() {
            let plot = map.get_mut(row, col).unwrap();
            let mut queue: Queue<&mut Plot> = Queue::new();
            queue.add(plot).expect("uh oh");
            while queue.size() > 0 {
                let mut p = queue.remove().unwrap();
                p.visited = true;
            }
        }
    }
    0
}

fn flood_fill<'a>(plot: &'a mut Plot, map: &'a mut Array2D<Plot>) -> Vec<&'a Plot> {
    let mut result: Vec<&Plot> = Vec::new();
    plot.visited = true;

    }
    result
}

fn main() {
    let (part1, time1) = time(|| price(read_lines("src/input.txt")));
    println!("Part 1: {} (took {} seconds)", part1, time1.as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::read_lines;

    #[test]
    fn test_part_1() {
        let price = price(read_lines("src/sample-map.txt"));
        assert_eq!(price, 1930);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(true, true);
    }
}
