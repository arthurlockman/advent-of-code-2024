use itertools::Itertools;
use queues::{IsQueue, Queue};
use utils::{read_lines, time};

#[derive(Debug, Eq, Copy, Clone, Hash)]
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

fn price(lines: Vec<String>) -> usize {
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

    let mut result = 0usize;
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
        result += region.len() * region.iter().map(|r| r.perimeter(&region)).sum::<usize>();
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
