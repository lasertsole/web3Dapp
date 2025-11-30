use anchor_lang::prelude::*;
use gambling::poker;
fn main() {
    poker::poker::get_whole_deck_of_cards();
    println!("Hello, world!");
}
