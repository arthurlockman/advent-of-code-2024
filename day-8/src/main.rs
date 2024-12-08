use std::iter::repeat;

use itertools::Itertools;
use utils::{read_lines, time};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Antenna {
    frequency: char,
    position: (usize, usize),
}

impl Antenna {
    fn new(frequency: char, position: (usize, usize)) -> Self {
        Self {
            frequency,
            position,
        }
    }

    fn antinodes(&self, other: &Antenna, limit: i32, include_antennas: bool) -> Vec<(i32, i32)> {
        let slope_x = self.position.0 as i32 - other.position.0 as i32;
        let slope_y = self.position.1 as i32 - other.position.1 as i32;
        let mut result: Vec<(i32, i32)> = Vec::new();
        let start = match include_antennas {
            true => 0,
            false => 1,
        };
        for i in start..limit + 1 {
            result.push((
                self.position.0 as i32 + slope_x * i,
                self.position.1 as i32 + slope_y * i,
            ));
            result.push((
                other.position.0 as i32 - slope_x * i,
                other.position.1 as i32 - slope_y * i,
            ));
        }
        result
    }
}

fn main() {
    let part1 = time(|| parse_antennas("src/input.txt", 1, false));
    println!(
        "Part 1: there are {} unique antinodes (took {} seconds)",
        part1.0,
        part1.1.as_secs_f64()
    );
    let part2 = time(|| parse_antennas("src/input.txt", 25, true));
    println!(
        "Part 2: there are {} unique antinodes (took {} seconds)",
        part2.0,
        part2.1.as_secs_f64()
    );
}

fn parse_antennas(filename: &str, limit: i32, include_antennas: bool) -> usize {
    let lines = read_lines(filename);
    let map_w = lines[0].len() as i32;
    let map_h = lines.len() as i32;

    let antinodes: Vec<(i32, i32)> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, freq)| {
                if freq != '.' {
                    return Some(Antenna::new(freq, (x, y)));
                }
                None
            })
        })
        .filter(|a| a.is_some())
        .map(|a| a.unwrap())
        .into_group_map_by(|a| a.frequency)
        .iter()
        .flat_map(|(_, antennas)| {
            repeat(antennas.iter())
                .take(2)
                .multi_cartesian_product()
                .filter(|a| !a.iter().all_equal())
                .flat_map(|c| c[0].antinodes(c[1], limit, include_antennas))
        })
        .filter(|&n| n.0 < map_w && n.0 >= 0 && n.1 < map_h && n.1 >= 0)
        .unique()
        .collect();
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let antinodes = parse_antennas("src/test-map.txt", 1, false);
        assert_eq!(antinodes, 14);
    }

    #[test]
    fn test_part_2() {
        let antinodes = parse_antennas("src/test-map.txt", 50, true);
        assert_eq!(antinodes, 34);
    }
}
