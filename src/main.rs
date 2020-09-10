#![allow(unused_imports, unused_variables, dead_code)]

mod card;
mod game;
mod game_state;
mod hand;
mod player;
mod rank;
mod score;
mod score_group;
mod suit;
mod utils;

fn main() {
    for card in card::full_deck() {
        println!("{}: {}", card, card.score())
    }

    let game_state = game_state::GameState::new(rank::Rank::Six);
    for hand_str in ["JS,QS,KS", "4H,4D,Joker", "3H,4D,5H"].iter() {
        println!("{}:", hand_str);
        for string in hand_info(hand_str, &game_state) {
            println!("\t{}", string);
        }
    }

    println!();
    let mut game = game::Game::new(2);
    game.debug_print();
    game.draw(game::DrawLocation::DrawPile);
    game.debug_print();
    game.discard(game.cur_player().hand[0]).unwrap();
    game.debug_print();
}

fn hand_info(hand_str: &str, game_state: &game_state::GameState) -> Vec<String> {
    let hand = match hand::Hand::try_from(hand_str) {
        Some(h) => h,
        None => return vec!["Invalid hand".to_string()],
    };
    let set_result = score_group::Set::try_from(&hand.cards(), &game_state);
    let run_result = score_group::Run::try_from(&hand.cards(), &game_state);
    vec![
        format!("Set: {:?}", set_result),
        format!("Run: {:?}", run_result),
    ]
}
