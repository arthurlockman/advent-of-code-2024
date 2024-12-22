mod button;
mod moves;

use crate::button::{Button, ToButton};
use crate::moves::Move;
use itertools::Itertools;
use memoize::memoize;
use std::collections::HashMap;
use utils::{time, Point};

fn main() {
    let (part1, time1) = time(|| part_1());
    println!("Part 1: {} (took {} secs)", part1, time1.as_secs_f64());
    let (part2, time2) = time(|| part_2());
    println!("Part 2: {} (took {} secs)", part2, time2.as_secs_f64());
}

fn part_1() -> u128 {
    let mut r: u128 = 0;
    for i in vec!["319a", "670a", "349a", "964a", "586a"] {
        r += code_difficulty(i, 2);
    }
    r
}

fn part_2() -> u128 {
    let mut r: u128 = 0;
    for i in vec!["319a", "670a", "349a", "964a", "586a"] {
        r += code_difficulty(i, 25);
    }
    r
}

fn code_difficulty(desired_code: &str, iterations: usize) -> u128 {
    let numeric_pad: HashMap<Button, Point> = vec![
        (Button::Seven, Point::new(0, 0)),
        (Button::Eight, Point::new(0, 1)),
        (Button::Nine, Point::new(0, 2)),
        (Button::Four, Point::new(1, 0)),
        (Button::Five, Point::new(1, 1)),
        (Button::Six, Point::new(1, 2)),
        (Button::One, Point::new(2, 0)),
        (Button::Two, Point::new(2, 1)),
        (Button::Three, Point::new(2, 2)),
        (Button::Zero, Point::new(3, 1)),
        (Button::LetterA, Point::new(3, 2)),
    ]
    .into_iter()
    .collect();
    let directional_pad: HashMap<Button, Point> = vec![
        (Button::Up, Point::new(0, 1)),
        (Button::Down, Point::new(1, 1)),
        (Button::Left, Point::new(1, 0)),
        (Button::Right, Point::new(1, 2)),
        (Button::Activate, Point::new(0, 2)),
    ]
    .into_iter()
    .collect();
    let mut s = find_control_sequence(
        desired_code,
        &numeric_pad,
        Button::LetterA,
        &Point::new(3, 0),
    );
    for i in 0..iterations {
        s = find_control_sequence(&s, &directional_pad, Button::Activate, &Point::new(0, 0));
        println!("Finished loop {}", i);
    }
    s.chars().count() as u128 * desired_code[0..3].to_string().parse::<usize>().unwrap() as u128
}

fn find_control_sequence(
    desired_output: &str,
    pad: &HashMap<Button, Point>,
    start: Button,
    hole: &Point,
) -> String {
    let mut current_button = start;
    let mut moves = Vec::<Move>::new();
    for c in desired_output.chars() {
        let desired_button = c.to_button();
        let m = find_move(
            pad.get(&current_button).unwrap().clone(),
            pad.get(&desired_button).unwrap().clone(),
            hole.clone(),
        );
        moves.extend(m);
        current_button = desired_button;
    }
    let r = moves.iter().join("");
    r
}

#[memoize]
fn find_move(a: Point, b: Point, hole: Point) -> Vec<Move> {
    println!("finding move");
    let net_move = b.clone() - a.clone();
    let mut moves = Vec::<Move>::new();
    let mut order = Vec::<Move>::new();
    if net_move.col < 0 && a.row == hole.row && b.col == hole.col {
        //vertical, horizontal
        order.push(Move::Up);
        order.push(Move::Right);
    } else if net_move.row > 0 && a.col == hole.col && b.row == hole.row {
        //h, v
        order.push(Move::Right);
        order.push(Move::Up);
    } else if net_move.row < 0 {
        //h, v
        order.push(Move::Right);
        order.push(Move::Up);
    } else {
        //v, h
        order.push(Move::Up);
        order.push(Move::Right);
    }
    for m in order {
        if m == Move::Up {
            for _ in 0..net_move.row.abs() {
                moves.push(if net_move.row > 0 {
                    Move::Down
                } else {
                    Move::Up
                })
            }
        } else if m == Move::Right {
            for _ in 0..net_move.col.abs() {
                moves.push(if net_move.col > 0 {
                    Move::Right
                } else {
                    Move::Left
                })
            }
        }
    }

    moves.push(Move::Press);
    moves
}

#[cfg(test)]
mod tests {
    use crate::code_difficulty;

    #[test]
    fn test_part_1() {
        assert_eq!(code_difficulty("029a", 2), 68 * 29);
        assert_eq!(code_difficulty("980a", 2), 60 * 980);
        assert_eq!(code_difficulty("179a", 2), 68 * 179);
        assert_eq!(code_difficulty("456a", 2), 64 * 456);
        assert_eq!(code_difficulty("379a", 2), 64 * 379);
    }
}
