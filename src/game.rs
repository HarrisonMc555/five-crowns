use crate::card::Card;
use crate::game_state::GameState;
use crate::rank::Rank;
use crate::score::Score;
use crate::score_group::ScoreGroup;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Game {
    players: Vec<Player>,
    current_player_index: usize,
    first_player_gone_out_index: Option<usize>,
    deck: Vec<Card>,
    discard_pile: Vec<Card>,
    state: GameState,
}

struct Player {
    hand: Vec<Card>,
    score: Score,
}

pub enum DrawLocation {
    DiscardPile,
    DrawPile,
}

pub enum Action {
    Discard(Card),
    GoOut(Card, Vec<ScoreGroup>),
}

pub struct EndOfRoundAction {
    go_out: Vec<ScoreGroup>,
    remaining: Vec<Card>,
}

impl Game {
    pub fn new(num_players: usize) -> Self {
        let mut deck = crate::card::full_deck().collect::<Vec<_>>();
        let mut rng = thread_rng();
        deck.shuffle(&mut rng);
        let players = (0..num_players).map(|_| Player::new()).collect();
        let discard_pile = Vec::new();
        let state = GameState::new(Rank::Three);
        let mut game = Game {
            players,
            current_player_index: 0,
            first_player_gone_out_index: None,
            deck,
            discard_pile,
            state,
        };
        game.deal();
        game.discard_pile
            .push(game.deck.pop().expect("Empty deck after dealing"));
        game
    }

    pub fn draw(&mut self, location: DrawLocation) -> Card {
        let card = match location {
            DrawLocation::DrawPile => self.next_card_from_deck(),
            DrawLocation::DiscardPile => self.next_card_from_discard_pile(),
        };
        self.players[self.current_player_index].hand.push(card);
        card
    }

    // pub fn turn(&mut self, discard: Card, going_out: Option<Vec<ScoreGroup>>) {
    pub fn turn(&mut self, action: Action) {
        let discard = match action {
            Action::Discard(discard) => discard,
            Action::GoOut(discard, _) => discard,
        };
        self.discard_pile.push(discard);
        if let Action::GoOut(_, score_groups) = action {
            self.first_player_gone_out_index = Some(self.current_player_index);
            // Add score
            // If first player out, it needs to include all cards
            // If not first player out, there can be remaining cards
        }
    }

    pub fn last_turn(&mut self, action: EndOfRoundAction) {
        let points = action.remaining.iter().map(|c| c.score()).sum::<Score>();
        self.players[self.current_player_index].score += points;
    }

    /// Return the next card from the deck. If the deck is empty, the discard pile (minus the top
    /// card) is shuffled to become the new deck.
    fn next_card_from_deck(&mut self) -> Card {
        if let Some(card) = self.deck.pop() {
            return card;
        }
        let mut bottom_discarded_cards = self
            .discard_pile
            .drain(0..self.discard_pile.len() - 1)
            .collect::<Vec<_>>();
        let mut rng = thread_rng();
        bottom_discarded_cards.shuffle(&mut rng);
        self.deck = bottom_discarded_cards;
        self.deck
            .pop()
            .expect("Empty deck and less than one discarded card")
    }

    fn next_card_from_discard_pile(&mut self) -> Card {
        self.discard_pile.pop().expect("Empty discard pile")
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
            score: Score::new(),
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
