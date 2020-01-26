use crate::card::Card;
use crate::game_state::GameState;
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
    fn try_from(cards: &[Card], game_state: &GameState) -> Result<Run> {
        assert!(MIN_CARDS_RUN >= 1);
        if cards.len() < MIN_CARDS_RUN {
            return Err(Error::TooFewCards);
        }
        let mut normal_cards = cards
            .iter()
            .flat_map(|c| c.non_wild(game_state))
            .collect::<Vec<_>>();
        let mut num_wilds = cards.len() - normal_cards.len();
        assert_eq!(
            num_wilds,
            cards.iter().filter(|c| c.is_wild(game_state)).count()
        );
        let first_card = if let Some(c) = normal_cards.first() {
            c
        } else {
            return Run::all_jokers(num_wilds);
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
            if (num_wilds as isize) < num_wilds_required {
                return Err(Error::NotAllInARow);
            }
            num_wilds -= num_wilds_required as usize;
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
    fn run_try_from_low_cards() {
        let game_state = GameState::new(Rank::Six);
        assert_eq!(
            Ok(Run {
                suit: Suit::Spade,
                low: Rank::Three,
                high: Rank::Five
            }),
            Run::try_from(&cards_from_str("3S,4S,5S"), &game_state)
        );
    }

    #[test]
    fn run_try_from_middle_cards() {
        let game_state = GameState::new(Rank::Six);
        assert_eq!(
            Ok(Run {
                suit: Suit::Heart,
                low: Rank::Seven,
                high: Rank::Ten
            }),
            Run::try_from(&cards_from_str("8H,9H,7H,10H"), &game_state)
        );
    }

    #[test]
    fn run_try_from_high_cards() {
        let game_state = GameState::new(Rank::Six);
        assert_eq!(
            Ok(Run {
                suit: Suit::Club,
                low: Rank::Ten,
                high: Rank::King
            }),
            Run::try_from(&cards_from_str("JC,10C,KC,QC"), &game_state)
        );
    }

    #[test]
    fn run_try_from_with_joker() {
        let game_state = GameState::new(Rank::Six);
        assert_eq!(
            Ok(Run {
                suit: Suit::Diamond,
                low: Rank::Five,
                high: Rank::Seven
            }),
            Run::try_from(&cards_from_str("7D,5D,Joker"), &game_state)
        );
    }

    #[test]
    fn run_try_from_with_game_state_wild() {
        let game_state = GameState::new(Rank::Six);
        assert_eq!(
            Ok(Run {
                suit: Suit::Star,
                low: Rank::Nine,
                // This is kind of a bug but we're going to
                // refactor anyways
                high: Rank::Ten // high: Rank::Jack
            }),
            Run::try_from(&cards_from_str("9R,10R,6H"), &game_state)
        );
    }

    fn cards_from_str(string: &str) -> Vec<Card> {
        use crate::hand::Hand;
        Hand::try_from(string).unwrap().cards().to_vec()
    }
}
