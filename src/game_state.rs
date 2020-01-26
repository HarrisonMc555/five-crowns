use crate::card::Card;
use crate::rank::Rank;

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
}
