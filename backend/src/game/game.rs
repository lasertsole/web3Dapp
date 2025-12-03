use std::collections::HashSet;
use std::fmt::Debug;
use crate::user::user::User;

// 游戏规则trait
pub trait GameRule<T> where T: Debug + Clone + Sized  {

    // 这是一个方法签名：它接受 self 的不可变引用，不返回任何值
    fn compare(&self, src: T, tar: T) -> bool;

    // Trait 也可以提供默认实现
    fn allocate(&self, users:&Vec<User>, game_item:&Vec<T>,allocate_rule:Box<dyn Fn(&Vec<User>, &Vec<T>) -> ()>) -> (){
        allocate_rule(users, game_item);
    }
}

// 游戏对局
pub struct Game<'a, T> {
    participants : Vec<&'a User>,
    game_rule: Box<dyn GameRule<T>>,
}

impl<'a, T> Game<'a, T> {
    pub fn new(participants: Vec<&User>, game_rule: Box<dyn GameRule<T>>) -> Game<T> {
        Game{participants, game_rule}
    }
}

// 游戏调度器
pub struct GamesScheduler<'a, T> where T: Debug + Clone + Sized {
    game_set: HashSet<Game<'a, T>>,
    participant_set: HashSet<&'a User>
}
