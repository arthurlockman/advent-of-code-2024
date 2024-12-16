use array2d::Array2D;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use utils::{read_lines, time};

fn main() {
    let (map, start, finish) = parse_map("src/input.txt");
    let (part1, time1) = time(|| cheapest_path(map.clone(), start, finish));
    println!("Part 1: {} (took {} seconds)", part1.0, time1.as_secs_f64());
    println!("Part 2: {} (computed at same time as 1)", part1.1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Wall,
    Empty,
    Start,
    Finish,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Move(Direction, Point, i32, Vec<Point>);

impl Ord for Move {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.2.cmp(&self.2).then_with(|| self.1.cmp(&other.1))
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Point(i32, i32);

impl Point {
    fn orthogonal_neighbors(&self, map: &Array2D<Cell>) -> Vec<Point> {
        let mut neighbors: Vec<Point> = Vec::new();
        let possible_neighbors = vec![Point(-1, 0), Point(1, 0), Point(0, 1), Point(0, -1)];
        for neighbor in possible_neighbors {
            let row = self.0 - neighbor.0;
            let col = self.1 - neighbor.1;

            if let Some(&n) = map.get(row as usize, col as usize) {
                if n != Cell::Wall {
                    neighbors.push(Point(row, col));
                }
            }
        }
        neighbors
    }

    fn direction(&self, other: Point) -> Direction {
        let dir = (other.0 - self.0, other.1 - self.1);
        match dir {
            (1, 0) => Direction::South,
            (-1, 0) => Direction::North,
            (0, 1) => Direction::East,
            (0, -1) => Direction::West,
            _ => unreachable!(),
        }
    }
}

fn parse_map(filename: &str) -> (Array2D<Cell>, Point, Point) {
    let mut start = Point(0, 0);
    let mut finish = Point(0, 0);
    (
        Array2D::from_rows(
            &read_lines(filename)
                .iter()
                .enumerate()
                .map(|l| {
                    l.1.chars()
                        .enumerate()
                        .filter_map(|c| match c.1 {
                            '#' => Some(Cell::Wall),
                            '.' => Some(Cell::Empty),
                            'S' => {
                                start = Point(l.0 as i32, c.0 as i32);
                                Some(Cell::Start)
                            }
                            'E' => {
                                finish = Point(l.0 as i32, c.0 as i32);
                                Some(Cell::Finish)
                            }
                            _ => None,
                        })
                        .collect_vec()
                })
                .collect_vec(),
        )
        .unwrap(),
        start,
        finish,
    )
}

fn cheapest_path(map: Array2D<Cell>, start: Point, finish: Point) -> (i32, i32) {
    let mut distances: HashMap<(Point, Direction), i32> = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push(Move(Direction::East, start, 0, vec![start]));
    let mut tiles_in_best_paths = HashSet::<Point>::new();
    let mut best = i32::MAX;

    while let Some(Move {
        0: direction,
        1: point,
        2: cost,
        3: path,
    }) = heap.pop()
    {
        if let Some(&c) = distances.get(&(point, direction)) {
            if cost > c {
                continue;
            } else {
                distances.insert((point, direction), cost);
            }
        } else {
            distances.insert((point, direction), cost);
        }
        if point == finish && cost <= best {
            best = cost;
            for tile in path {
                tiles_in_best_paths.insert(tile);
            }
            continue;
        }

        for edge in &point.orthogonal_neighbors(&map) {
            let d = point.direction(*edge);
            let mut new_cost = 1;
            if d != direction {
                new_cost += 1000;
            }
            let mut new_path = path.clone();
            new_path.push(*edge);
            let next = Move(d, *edge, cost + new_cost, new_path);
            heap.push(next);
        }
    }
    (best, tiles_in_best_paths.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let (map, start, finish) = parse_map("src/test-input.txt");
        let result = cheapest_path(map, start, finish);
        assert_eq!(result.0, 11048);
        assert_eq!(result.1, 64);
    }
}
