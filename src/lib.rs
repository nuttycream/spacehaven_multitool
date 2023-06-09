#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod modding;
mod save_editing;
mod config;

pub mod logger;
pub mod utils;

pub use config::Config;
pub use app::App;
pub use modding::view::ModdingView;
pub use save_editing::view::SaveEditingView;


//TODO: Implement Config
pub struct GameInfo {
    game_path: std::path::PathBuf, //jarpath, game root folder
    version: String,
}

