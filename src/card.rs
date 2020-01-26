use std::fmt;

use crate::game_state::GameState;
use crate::rank::{Rank, ALL_RANKS};
use crate::score::Score;
use crate::suit::{Suit, ALL_SUITS};
use crate::utils;

const NUM_JOKERS_IN_DECK: usize = 4;
const JOKER_SCORE: Score = Score(25);
const JOKER_STRING: &str = "Joker";

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum Card {
    Normal(NormalCard),
    Joker,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct NormalCard {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card::Normal(NormalCard { suit, rank })
    }

    pub fn joker() -> Self {
        Card::Joker
    }

    pub fn score(&self) -> Score {
        match self {
            Card::Normal(card) => card.rank.score(),
            Card::Joker => JOKER_SCORE,
        }
    }

    pub fn suit(&self) -> Option<Suit> {
        self.normal().map(NormalCard::suit)
    }

    pub fn rank(&self) -> Option<Rank> {
        self.normal().map(NormalCard::rank)
    }

    pub fn is_wild(&self, game_state: &GameState) -> bool {
        game_state.is_card_wild(*self)
    }

    pub fn normal(&self) -> Option<&NormalCard> {
        if let Card::Normal(card) = self {
            Some(card)
        } else {
            None
        }
    }

    pub fn non_wild(&self, game_state: &GameState) -> Option<&NormalCard> {
        if let Card::Normal(card) = self {
            if self.is_wild(game_state) {
                None
            } else {
                Some(card)
            }
        } else {
            None
        }
    }

    pub fn is_normal(&self) -> bool {
        match self {
            Card::Normal(_) => true,
            _ => false,
        }
    }

    pub fn is_joker(&self) -> bool {
        match self {
            Card::Joker => true,
            _ => false,
        }
    }

    pub fn try_from(string: &str) -> Option<Self> {
        if string == JOKER_STRING {
            return Some(Card::Joker);
        }
        let (rank_str, suit_char) = utils::split_last_char(string)?;
        let suit = Suit::try_from(suit_char)?;
        let rank = Rank::try_from(&rank_str)?;
        Some(Card::new(suit, rank))
    }
}

impl NormalCard {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        NormalCard { suit, rank }
    }

    pub fn score(&self) -> Score {
        self.rank.score()
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }
}

pub(crate) fn full_deck() -> impl Iterator<Item = Card> {
    ALL_SUITS
        .iter()
        .flat_map(move |&suit| {
            ALL_RANKS
                .iter()
                .map(move |&rank| Card::Normal(NormalCard::new(suit, rank)))
        })
        .chain(std::iter::repeat(Card::Joker).take(NUM_JOKERS_IN_DECK))
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Card::Normal(card) => write!(f, "{}{}", card.rank, card.suit),
            Card::Joker => write!(f, "{}", JOKER_STRING),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_from() {
        // Valid
        assert_eq!(
            Some(Card::new(Suit::Spade, Rank::Three)),
            Card::try_from("3S")
        );
        assert_eq!(
            Some(Card::new(Suit::Club, Rank::Five)),
            Card::try_from("5C")
        );
        assert_eq!(
            Some(Card::new(Suit::Heart, Rank::Seven)),
            Card::try_from("7H")
        );
        assert_eq!(
            Some(Card::new(Suit::Diamond, Rank::Ten)),
            Card::try_from("10D")
        );
        assert_eq!(
            Some(Card::new(Suit::Star, Rank::Jack)),
            Card::try_from("JR")
        );
        assert_eq!(
            Some(Card::new(Suit::Spade, Rank::Queen)),
            Card::try_from("QS")
        );
        assert_eq!(
            Some(Card::new(Suit::Club, Rank::King)),
            Card::try_from("KC")
        );
        // Invalid rank
        assert_eq!(None, Card::try_from("1D"));
        assert_eq!(None, Card::try_from("2R"));
        assert_eq!(None, Card::try_from("11S"));
        assert_eq!(None, Card::try_from("12C"));
        // Invalid suit
        assert_eq!(None, Card::try_from("3A"));
        assert_eq!(None, Card::try_from("4B"));
        assert_eq!(None, Card::try_from("10E"));
        assert_eq!(None, Card::try_from("JF"));
        // Backwards
        assert_eq!(None, Card::try_from("S3"));
        assert_eq!(None, Card::try_from("S3"));
        assert_eq!(None, Card::try_from("C5"));
        assert_eq!(None, Card::try_from("H7"));
        assert_eq!(None, Card::try_from("D10"));
        assert_eq!(None, Card::try_from("RJ"));
        assert_eq!(None, Card::try_from("SQ"));
        assert_eq!(None, Card::try_from("CK"));
        // Empty
        assert_eq!(None, Card::try_from(""));
    }
}
