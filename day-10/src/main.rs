use std::hash::Hash;

use array2d::Array2D;
use itertools::Itertools;
use utils::{read_lines, time};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Tile {
    height: i32,
    position: (i32, i32),
}

impl Tile {
    fn new(height: u32, x: usize, y: usize) -> Tile {
        Tile {
            height: height as i32,
            position: (x as i32, y as i32),
        }
    }
}

fn main() {
    let (score1, time1) = time(|| score_trailheads(build_map(read_lines("src/input.txt")), true));
    println!(
        "Part 1 Score: {} (took {} seconds)",
        score1,
        time1.as_secs_f64()
    );
    let (score2, time2) = time(|| score_trailheads(build_map(read_lines("src/input.txt")), false));
    println!(
        "Part 2 Score: {} (took {} seconds)",
        score2,
        time2.as_secs_f64()
    );
}

fn score_trailheads(map: Array2D<Tile>, distinct: bool) -> usize {
    let mut score = 0;
    // Starting from each trailhead...
    for trailhead in map.elements_row_major_iter().filter(|&e| e.height == 0) {
        let mut visited: Vec<&Tile> = Vec::new();
        visited.push(trailhead);
        // First find all the neighbors of the trailhead tile
        let mut n: Vec<&Tile> = neighbors(&map, trailhead);
        if n.len() == 0 {
            // bail out, this trailhead has no eligible neighbors
            continue;
        }
        let mut local_score = 0;
        while n.len() > 0 {
            visited.extend_from_slice(&n);
            let new_neighbors: Vec<&Tile> = if distinct {
                n.iter()
                    .flat_map(|&t| neighbors(&map, t))
                    .unique()
                    .filter(|t| !visited.contains(t))
                    .collect()
            } else {
                n.iter()
                    .flat_map(|&t| neighbors(&map, t))
                    .filter(|t| !visited.contains(t))
                    .collect()
            };
            local_score += new_neighbors.iter().filter(|&&t| t.height == 9).count();
            n = new_neighbors;
        }
        score += local_score
    }
    score
}

fn neighbors<'a, 'b>(map: &'a Array2D<Tile>, tile: &'b Tile) -> Vec<&'a Tile> {
    let mut n: Vec<&Tile> = Vec::new();
    if let Some(t) = map.get(
        (tile.position.1 + 1).try_into().unwrap_or(0),
        tile.position.0.try_into().unwrap_or(0),
    ) {
        if t.height == tile.height + 1 {
            n.push(t);
        }
    }
    if let Some(t) = map.get(
        (tile.position.1 - 1).try_into().unwrap_or(0),
        tile.position.0.try_into().unwrap_or(0),
    ) {
        if t.height == tile.height + 1 {
            n.push(t);
        }
    }
    if let Some(t) = map.get(
        tile.position.1.try_into().unwrap_or(0),
        (tile.position.0 + 1).try_into().unwrap_or(0),
    ) {
        if t.height == tile.height + 1 {
            n.push(t);
        }
    }
    if let Some(t) = map.get(
        tile.position.1.try_into().unwrap_or(0),
        (tile.position.0 - 1).try_into().unwrap_or(0),
    ) {
        if t.height == tile.height + 1 {
            n.push(t);
        }
    }
    n
}

fn build_map(raw_map: Vec<String>) -> Array2D<Tile> {
    let rows: Vec<Vec<Tile>> = raw_map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Tile::new(c.to_digit(10).unwrap(), x, y))
                .collect()
        })
        .collect();
    Array2D::from_rows(&rows).expect("uh oh")
}

#[cfg(test)]
mod tests {
    use utils::read_lines;

    use super::*;

    #[test]
    fn test_part_1() {
        let map = build_map(read_lines("src/test-input.txt"));
        let score = score_trailheads(map, true);
        assert_eq!(score, 36);
    }

    #[test]
    fn test_part_2() {
        let map = build_map(read_lines("src/test-input.txt"));
        let score = score_trailheads(map, false);
        assert_eq!(score, 81);
    }
}
