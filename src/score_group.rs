use crate::card::Card;
use crate::rank::Rank;
use crate::suit::Suit;

const MIN_CARDS: usize = 3;
const MIN_CARDS_RUN: usize = MIN_CARDS;
const MIN_CARDS_SET: usize = MIN_CARDS;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    TooFewCards,
    TooManyCards,
    NotAllSameSuit,
    NotAllInARow,
}

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

impl Run {
    fn try_from(cards: &[Card]) -> Result<Run> {
        assert!(MIN_CARDS_RUN >= 1);
        if cards.len() < MIN_CARDS_RUN {
            return Err(Error::TooFewCards);
        }
        let mut normal_cards = cards.iter().flat_map(Card::normal).collect::<Vec<_>>();
        let num_jokers = cards.len() - normal_cards.len();
        assert_eq!(num_jokers, cards.iter().filter(|c| c.is_joker()).count());
        let first_card = if let Some(c) = normal_cards.first() {
            c
        } else {
            return Run::all_jokers(num_jokers);
        };
        let suit = first_card.suit();
        if !normal_cards[1..].iter().all(|c| c.suit() == suit) {
            return Err(Error::NotAllSameSuit);
        }
        normal_cards.sort_unstable_by_key(|c| c.rank());
        for window in normal_cards.windows(2) {
            let (prev, next) = (window[0], window[1]);
            if prev.rank().next() != Some(next.rank()) {
                return Err(Error::NotAllInARow);
            }
        }
        Ok(Run {
            suit,
            low: normal_cards.first().unwrap().rank(),
            high: normal_cards.last().unwrap().rank(),
        })
    }

    fn all_jokers(num_jokers: usize) -> Result<Run> {
        Ok(Run {
            suit: Suit::Club,
            low: Rank::Three,
            high: Rank::Three.plus(num_jokers).ok_or(Error::TooManyCards)?,
        })
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_run_try_from() {
//         assert_eq!(Run::try_from());
//     }
// }
