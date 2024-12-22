mod button;

use crate::button::{Button, ToButton};
use itertools::Itertools;
use memoize::memoize;
use std::collections::HashMap;
use std::iter::zip;
use std::sync::LazyLock;
use utils::{time, Point};

static NUMERIC_PAD: LazyLock<HashMap<Button, Point>> = LazyLock::new(|| {
    vec![
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
        (Button::Activate, Point::new(3, 2)),
    ]
    .into_iter()
    .collect()
});

static D_PAD: LazyLock<HashMap<Button, Point>> = LazyLock::new(|| {
    vec![
        (Button::Up, Point::new(0, 1)),
        (Button::Down, Point::new(1, 1)),
        (Button::Left, Point::new(1, 0)),
        (Button::Right, Point::new(1, 2)),
        (Button::Activate, Point::new(0, 2)),
    ]
    .into_iter()
    .collect()
});

fn main() {
    let (part1, time1) = time(|| solve(4));
    println!("Part 1: {} (took {} secs)", part1, time1.as_secs_f64());
    let (part2, time2) = time(|| solve(27));
    println!("Part 2: {} (took {} secs)", part2, time2.as_secs_f64());
}

fn solve(keypads: i32) -> u128 {
    let mut r: u128 = 0;
    for i in vec!["319A", "670A", "349A", "964A", "586A"] {
        r += shortest(i.to_string(), keypads) * i[0..3].to_string().parse::<u128>().unwrap()
    }
    r
}

#[memoize]
fn shortest(moves: String, depth: i32) -> u128 {
    if depth == 1 {
        return moves.chars().count() as u128;
    }
    let numeric = "1234567890".chars().any(|x| moves.contains(x));
    let mut res = 0u128;
    for (key1, key2) in zip(format!("A{}", moves).chars(), moves.chars()) {
        let sp = shortest_path(key1.to_button(), key2.to_button(), numeric);
        res += sp
            .into_iter()
            .map(|s| shortest(format!("{}A", s), depth - 1))
            .min()
            .unwrap()
    }
    res
}

#[memoize]
fn shortest_path(a: Button, b: Button, numeric: bool) -> Vec<String> {
    let keypad = if numeric { &NUMERIC_PAD } else { &D_PAD };
    let pos1 = keypad.get(&a).unwrap().clone();
    let pos2 = keypad.get(&b).unwrap().clone();
    let (dr, dc) = (pos2.row - pos1.row, pos2.col - pos1.col);

    let row_moves = if dr >= 0 {
        "v".repeat(dr as usize)
    } else {
        "^".repeat(dr.abs() as usize)
    };
    let col_moves = if dc >= 0 {
        ">".repeat(dc as usize)
    } else {
        "<".repeat(dc.abs() as usize)
    };

    if dr == 0 && dc == 0 {
        vec!["".to_string()]
    } else if dr == 0 {
        vec![col_moves]
    } else if dc == 0 {
        vec![row_moves]
    } else if !keypad.values().contains(&Point::new(pos1.row, pos2.col)) {
        vec![format!("{}{}", row_moves, col_moves)]
    } else if !keypad.values().contains(&Point::new(pos2.row, pos1.col)) {
        vec![format!("{}{}", col_moves, row_moves)]
    } else {
        vec![
            format!("{}{}", col_moves, row_moves),
            format!("{}{}", row_moves, col_moves),
        ]
    }
}
