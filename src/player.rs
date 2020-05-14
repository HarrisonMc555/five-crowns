use crate::rank::Rank;

pub trait Player {
    fn start_game();

    fn start_round(start_round_info: StartRoundInfo);
}

pub struct StartRoundInfo {
    round_num: usize,
    num_cards: usize,
    wild_rank: Rank,
    
}

impl StartRoundInfo {
    pub fn round_num(&self) -> usize {
        self.round_num
    }

    pub fn num_cards(&self) -> usize {
        self.wild_rank().number()
    }

    pub fn wild_rank(&self) -> Rank {
        Rank::try_from_number(self.num_cards()).unwrap()
    }
}
