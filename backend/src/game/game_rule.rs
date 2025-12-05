use std::any::Any;
use std::collections::HashMap;
use crate::game::game_item::GameItem;
use crate::game::game::GameState;
use crate::game::player::Player;
use std::fmt; // 引入 fmt 模块

/// 游戏规则
pub struct GameRule {
    pub compare:&'static mut dyn Fn(&Vec<&dyn GameItem>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>) -> bool,
    pub allocate:&'static mut dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>) -> (),
    pub game_start:&'static mut dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->(),
    pub game_progress:&'static mut dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->(),
    pub game_pause:&'static mut dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->(),
    pub game_resume:&'static mut dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->(),
    pub game_finish:&'static mut dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->(),
    pub game_wait_start:&'static mut dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->(),
    pub players_join: &'static mut dyn Fn(&Vec<&Player>, &Vec<&Player>, &Vec<&dyn GameItem>, GameState, &HashMap<String, Box<dyn Any + Send + Sync>>)->(),
    pub players_leave: &'static mut dyn Fn(&Vec<&Player>, &Vec<&Player>, &Vec<&dyn GameItem>, GameState, &HashMap<String, Box<dyn Any + Send + Sync>>)->()
}

impl GameRule {
    pub fn new(
        compare:&'static mut dyn Fn(&Vec<&dyn GameItem>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>) -> bool,
        allocate:&'static mut dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>) -> (),
        game_start:&'static mut dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->(),
        game_progress:&'static mut dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->(),
        game_pause:&'static mut dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->(),
        game_resume:&'static mut dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->(),
        game_finish:&'static mut dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->(),
        game_wait_start:&'static mut dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->(),
        players_join: &'static mut dyn Fn(&Vec<&Player>, &Vec<&Player>, &Vec<&dyn GameItem>, GameState, &HashMap<String, Box<dyn Any + Send + Sync>>)->(),
        players_leave: &'static mut dyn Fn(&Vec<&Player>, &Vec<&Player>, &Vec<&dyn GameItem>, GameState, &HashMap<String, Box<dyn Any + Send + Sync>>)->()
    ) -> Self{
        GameRule {
            compare,
            allocate,
            game_start,
            game_progress,
            game_pause,
            game_resume,
            game_finish,
            game_wait_start,
            players_join,
            players_leave
        }
    }
}

// 手动实现 Debug Trait
impl fmt::Debug for GameRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameRule")
            .field("compare", &"[FnMut compare]")
            .field("allocate", &"[FnMut allocate]")
            .field("game_start", &"[FnMut game_start]")
            .field("game_progress", &"[FnMut game_progress]")
            .field("game_pause", &"[FnMut game_pause]")
            .field("game_resume", &"[FnMut game_resume]")
            .field("game_finish", &"[FnMut game_finish]")
            .field("game_wait_start", &"[FnMut game_wait_start]")
            .field("players_join", &"[FnMut players_join]")
            .field("players_leave", &"[FnMut players_leave]")
            .finish()
    }
}