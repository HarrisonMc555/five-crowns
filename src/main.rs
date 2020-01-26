mod card;
mod game;
mod game_state;
mod hand;
mod rank;
mod score;
mod score_group;
mod suit;
mod utils;

fn main() {
    println!(
        "{:?}",
        card::full_deck()
            .map(|card| card.to_string())
            .collect::<Vec<_>>()
    );
    println!(
        "{:?}",
        card::full_deck()
            .map(|card| card.score().to_string())
            .collect::<Vec<_>>()
    );
}
