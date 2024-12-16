use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use array2d::Array2D;
use itertools::Itertools;
use utils::{read_lines, time};

fn main() {
    let (map, start, finish) = parse_map("src/input.txt");
    let (part1, time1) = time(|| cheapest_path(map.clone(), start, finish));
    println!("Part 1: {} (took {} seconds)", part1, time1.as_secs_f64());
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move(Direction, Point, i32);

impl Ord for Move {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.2.cmp(&self.2)
            .then_with(|| self.1.cmp(&other.1))
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
            if row >= 0 && col >= 0 {
                if let Some(&n) = map.get(row as usize, col as usize) {
                    if n != Cell::Wall {
                        neighbors.push(Point(row, col));
                    }
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

fn cheapest_path(map: Array2D<Cell>, start: Point, finish: Point) -> i32 {
    let mut distances: HashMap<Point, i32> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut visited: HashSet<(Point, Direction)> = HashSet::new();
    distances.insert(start, 0);
    heap.push(Move(Direction::East, start, 0));
    let mut answers: Vec<i32> = Vec::new();

    while let Some(Move {0: direction, 1: point, 2: cost}) = heap.pop() {
        if point == finish {
            answers.push(cost);
        }
        visited.insert((point, direction));
        for edge in &point.orthogonal_neighbors(&map) {
            let d = point.direction(*edge);
            let mut nc = 1;
            if d != direction {
                nc += 1000;
            }
            let next = Move(d, *edge, cost + nc);
            let mut c = i32::MAX;
            if let Some(dist) = distances.get(edge) {
                c = *dist;
            }
            if next.2 <= c {
                heap.push(next);
                distances.insert(*edge, cost);
            }
        }
    }
    println!("{:?}", answers);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let (map, start, finish) = parse_map("src/test-input.txt");
        let result = cheapest_path(map, start, finish);
        assert_eq!(result, 11048);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(true, true);
    }
}
