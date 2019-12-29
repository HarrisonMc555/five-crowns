use crate::card::Card;
use crate::rank::Rank;
use crate::suit::Suit;

pub enum ScoreGroup {
    Run(Run),
    Set(Set),
}

pub struct Run {
    suit: Suit,
    low: Rank,
    high: Rank,
}

pub struct Set {
    rank: Rank,
    suits: Vec<Suit>,
}
