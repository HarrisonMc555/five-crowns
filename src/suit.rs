use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum Suit {
    Spade,
    Club,
    Heart,
    Diamond,
    Star,
}

impl Suit {
    pub fn try_from(c: char) -> Option<Self> {
        Some(match c {
            'S' => Suit::Spade,
            'C' => Suit::Club,
            'H' => Suit::Heart,
            'D' => Suit::Diamond,
            'R' => Suit::Star,
            _ => return None,
        })
    }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_from() {
        assert_eq!(Some(Suit::Spade), Suit::try_from('S'));
        assert_eq!(Some(Suit::Club), Suit::try_from('C'));
        assert_eq!(Some(Suit::Heart), Suit::try_from('H'));
        assert_eq!(Some(Suit::Diamond), Suit::try_from('D'));
        assert_eq!(Some(Suit::Star), Suit::try_from('R'));
        assert_eq!(None, Suit::try_from('A'));
        assert_eq!(None, Suit::try_from('B'));
        assert_eq!(None, Suit::try_from('E'));
        assert_eq!(None, Suit::try_from('F'));
    }
}
