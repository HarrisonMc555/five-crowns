mod card;
mod game;
mod rank;
mod score_group;
mod suit;

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
