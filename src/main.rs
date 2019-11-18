mod card;
mod game;
mod rank;
mod suit;

fn main() {
    println!(
        "{:?}",
        card::full_deck()
            .map(|card| card.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
}
