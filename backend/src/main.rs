mod user;
mod game;
mod timer;
mod event;

fn main() {
    println!("{:?} ", game::game_items::poker::poker::get_all_cards());
}
