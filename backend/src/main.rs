#[macro_use]
extern crate lazy_static;
mod user;
mod poker;
mod game;

fn main() {
    println!("{:?} ", *poker::poker::ALL_CARDS);
}
