use crate::card::Card;
use crate::game_state::GameState;
use crate::rank::Rank;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Game {
    players: Vec<Player>,
    deck: Vec<Card>,
    discard_pile: Vec<Card>,
    state: GameState,
}

struct Player {
    hand: Vec<Card>,
    score: usize,
}

enum DrawLocation {
    DiscardPile,
    DrawPile,
}

impl Game {
    pub fn new(num_players: usize) -> Self {
        let mut deck = crate::card::full_deck().collect::<Vec<_>>();
        let mut rng = thread_rng();
        deck.shuffle(&mut rng);
        let players = (0..num_players).map(|_| Player::new()).collect();
        let discard_pile = Vec::new();
        let state = GameState::new(Rank::Three);
        Game {
            players,
            deck,
            discard_pile,
            state,
        }
    }

    fn deal(&mut self) {
        let num_cards = self.state.wild_rank().number();
        for player in self.players.iter_mut() {
            let index = self.deck.len() - num_cards;
            player.hand.extend(self.deck.drain(index..));
        }
    }

    pub fn debug_strings(&self) -> Vec<String> {
        let mut strings = Vec::new();
        strings.push(format!("Round: {} cards", self.state.wild_rank().number()));
        strings.push("Players:".to_string());
        for player in self.players.iter() {
            strings.push(player.debug_string());
        }
        strings.push(format!(
            "Discard Pile: {}",
            pretty_cards(&self.discard_pile)
        ));
        strings.push(format!("Deck: {}", pretty_cards(&self.deck)));
        strings
    }
}

impl Player {
    pub fn new() -> Self {
        Player {
            hand: Vec::new(),
            score: 0,
        }
    }

    pub fn debug_string(&self) -> String {
        format!(
            "\t{} points, cards: {}",
            self.score,
            pretty_cards(&self.hand),
        )
    }
}

fn pretty_cards(cards: &[Card]) -> String {
    cards
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
