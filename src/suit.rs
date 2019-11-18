use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum Suit {
    Spade,
    Club,
    Heart,
    Diamond,
    Star,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Suit::Spade => "S",
            Suit::Club => "C",
            Suit::Heart => "H",
            Suit::Diamond => "D",
            Suit::Star => "R",
        };
        write!(f, "{}", symbol)
    }
}

pub(crate) const ALL_SUITS: [Suit; 5] = [
    Suit::Spade,
    Suit::Club,
    Suit::Heart,
    Suit::Diamond,
    Suit::Star,
];
