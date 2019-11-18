use std::fmt;

use crate::rank::{Rank, ALL_RANKS};
use crate::suit::{Suit, ALL_SUITS};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

pub(crate) fn full_deck() -> impl Iterator<Item = Card> {
    ALL_SUITS
        .iter()
        .flat_map(move |&suit| ALL_RANKS.iter().map(move |&rank| Card::new(suit, rank)))
}
