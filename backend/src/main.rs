#[macro_use]
extern crate lazy_static;
mod user;
mod poker;
mod game;

fn main() {
    println!("{:?} ", poker::poker::get_all_cards());
    println!("{:?} ", *game::game::GLOBAL_GAMES_SCHEDULER);
}
