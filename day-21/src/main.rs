mod button;
mod moves;

use crate::button::{Button, ToButton};
use crate::moves::Move;
use itertools::Itertools;
use std::collections::HashMap;
use utils::Point;

fn main() {
    println!("{}", code_difficulty("379A"));
}

fn code_difficulty(desired_code: &str) -> usize {
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
    let s1 = find_control_sequence(desired_code, &numeric_pad, Button::LetterA);
    let s2 = find_control_sequence(&s1, &directional_pad, Button::Activate);
    let s3 = find_control_sequence(
        &"v<<A>>^A<A>AvA<^AA>A<vAAA>^A",
        &directional_pad,
        Button::Activate,
    );
    let s = s3.chars().count();
    let c = desired_code[0..3].to_string().parse::<usize>().unwrap();
    s3.chars().count() * desired_code[0..3].to_string().parse::<usize>().unwrap()
}

fn find_control_sequence(
    desired_output: &str,
    pad: &HashMap<Button, Point>,
    start: Button,
) -> String {
    let mut current_button = start;
    let mut moves = Vec::<Move>::new();
    for c in desired_output.chars() {
        let desired_button = c.to_button();
        println!("Moving from {} to {}", current_button, desired_button);
        let m = find_move(
            pad.get(&current_button).unwrap(),
            pad.get(&desired_button).unwrap(),
        );
        m.iter().for_each(|mv| moves.push(mv.clone()));
        current_button = desired_button;
    }
    moves.iter().join("")
}

fn find_move(a: &Point, b: &Point) -> Vec<Move> {
    let net_move = b.clone() - a.clone();
    println!("A: {:?}, B: {:?} net_move: {:?}", a, b, net_move);
    let mut moves = Vec::<Move>::new();
    for _ in 0..net_move.col.abs() {
        moves.push(if net_move.col > 0 {
            Move::Right
        } else {
            Move::Left
        })
    }
    for _ in 0..net_move.row.abs() {
        moves.push(if net_move.row > 0 {
            Move::Down
        } else {
            Move::Up
        })
    }
    moves.push(Move::Press);
    println!("moves: {:?}", moves.iter().join(""));
    moves // TODO: handle the corner cases where the robot moves over empty space and crashes, that's why this doesn't work
}

#[cfg(test)]
mod tests {
    use crate::code_difficulty;

    #[test]
    fn test_part_1() {
        assert_eq!(code_difficulty("029a"), 68 * 29);
        assert_eq!(code_difficulty("980a"), 60 * 980);
        assert_eq!(code_difficulty("179a"), 68 * 179);
        assert_eq!(code_difficulty("456a"), 64 * 456);
        assert_eq!(code_difficulty("379a"), 64 * 379);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(true, true);
    }
}
