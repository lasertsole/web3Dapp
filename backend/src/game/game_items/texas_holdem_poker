use std::fmt::{Debug};
use lazy_static::lazy_static;
use crate::game::game::{GameItem, GameRule};
use crate::user::user::User;

#[derive(Debug)]
pub struct TexasHoldemPokerGameRules {}

impl GameRule for TexasHoldemPokerGameRules {
    fn compare(&self, src: Vec<&dyn GameItem>, tar: Vec<&dyn GameItem>) -> bool {
        return true;
    }

    fn allocate(&self, users: &Vec<User>, game_item: &Vec<&dyn GameItem>, allocate_rule: &dyn Fn(&Vec<User>, &Vec<&dyn GameItem>) -> ()) -> () {
        todo!()
    }
}

lazy_static! {

}
