use crate::card::Card;
use crate::rank::Rank;
use crate::suit::Suit;

const MIN_CARDS: usize = 3;
const MIN_CARDS_RUN: usize = MIN_CARDS;
const MIN_CARDS_SET: usize = MIN_CARDS;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    TooFewCards,
    TooManyCards,
    NotAllSameSuit,
    NotAllInARow,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ScoreGroup {
    Run(Run),
    Set(Set),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Run {
    suit: Suit,
    low: Rank,
    high: Rank,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        let mut num_jokers = cards.len() - normal_cards.len();
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
            let steps = prev.rank().steps_to(next.rank());
            let num_wilds_required = steps - 1;
            if num_wilds_required < num_jokers as isize {
                return Err(Error::NotAllInARow);
            }
            num_jokers -= num_wilds_required as usize;
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_run_try_from() {
        assert_eq!(
            Ok(Run {
                suit: Suit::Spade,
                low: Rank::Three,
                high: Rank::Five
            }),
            Run::try_from(&cards_from_str("3S,4S,5S"))
        );
        assert_eq!(
            Ok(Run {
                suit: Suit::Heart,
                low: Rank::Seven,
                high: Rank::Ten
            }),
            Run::try_from(&cards_from_str("8H,9H,7H,10H"))
        );
        assert_eq!(
            Ok(Run {
                suit: Suit::Club,
                low: Rank::Ten,
                high: Rank::King
            }),
            Run::try_from(&cards_from_str("JC,10C,KC,QC"))
        );
        assert_eq!(
            Ok(Run {
                suit: Suit::Diamond,
                low: Rank::Five,
                high: Rank::Seven
            }),
            Run::try_from(&cards_from_str("7D,5D,Joker"))
        );
    }

    fn cards_from_str(string: &str) -> Vec<Card> {
        use crate::hand::Hand;
        Hand::try_from(string).unwrap().cards().to_vec()
    }
}
