use crate::game::{Action, EndOfRoundAction};
use crate::game_state::GameState;
use crate::hand::Hand;
use crate::game::DrawLocation;
use crate::card::Card;
use crate::rank::Rank;

pub trait Player {
    fn start_game(&mut self);

    fn start_round(&mut self, start_round_info: StartRoundInfo, hand: Hand);

    fn other_player_draw(&mut self, draw_location: DrawLocation);

    fn other_player_turn(&mut self, action: Action);

    fn other_player_last_turn(&mut self, action: EndOfRoundAction);

    fn your_draw(&mut self) -> DrawLocation;

    fn your_turn(&mut self, card: Card) -> Action;

    fn your_last_turn(&mut self, card: Card) -> EndOfRoundAction;
}

pub struct StartRoundInfo {
    pub game_state: GameState,
}

pub struct DummyPlayer {
    pub hand: Hand,
    pub game_state: GameState,
}

impl Player for DummyPlayer {
    fn start_game(&mut self) {}

    fn start_round(&mut self, start_round_info: StartRoundInfo, hand: Hand) {
        self.hand = hand;
        self.game_state = start_round_info.game_state;
    }

    fn other_player_draw(&mut self, draw_location: DrawLocation) {}

    fn other_player_turn(&mut self, action: Action) {}

    fn other_player_last_turn(&mut self, action: EndOfRoundAction) {}

    fn your_draw(&mut self) -> DrawLocation {
        DrawLocation::DrawPile
    }

    fn your_turn(&mut self, card: Card) -> Action {
        self.hand.cards.push(card);
        Action::Discard(self.hand.cards.remove(0))
    }

    fn your_last_turn(&mut self, card: Card) -> EndOfRoundAction {
        self.hand.cards.push(card);
        let discard = self.hand.cards.remove(0);
        let remaining = self.hand.cards.clone();
        EndOfRoundAction {
            discard,
            go_out: Vec::new(),
            remaining,
        }
    }
}
