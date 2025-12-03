use std::fmt::Debug;
use crate::user::user::User;
use std::any::Any;
use std::collections::HashSet;
use std::hash::Hash;

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

// 分别实现 Hash、PartialEq、Eq的trait，使dyn GameRule可比较哈希值，从而可以插入HashSet
impl Hash for Game {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // 哈希参与者
        self.participants.hash(state);
        // 哈希 game_rule 指针的地址
        std::ptr::hash(self.game_rule as *const _, state);
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        // 比较参与者列表
        self.participants == other.participants &&
            // 比较 game_rule 指针的地址，以判断是否是同一个实例
            std::ptr::eq(self.game_rule as *const _, other.game_rule as *const _)
    }
}

impl Eq for Game {}

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

    pub fn add_game(&mut self, game: Game) {
        self.game_set.insert(game);
    }

    pub fn add_participant(&mut self, user: User) {
        self.participant_set.insert(user);
    }

    pub fn remove_game(&mut self, game: &Game) {
        self.game_set.remove(game);
    }

    pub fn remove_participant(&mut self, user: &User) {
        self.participant_set.remove(user);
    }
}

lazy_static! {
    //控制为单例模式
    pub static ref GLOBAL_GAMES_SCHEDULER: GamesScheduler = GamesScheduler::new(HashSet::new(), HashSet::new());
}
