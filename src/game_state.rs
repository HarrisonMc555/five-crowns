use crate::card::{Card, NormalCard};
use crate::rank::Rank;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct GameState {
    wild_rank: Rank,
}

impl GameState {
    pub fn new(wild_rank: Rank) -> GameState {
        GameState { wild_rank }
    }

    pub fn is_card_wild(&self, card: Card) -> bool {
        match card {
            Card::Normal(c) => self.is_rank_wild(c.rank()),
            Card::Joker => true,
        }
    }

    pub fn is_rank_wild(&self, rank: Rank) -> bool {
        self.wild_rank == rank
    }

    pub fn non_wild(&self, card: Card) -> Option<NormalCard> {
        card.normal()
            .filter(|card| !self.is_rank_wild(card.rank()))
            .cloned()
    }

    pub fn wild_rank(self) -> Rank {
        self.wild_rank
    }

    pub fn round_num(self) -> usize {
        self.wild_rank.number() - 2
    }

    pub fn num_cards(self) -> usize {
        self.wild_rank.number()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_round_num() {
        let game_state = GameState::new(Rank::Three);
        assert_eq!(game_state.round_num(), 1);
        let game_state = GameState::new(Rank::Ten);
        assert_eq!(game_state.round_num(), 8);
        let game_state = GameState::new(Rank::King);
        assert_eq!(game_state.round_num(), 11);
    }

    #[test]
    fn test_num_cards() {
        let game_state = GameState::new(Rank::Three);
        assert_eq!(game_state.num_cards(), 3);
        let game_state = GameState::new(Rank::Ten);
        assert_eq!(game_state.num_cards(), 10);
        let game_state = GameState::new(Rank::King);
        assert_eq!(game_state.num_cards(), 13);
    }
}
