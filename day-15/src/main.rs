mod object;

use std::collections::HashMap;
use utils::{read_file, read_lines, time};
use crate::object::{Direction, Object, ObjectType};

fn main() {
    let (part1, time1) = time(|| run_sim("src/map.txt", "src/moves.txt"));
    println!("Part 1: {} (took {} seconds)", part1, time1.as_secs_f64());
}

fn parse_map(filename: &str) -> (HashMap<(i32, i32), Object>, (i32, i32), i32, i32) {
    let file = read_lines(filename);
    let mut map: HashMap<(i32, i32), Object> = HashMap::new();
    let mut robot: (i32, i32) = (-1, -1);

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

fn run_sim(map_file: &str, moves_file: &str) -> i32 {
    let (mut map, mut robot, rows, cols) = parse_map(map_file);
    let moves = parse_moves(moves_file);
    for m in moves {
        let mov = match m {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        };
        let next_position = (robot.0 + mov.0, robot.1 + mov.1);
        if let Some(next) = map.get(&next_position) {
            // Something is in the next position, let's see if we can push it
            let (new_map, push_result) = next.try_push(m, next_position, map.clone());
            if push_result {
                // We pushed it
                map = new_map;
                robot = next_position;
            }
            // No push, no change
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
                if o.kind == ObjectType::Box {
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
        let score = run_sim("src/sample-map.txt", "src/sample-moves.txt");
        assert_eq!(score, 10092);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(true, true);
    }
}
