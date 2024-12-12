use array2d::Array2D;
use itertools::Itertools;
use std::cmp::Ordering;
use utils::Array2DTools;

#[derive(Debug, Eq, Copy, Clone, Ord, Hash)]
struct Plot {
    row: usize,
    col: usize,
    plant: char,
}

impl Plot {
    fn new(row: usize, col: usize, plant: char) -> Plot {
        Plot { row, col, plant }
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

fn build_regions(lines: Vec<String>) -> Vec<Vec<Plot>> {
    let rows: Vec<Vec<Plot>> = lines
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
    let map = Array2D::from_rows(&rows).expect("Failed to create plots");
    let mut regions: Vec<Vec<Plot>> = Vec::new();
    let mut visited: Vec<(usize, usize)> = Vec::new();
    for ((row, col), plot) in map.enumerate_row_major() {
        if visited.iter().any(|&(r, c)| r == row && c == col) {
            continue;
        }
        let mut region: Vec<Plot> = Vec::new();
        region.push(*plot);
        visited.push((row, col));
        let mut n = map
            .neighbors(row, col)
            .into_iter()
            .filter(|&p| p.1.plant == plot.plant)
            .collect_vec();
        while n.len() > 0 {
            region = region
                .into_iter()
                .chain(n.iter().map(|&p| p.1).cloned())
                .collect_vec();
            n = n
                .into_iter()
                .flat_map(|((row, col), _)| {
                    map.neighbors(row, col)
                        .into_iter()
                        .filter(|&p| p.1.plant == plot.plant)
                        .collect_vec()
                })
                .filter(|&p| !region.contains(&p.1))
                .collect_vec();
        }
        region.sort();
        regions.push(region.into_iter().unique().collect_vec());
    }
    regions = regions.into_iter().unique().collect_vec();
    // regions.iter().for_each(|region| println!("{:?}", region));

    regions
}

fn area(region: Vec<Plot>) -> usize {
    region.len()
}

fn perimeter(region: Vec<Plot>) -> usize {
    // Go through each plot and find out how many sides of the plot are covered.
    // if all sides are covered, perimeter is 0. then sum all the plots.
    todo!();
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::read_lines;

    #[test]
    fn test_part_1() {
        let regions = build_regions(read_lines("src/sample-map.txt"));
        assert_eq!(regions.len(), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(true, true);
    }
}
