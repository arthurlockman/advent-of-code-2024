use std::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
    Press,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Up => f.write_str("^"),
            Move::Down => f.write_str("v"),
            Move::Left => f.write_str("<"),
            Move::Right => f.write_str(">"),
            Move::Press => f.write_str("A"),
        }
    }
}
