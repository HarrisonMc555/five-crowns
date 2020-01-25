use std::fmt;

use crate::score::Score;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum Rank {
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn score(&self) -> Score {
        Score(match self {
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
        })
    }

    pub fn next(&self) -> Option<Rank> {
        self.plus(1)
    }

    pub fn plus(&self, offset: usize) -> Option<Rank> {
        ALL_RANKS.get(self.index() + offset).cloned()
    }

    pub fn minus(&self, offset: usize) -> Option<Rank> {
        ALL_RANKS.get(self.index() - offset).cloned()
    }

    fn index(&self) -> usize {
        match self {
            Rank::Three => 0,
            Rank::Four => 1,
            Rank::Five => 2,
            Rank::Six => 3,
            Rank::Seven => 4,
            Rank::Eight => 5,
            Rank::Nine => 6,
            Rank::Ten => 7,
            Rank::Jack => 8,
            Rank::Queen => 9,
            Rank::King => 10,
        }
    }

    pub fn try_from(string: &str) -> Option<Self> {
        Some(match string {
            "3" => Rank::Three,
            "4" => Rank::Four,
            "5" => Rank::Five,
            "6" => Rank::Six,
            "7" => Rank::Seven,
            "8" => Rank::Eight,
            "9" => Rank::Nine,
            "10" => Rank::Ten,
            "J" => Rank::Jack,
            "Q" => Rank::Queen,
            "K" => Rank::King,
            _ => return None,
        })
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        };
        write!(f, "{}", symbol)
    }
}

pub(crate) const ALL_RANKS: [Rank; 11] = [
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next() {
        assert_eq!(Rank::Three.next(), Some(Rank::Four));
        assert_eq!(Rank::Ten.next(), Some(Rank::Jack));
        assert_eq!(Rank::Queen.next(), Some(Rank::King));
        assert_eq!(Rank::King.next(), None);
    }

    #[test]
    fn test_plus() {
        assert_eq!(Rank::Three.plus(0), Some(Rank::Three));
        assert_eq!(Rank::Three.plus(1), Some(Rank::Four));
        assert_eq!(Rank::Three.plus(2), Some(Rank::Five));
        assert_eq!(Rank::Three.plus(5), Some(Rank::Eight));
        assert_eq!(Rank::Three.plus(5), Some(Rank::Eight));
        assert_eq!(Rank::Three.plus(10), Some(Rank::King));
        assert_eq!(Rank::Three.plus(11), None);
        assert_eq!(Rank::Three.plus(12), None);
        assert_eq!(Rank::Three.plus(20), None);
        assert_eq!(Rank::Four.plus(0), Some(Rank::Four));
        assert_eq!(Rank::Four.plus(1), Some(Rank::Five));
        assert_eq!(Rank::Four.plus(2), Some(Rank::Six));
        assert_eq!(Rank::Four.plus(5), Some(Rank::Nine));
        assert_eq!(Rank::Four.plus(9), Some(Rank::King));
        assert_eq!(Rank::Four.plus(10), None);
        assert_eq!(Rank::Four.plus(11), None);
        assert_eq!(Rank::Four.plus(50), None);
        assert_eq!(Rank::Ten.plus(0), Some(Rank::Ten));
        assert_eq!(Rank::Ten.plus(1), Some(Rank::Jack));
        assert_eq!(Rank::Ten.plus(2), Some(Rank::Queen));
        assert_eq!(Rank::Ten.plus(3), Some(Rank::King));
        assert_eq!(Rank::Ten.plus(4), None);
        assert_eq!(Rank::Ten.plus(5), None);
        assert_eq!(Rank::Ten.plus(99), None);
    }

    #[test]
    fn test_try_from() {
        assert_eq!(Some(Rank::Three), Rank::try_from("3"));
        assert_eq!(Some(Rank::Four), Rank::try_from("4"));
        assert_eq!(Some(Rank::Five), Rank::try_from("5"));
        assert_eq!(Some(Rank::Six), Rank::try_from("6"));
        assert_eq!(Some(Rank::Seven), Rank::try_from("7"));
        assert_eq!(Some(Rank::Eight), Rank::try_from("8"));
        assert_eq!(Some(Rank::Nine), Rank::try_from("9"));
        assert_eq!(Some(Rank::Ten), Rank::try_from("10"));
        assert_eq!(Some(Rank::Jack), Rank::try_from("J"));
        assert_eq!(Some(Rank::Queen), Rank::try_from("Q"));
        assert_eq!(Some(Rank::King), Rank::try_from("K"));
        assert_eq!(None, Rank::try_from("1"));
        assert_eq!(None, Rank::try_from("2"));
        assert_eq!(None, Rank::try_from("11"));
        assert_eq!(None, Rank::try_from("12"));
        assert_eq!(None, Rank::try_from("13"));
        assert_eq!(None, Rank::try_from("14"));
        assert_eq!(None, Rank::try_from("14"));
        assert_eq!(None, Rank::try_from("A"));
        assert_eq!(None, Rank::try_from("S"));
        assert_eq!(None, Rank::try_from("C"));
        assert_eq!(None, Rank::try_from("H"));
        assert_eq!(None, Rank::try_from("D"));
        assert_eq!(None, Rank::try_from("R"));
        assert_eq!(None, Rank::try_from(""));
    }
}
