use array2d::Array2D;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use utils::{read_lines, time};

fn main() {
    let (part1, time1) = time(|| part_1("src/input.txt", 1024, 70));
    println!("Part 1: {} (took {} µs)", part1, time1.as_micros());
    let (part2, time2) = time(|| part_2("src/input.txt", 70));
    println!(
        "Part 2: {:?} (took {} milliseconds)",
        part2.unwrap(),
        time2.as_millis()
    );
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn new(row: i32, col: i32) -> Self {
        Point { row, col }
    }

    fn neighbors(&self) -> Vec<(Point, usize)> {
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

fn parse(filename: &str) -> Vec<(usize, usize)> {
    read_lines(filename)
        .iter()
        .map(|line| {
            let l = line.split(",").collect_vec();
            (
                l[0].parse::<usize>().unwrap(),
                l[1].parse::<usize>().unwrap(),
            )
        })
        .collect_vec()
}

fn part_1(filename: &str, num_bytes: usize, coordinate_max: usize) -> usize {
    let bytes = parse(filename);
    find_path(bytes, num_bytes, coordinate_max).unwrap()
}

fn part_2(filename: &str, coordinate_max: usize) -> Option<(usize, usize)> {
    let bytes = parse(filename);
    for x in 0..bytes.len() {
        let result = find_path(bytes.clone(), x, coordinate_max);
        if result.is_none() {
            return Some(bytes[x - 1]);
        }
    }
    None
}

fn find_path(bytes: Vec<(usize, usize)>, num_bytes: usize, coordinate_max: usize) -> Option<usize> {
    let mut grid: Array2D<bool> =
        Array2D::filled_with(false, coordinate_max + 1, coordinate_max + 1);
    for (col, row) in bytes[0..num_bytes].into_iter() {
        grid.set(*row, *col, true)
            .expect("Uh oh we're outside the grid");
    }
    if let Some(result) = dijkstra(
        &Point::new(0, 0),
        |p| {
            p.neighbors().into_iter().filter(|x| {
                *grid
                    .get(x.0.row as usize, x.0.col as usize)
                    .unwrap_or(&true)
                    == false
            })
        },
        |p| *p == Point::new(coordinate_max as i32, coordinate_max as i32),
    ) {
        Some(result.0.len() - 1)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn test_part_1() {
        let result = part_1("src/sample.txt", 12, 6);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("src/sample.txt", 6);
        assert_eq!(result, Some((6, 1)));
    }
}
