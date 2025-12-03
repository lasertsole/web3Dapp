use std::fmt::Debug;
use crate::user::user::User;
use std::any::Any;
use std::collections::HashSet;

// 游戏物品trait
pub trait GameItem: Debug + Any + Send + Sync{}

// 游戏规则trait
pub trait GameRule: Debug + Send + Sync {

    // 这是一个方法签名：它接受 self 的不可变引用，不返回任何值
    fn compare(&self, src: &dyn GameItem, tar: &dyn GameItem) -> bool;

    // Trait 也可以提供默认实现
    fn allocate(&self, users:&Vec<User>, game_item:&Vec<&dyn GameItem>,allocate_rule:&dyn Fn(&Vec<User>, &Vec<&dyn GameItem>) -> ()) -> (){
        allocate_rule(users, game_item);
    }
}

// 游戏对局
#[derive(Debug)]
pub struct Game {
    participants : Vec<&'static User>,
    game_rule: &'static dyn GameRule,
}

impl Game {
    pub fn new(participants: Vec<&'static User>, game_rule: &'static (dyn GameRule)) -> Game {
        Game{participants, game_rule}
    }
}

// 游戏调度器
#[derive(Debug)]
pub struct GamesScheduler {
    game_set: HashSet<Game>,
    participant_set: HashSet<User>
}

impl GamesScheduler {
    //控制为单例模式
    pub(self) fn new(game_set: HashSet<Game>, participant_set: HashSet<User>) -> GamesScheduler {
        GamesScheduler{game_set, participant_set}
    }

    pub fn get_game_set(&self) -> &HashSet<Game> {
        &self.game_set
    }

    pub fn get_participant_set(&self) -> &HashSet<User> {
        &self.participant_set
    }
}

lazy_static! {
    //控制为单例模式
    pub static ref GLOBAL_GAMES_SCHEDULER: GamesScheduler = GamesScheduler::new(HashSet::new(), HashSet::new());
}
