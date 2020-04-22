use crate::card::{Card, NormalCard};
use crate::game_state::GameState;
use crate::rank::Rank;
use crate::suit::Suit;

const MIN_CARDS: usize = 3;
const MIN_CARDS_RUN: usize = MIN_CARDS;
const MAX_CARDS_RUN: usize = crate::rank::NUM_RANKS;
const MIN_CARDS_SET: usize = MIN_CARDS;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    TooFewCards,
    TooManyCards,
    NotAllSameSuit,
    NotAllSameRank,
    NotAllInOrder,
    OutOfRange,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ScoreGroup {
    Run(Run),
    Set(Set),
}

#[derive(Debug, Eq, Clone, PartialEq, Ord, PartialOrd)]
pub struct Run {
    cards: Vec<Card>,
    info: RunInfo,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum RunInfo {
    Normal {
        suit: Suit,
        low_rank: Rank,
        high_rank: Rank,
    },
    AllWilds,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Set {
    cards: Vec<Card>,
    info: SetInfo,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum SetInfo {
    Normal { rank: Rank },
    AllWilds,
}

impl Run {
    fn try_from(cards: &[Card], game_state: &GameState) -> Result<Run> {
        if cards.len() < MIN_CARDS_RUN {
            return Err(Error::TooFewCards);
        }
        if cards.len() > MAX_CARDS_RUN {
            return Err(Error::TooManyCards);
        }

        let (index, first_non_wild) = match first_non_wild_with_index(cards, game_state) {
            Some(index_and_card) => index_and_card,
            None => {
                return Ok(Run {
                    cards: cards.to_vec(),
                    info: RunInfo::AllWilds,
                })
            }
        };

        let suit = first_non_wild.suit();
        if !all_match_expected_suit(cards, game_state, suit) {
            return Err(Error::NotAllSameSuit);
        }

        let (low_rank, high_rank) = get_low_high_ranks(first_non_wild.rank(), index, cards.len())
            .ok_or(Error::OutOfRange)?;

        if !all_match_expected_ranks(cards, game_state, low_rank, high_rank) {
            return Err(Error::NotAllInOrder);
        }

        let info = RunInfo::Normal {
            suit,
            low_rank,
            high_rank,
        };

        Ok(Run {
            cards: cards.to_vec(),
            info,
        })
    }
}

fn first_non_wild(cards: &[Card], game_state: &GameState) -> Option<NormalCard> {
    cards
        .iter()
        .filter_map(|card| card.non_wild(game_state).cloned())
        .next()
}

fn first_non_wild_with_index(
    cards: &[Card],
    game_state: &GameState,
) -> Option<(usize, NormalCard)> {
    cards
        .iter()
        .enumerate()
        .filter_map(|(i, card)| card.non_wild(game_state).map(|&nc| (i, nc)))
        .next()
}

fn get_low_high_ranks(rank: Rank, index: usize, total: usize) -> Option<(Rank, Rank)> {
    let low_rank = rank.minus(index)?;
    let high_rank = low_rank.plus(total - 1)?;
    Some((low_rank, high_rank))
}

fn non_wilds<'a>(
    cards: &'a [Card],
    game_state: &'a GameState,
) -> impl 'a + Iterator<Item = NormalCard> {
    cards
        .iter()
        .filter_map(move |card| card.non_wild(game_state))
        .cloned()
}

fn all_match_expected_suit(cards: &[Card], game_state: &GameState, suit: Suit) -> bool {
    non_wilds(cards, game_state).all(|card| card.suit() == suit)
}

fn all_match_expected_ranks(
    cards: &[Card],
    game_state: &GameState,
    low_rank: Rank,
    high_rank: Rank,
) -> bool {
    cards
        .iter()
        .zip(Rank::range(low_rank, high_rank))
        .filter_map(|(card, rank)| card.non_wild(game_state).map(|c| (c, rank)))
        .all(|(card, rank)| card.rank() == rank)
}

fn all_match_expected_rank(cards: &[Card], game_state: &GameState, rank: Rank) -> bool {
    cards
        .iter()
        .filter_map(|card| card.non_wild(game_state))
        .all(|card| card.rank() == rank)
}

impl Set {
    fn try_from(cards: &[Card], game_state: &GameState) -> Result<Set> {
        if cards.len() < MIN_CARDS_SET {
            return Err(Error::TooFewCards);
        }

        let first_non_wild = match first_non_wild(cards, game_state) {
            Some(card) => card,
            None => {
                return Ok(Set {
                    cards: cards.to_vec(),
                    info: SetInfo::AllWilds,
                })
            }
        };

        let rank = first_non_wild.rank();

        if !all_match_expected_rank(cards, game_state, rank) {
            return Err(Error::NotAllSameRank);
        }

        let info = SetInfo::Normal { rank };

        Ok(Set {
            cards: cards.to_vec(),
            info,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn run_try_from_low_cards() -> Result<()> {
        let game_state = GameState::new(Rank::Six);
        assert_eq!(
            RunInfo::Normal {
                suit: Suit::Spade,
                low_rank: Rank::Three,
                high_rank: Rank::Five
            },
            Run::try_from(&cards_from_str("3S,4S,5S"), &game_state)?.info
        );
        Ok(())
    }

    #[test]
    fn run_try_from_middle_cards() -> Result<()> {
        let game_state = GameState::new(Rank::Six);
        assert_eq!(
            RunInfo::Normal {
                suit: Suit::Heart,
                low_rank: Rank::Seven,
                high_rank: Rank::Ten
            },
            Run::try_from(&cards_from_str("7H,8H,9H,10H"), &game_state)?.info
        );
        Ok(())
    }

    #[test]
    fn run_try_from_high_cards() -> Result<()> {
        let game_state = GameState::new(Rank::Six);
        assert_eq!(
            RunInfo::Normal {
                suit: Suit::Club,
                low_rank: Rank::Ten,
                high_rank: Rank::King
            },
            Run::try_from(&cards_from_str("10C,JC,QC,KC"), &game_state)?.info
        );
        Ok(())
    }

    #[test]
    fn run_try_from_joker() -> Result<()> {
        let game_state = GameState::new(Rank::Six);
        assert_eq!(
            RunInfo::Normal {
                suit: Suit::Diamond,
                low_rank: Rank::Five,
                high_rank: Rank::Seven
            },
            Run::try_from(&cards_from_str("5D,Joker,7D"), &game_state)?.info
        );
        Ok(())
    }

    #[test]
    fn run_try_from_game_state_wild() -> Result<()> {
        let game_state = GameState::new(Rank::Six);
        assert_eq!(
            RunInfo::Normal {
                suit: Suit::Star,
                low_rank: Rank::Nine,
                high_rank: Rank::Jack,
            },
            Run::try_from(&cards_from_str("9R,10R,6H"), &game_state)?.info
        );
        Ok(())
    }

    #[test]
    fn run_try_from_all_jokers() -> Result<()> {
        let game_state = GameState::new(Rank::Six);
        let run = Run::try_from(
            &cards_from_str("Joker,Joker,Joker"),
            &game_state,
        )?;
        assert_eq!(run.info, RunInfo::AllWilds);
        Ok(())
    }

    #[test]
    fn run_try_from_all_wilds() -> Result<()> {
        let game_state = GameState::new(Rank::Six);
        let run = Run::try_from(
            &cards_from_str("Joker,Joker,6D"),
            &game_state,
        )?;
        assert_eq!(run.info, RunInfo::AllWilds);
        Ok(())
    }

    #[test]
    fn run_try_from_not_all_same_suit() {
        let game_state = GameState::new(Rank::Six);
        let result = Run::try_from(&cards_from_str("7H,8H,9D"), &game_state);
        assert_eq!(result, Err(Error::NotAllSameSuit));
    }

    #[test]
    fn run_try_from_not_all_in_order() {
        let game_state = GameState::new(Rank::Six);
        let result = Run::try_from(&cards_from_str("7H,8H,10H"), &game_state);
        assert_eq!(result, Err(Error::NotAllInOrder));
    }

    #[test]
    fn run_try_from_before_three() {
        let game_state = GameState::new(Rank::Six);
        let result = Run::try_from(&cards_from_str("Joker,3S,4S"), &game_state);
        assert_eq!(result, Err(Error::OutOfRange));
    }

    #[test]
    fn run_try_from_after_king() {
        let game_state = GameState::new(Rank::Six);
        let result = Run::try_from(&cards_from_str("QD,KD,Joker"), &game_state);
        assert_eq!(result, Err(Error::OutOfRange));
    }

    #[test]
    fn run_try_from_too_few_cards() {
        let game_state = GameState::new(Rank::Six);
        let result = Run::try_from(&cards_from_str("4H,5H"), &game_state);
        assert_eq!(result, Err(Error::TooFewCards));
    }

    #[test]
    fn run_try_from_too_many_cards() {
        let game_state = GameState::new(Rank::Six);
        let result = Run::try_from(
            &cards_from_str("3H,4H,5H,6H,7H,8H,9H,10H,JH,QH,KH,Joker"),
            &game_state,
        );
        assert_eq!(result, Err(Error::TooManyCards));
    }

    #[test]
    fn set_try_from_low_cards() -> Result<()> {
        let game_state = GameState::new(Rank::Six);
        assert_eq!(
            SetInfo::Normal { rank: Rank::Three },
            Set::try_from(&cards_from_str("3S,3H,3D"), &game_state)?.info
        );
        Ok(())
    }

    #[test]
    fn set_try_from_high_cards() -> Result<()> {
        let game_state = GameState::new(Rank::Six);
        assert_eq!(
            SetInfo::Normal { rank: Rank::King },
            Set::try_from(&cards_from_str("KR,KD,KS"), &game_state)?.info
        );
        Ok(())
    }

    #[test]
    fn set_try_from_middle_cards() -> Result<()> {
        let game_state = GameState::new(Rank::Six);
        assert_eq!(
            SetInfo::Normal { rank: Rank::Ten },
            Set::try_from(&cards_from_str("10S,10R,10D"), &game_state)?.info
        );
        Ok(())
    }

    #[test]
    fn set_try_from_repeated_suit() -> Result<()> {
        let game_state = GameState::new(Rank::Six);
        assert_eq!(
            SetInfo::Normal { rank: Rank::Five },
            Set::try_from(&cards_from_str("5S,5D,5D"), &game_state)?.info
        );
        Ok(())
    }

    #[test]
    fn set_try_from_joker() -> Result<()> {
        let game_state = GameState::new(Rank::Six);
        assert_eq!(
            SetInfo::Normal { rank: Rank::Queen },
            Set::try_from(&cards_from_str("Joker,QD,QR"), &game_state)?.info
        );
        Ok(())
    }

    #[test]
    fn set_try_from_game_state_wild() -> Result<()> {
        let game_state = GameState::new(Rank::Six);
        assert_eq!(
            SetInfo::Normal { rank: Rank::Queen },
            Set::try_from(&cards_from_str("6H,QD,QR"), &game_state)?.info
        );
        Ok(())
    }

    #[test]
    fn set_try_from_many_cards() -> Result<()> {
        let game_state = GameState::new(Rank::Six);
        assert_eq!(
            SetInfo::Normal { rank: Rank::Seven },
            Set::try_from(&cards_from_str("7D,7H,7R,Joker,7S,7D"), &game_state)?.info
        );
        Ok(())
    }

    #[test]
    fn set_try_from_all_jokers() -> Result<()> {
        let game_state = GameState::new(Rank::Six);
        let set = Set::try_from(
            &cards_from_str("Joker,Joker,Joker"),
            &game_state,
        )?;
        assert_eq!(set.info, SetInfo::AllWilds);
        Ok(())
    }

    #[test]
    fn set_try_from_all_wilds() -> Result<()> {
        let game_state = GameState::new(Rank::Six);
        let set = Set::try_from(
            &cards_from_str("Joker,Joker,6D"),
            &game_state,
        )?;
        assert_eq!(set.info, SetInfo::AllWilds);
        Ok(())
    }

    #[test]
    fn set_try_from_not_all_same_rank() {
        let game_state = GameState::new(Rank::Six);
        let result = Set::try_from(&cards_from_str("7H,7D,8D"), &game_state);
        assert_eq!(result, Err(Error::NotAllSameRank));
    }

    #[test]
    fn set_try_from_too_few_cards() {
        let game_state = GameState::new(Rank::Six);
        let result = Set::try_from(&cards_from_str("8D,8H"), &game_state);
        assert_eq!(result, Err(Error::TooFewCards));
    }

    fn cards_from_str(string: &str) -> Vec<Card> {
        use crate::hand::Hand;
        Hand::try_from(string).unwrap().cards().to_vec()
    }
}
