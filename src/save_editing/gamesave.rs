use std::collections::HashMap;

#[derive(Default)]
pub struct GameSave {
    pub name: String,
    pub path: std::path::PathBuf,
    pub date: String,

    pub bank: i32,

    pub ships: Vec<Ship>,
    pub factions: Vec<Faction>,
    pub research_tree: Vec<Tech>,
    pub game_settings: HashMap<String, String>,
}

pub struct Ship {
    pub name: String,
    pub owner: String,
    pub size_x: i32,
    pub size_y: i32,

    pub characters: Vec<Character>,

    pub item_storages: HashMap<i32, i32>,
    pub tool_storages: Vec<i32>,
}

pub struct Character {
    pub name: String,
    pub side: String,
    pub stats: HashMap<String, i32>,
    pub attributes: HashMap<i32, i32>,
    pub traits: Vec<i32>,
    pub skills: HashMap<i32, i32>,
}

pub struct Faction {
    pub name: String,
    pub relationships: Vec<Relationship>,
}

#[derive(Default)]
pub struct Relationship {
    pub name: String,
    pub amount: i32,
    pub patience: i32,
    pub stance: String,

    pub trade: bool,
    pub ship: bool,
    pub vision: bool,
}

#[derive(Default)]
pub struct Tech {
    pub id: i32,
    pub stages: HashMap<i32, Stage>,
}

#[derive(Default)]
pub struct Stage {
    //pub tasks: HashMap<i32, Task>,
    pub basic: i32,
    pub intermediate: i32,
    pub advanced: i32,

    pub is_done: bool,
}

