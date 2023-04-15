#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod modding;
mod save_editing;

pub mod logger;
pub mod utils;

pub use app::App;
pub use modding::view::ModdingView;
pub use save_editing::view::SaveEditingView;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}
