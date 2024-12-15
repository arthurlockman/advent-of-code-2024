mod object;

use crate::object::{AddMove, Direction, Object, ObjectType};
use itertools::Itertools;
use std::collections::HashMap;
use utils::{read_file, read_lines, time};

fn main() {
    let (part1, time1) = time(|| run_sim("src/map.txt", "src/moves.txt", false));
    println!("Part 1: {} (took {} seconds)", part1, time1.as_secs_f64());
    let (part2, time2) = time(|| run_sim("src/map.txt", "src/moves.txt", true));
    println!("Part 2: {} (took {} seconds)", part2, time2.as_secs_f64());
}

fn parse_map(filename: &str, wide: bool) -> (HashMap<(i32, i32), Object>, (i32, i32), i32, i32) {
    let mut file = read_lines(filename);
    let mut map: HashMap<(i32, i32), Object> = HashMap::new();
    let mut robot: (i32, i32) = (-1, -1);

    if !wide {
        for (row, line) in file.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '#' || c == 'O' {
                    let o = Object::new(c);
                    map.insert((row as i32, col as i32), o);
                } else if c == '@' {
                    robot = (row as i32, col as i32);
                }
            }
        }
    } else {
        file = file
            .iter()
            .map(|l| {
                l.replace("#", "##")
                    .replace(".", "..")
                    .replace("@", "@.")
                    .replace("O", "[]")
            })
            .collect_vec();
        for (row, line) in file.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                match c {
                    '[' | '#' | ']' => {
                        map.insert((row as i32, col as i32), Object::new(c));
                    }
                    '@' => robot = (row as i32, col as i32),
                    _ => (),
                }
            }
        }
    }
    (map, robot, file.len() as i32, file[0].len() as i32)
}

fn parse_moves(filename: &str) -> Vec<Direction> {
    read_file(filename)
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::North),
            '>' => Some(Direction::East),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            _ => None,
        })
        .collect()
}

fn run_sim(map_file: &str, moves_file: &str, wide: bool) -> i32 {
    let (mut map, mut robot, rows, cols) = parse_map(map_file, wide);
    let moves = parse_moves(moves_file);
    for m in moves {
        let mov = m.to_move();
        let next_position = (robot.0 + mov.0, robot.1 + mov.1);
        if let Some(next) = map.get(&next_position) {
            // Something is in the next position, let's see if we can push it
            let (new_map, push_result) = next.try_push(m, next_position, map.clone(), false);
            if push_result {
                // We pushed it
                map = new_map;
                robot = next_position;
            }
        } else {
            // Next position is empty, we can move and the map doesn't change
            robot = next_position;
        }
    }
    print_map(&map, rows, cols, robot);
    // Finally calculate the score
    let mut score = 0;
    for row in 0..rows {
        for col in 0..cols {
            if let Some(o) = map.get(&(row, col)) {
                if o.kind == ObjectType::Box || o.kind == ObjectType::BoxLeft {
                    score += 100 * row + col;
                }
            }
        }
    }
    score
}

fn print_map(map: &HashMap<(i32, i32), Object>, rows: i32, cols: i32, robot: (i32, i32)) {
    for row in 0..rows {
        for col in 0..cols {
            if row == robot.0 && col == robot.1 {
                print!("@");
            } else if let Some(o) = map.get(&(row, col)) {
                match o.kind {
                    ObjectType::Wall => print!("#"),
                    ObjectType::Box => print!("O"),
                    ObjectType::BoxLeft => print!("["),
                    ObjectType::BoxRight => print!("]"),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let score = run_sim("src/sample-map.txt", "src/sample-moves.txt", false);
        assert_eq!(score, 10092);
    }

    #[test]
    fn test_part_2() {
        let score = run_sim("src/sample-map.txt", "src/sample-moves.txt", true);
        assert_eq!(score, 9021);
    }
}
