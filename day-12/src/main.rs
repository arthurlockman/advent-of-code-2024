use itertools::Itertools;
use queues::{IsQueue, Queue};
use utils::{read_lines, time};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Side {
    direction: Direction,
    index: usize,
}

impl Side {
    fn new(direction: Direction, index: usize) -> Self {
        Self { direction, index }
    }
}

#[derive(Debug, Eq, Copy, Clone, Hash)]
struct Plot {
    row: usize,
    col: usize,
    plant: char,
}

impl Plot {
    fn new(row: usize, col: usize, plant: char) -> Plot {
        Plot { row, col, plant }
    }

    fn get_sides(&self, region: &Vec<&Plot>) -> Vec<Side> {
        let mut s: Vec<Side> = Vec::new();
        let east = region
            .iter()
            .find(|p| p.plant == self.plant && p.col == self.col + 1 && p.row == self.row);
        if east.is_none() {
            s.push(Side::new(Direction::East, self.col));
        }
        if self.col as i32 - 1 >= 0 {
            let west = region
                .iter()
                .find(|p| p.plant == self.plant && p.col == self.col - 1 && p.row == self.row);
            if west.is_none() {
                s.push(Side::new(Direction::West, self.col));
            }
        } else {
            s.push(Side::new(Direction::West, self.col));
        }
        let south = region
            .iter()
            .find(|p| p.plant == self.plant && p.col == self.col && p.row == self.row + 1);
        if south.is_none() {
            s.push(Side::new(Direction::South, self.row));
        }
        if self.row as i32 - 1 >= 0 {
            let north = region
                .iter()
                .find(|p| p.plant == self.plant && p.col == self.col && p.row == self.row - 1);
            if north.is_none() {
                s.push(Side::new(Direction::North, self.row));
            }
        } else {
            s.push(Side::new(Direction::North, self.row));
        }
        s
    }

    fn perimeter(&self, region: &Vec<&Plot>) -> usize {
        4 - self.valid_orthogonal_neighbors(region).len()
    }

    fn valid_orthogonal_neighbors<'a>(&'a self, region: &Vec<&'a Plot>) -> Vec<&'a Plot> {
        let mut n: Vec<&Plot> = Vec::new();
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

impl PartialEq for Plot {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col && self.plant == other.plant
    }
}

fn build_regions(lines: Vec<String>) -> Vec<Vec<Plot>> {
    let plots: Vec<Plot> = lines
        .iter()
        .enumerate()
        .flat_map(|line| {
            line.1
                .chars()
                .enumerate()
                .map(|c| Plot::new(line.0, c.0, c.1))
                .collect_vec()
        })
        .collect();

    let mut result: Vec<Vec<Plot>> = Vec::new();
    let mut visited: Vec<&Plot> = Vec::new();
    for plot in &plots {
        if visited.iter().any(|&x| x == plot) {
            continue;
        }
        let mut region: Vec<&Plot> = Vec::new();
        let mut queue: Queue<&Plot> = Queue::new();
        queue.add(&plot).expect("uh oh");
        while queue.size() > 0 {
            let p = queue.remove().unwrap();
            if visited.iter().any(|&x| x == p) {
                continue;
            }
            visited.push(p);
            region.push(p);
            let neighbors = p.valid_orthogonal_neighbors(&plots.iter().collect_vec());
            for neighbor in neighbors.into_iter() {
                if !visited.iter().any(|&x| x == neighbor) {
                    queue.add(&neighbor).expect("uh oh");
                }
            }
        }
        result.push(region.into_iter().cloned().collect_vec());
    }
    result
}

fn price_part1(lines: Vec<String>) -> usize {
    let regions = build_regions(lines);
    let mut result = 0;
    for region in regions.iter() {
        result += region.len()
            * region
                .iter()
                .map(|r| r.perimeter(&region.iter().collect_vec()))
                .sum::<usize>();
    }
    result
}

fn price_part2(lines: Vec<String>) -> usize {
    let regions = build_regions(lines);
    let mut result = 0;
    for region in regions {
        let sides = region
            .iter()
            .flat_map(|p| {
                p.get_sides(&region.iter().collect_vec())
                    .iter()
                    .map(|&s| (s, p))
                    .collect_vec()
            })
            .into_group_map_by(|s| s.0);
        let mut side_count = 0;
        for (_, side) in sides {
            let mut s: Vec<Vec<&Plot>> = Vec::new();
            for (_, p) in side {
                if let Some(possible_side) = s
                    .iter()
                    .position(|x| x.iter().any(|y| y.valid_orthogonal_neighbors(&vec![p]).len() != 0))
                {
                    let z = s.get_mut(possible_side).unwrap();
                    z.push(p);
                } else {
                    s.push(vec![p]);
                }
            }
            side_count += s.len();
        }
        result += side_count * region.len();
    }
    result
}

fn main() {
    let (part1, time1) = time(|| price_part1(read_lines("src/input.txt")));
    println!("Part 1: {} (took {} seconds)", part1, time1.as_secs_f64());
    let (part2, time2) = time(|| price_part2(read_lines("src/input.txt")));
    println!("Part 2: {} (took {} seconds)", part2, time2.as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::read_lines;

    #[test]
    fn test_part_1() {
        let price = price_part1(read_lines("src/sample-map.txt"));
        assert_eq!(price, 1930);
    }

    #[test]
    fn test_part_2() {
        let price = price_part2(read_lines("src/sample-map.txt"));
        assert_eq!(price, 1206);
    }
}
