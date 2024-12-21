use std::fmt::Display;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Button {
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
    Zero,
    LetterA,
    Up,
    Down,
    Left,
    Right,
    Activate,
}

pub trait ToButton {
    fn to_button(&self) -> Button;
}

impl ToButton for char {
    fn to_button(&self) -> Button {
        match self {
            '9' => Button::Nine,
            '8' => Button::Eight,
            '7' => Button::Seven,
            '6' => Button::Six,
            '5' => Button::Five,
            '4' => Button::Four,
            '3' => Button::Three,
            '2' => Button::Two,
            '1' => Button::One,
            '0' => Button::Zero,
            'a' => Button::LetterA,
            'A' => Button::Activate,
            '^' => Button::Up,
            'v' => Button::Down,
            '>' => Button::Right,
            '<' => Button::Left,
            _ => panic!(),
        }
    }
}

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Button::Nine => write!(f, "9"),
            Button::Eight => write!(f, "8"),
            Button::Seven => write!(f, "7"),
            Button::Six => write!(f, "6"),
            Button::Five => write!(f, "5"),
            Button::Four => write!(f, "4"),
            Button::Three => write!(f, "3"),
            Button::Two => write!(f, "2"),
            Button::One => write!(f, "1"),
            Button::Zero => write!(f, "0"),
            Button::LetterA => write!(f, "a"),
            Button::Activate => write!(f, "A"),
            Button::Up => write!(f, "^"),
            Button::Down => write!(f, "v"),
            Button::Left => write!(f, "<"),
            Button::Right => write!(f, ">"),
        }
    }
}
