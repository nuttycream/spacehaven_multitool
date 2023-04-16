#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod modding;
mod save_editing;

pub mod logger;
pub mod utils;

pub use app::App;
pub use modding::view::ModdingView;
pub use save_editing::view::SaveEditingView;

#[cfg(test)]
mod tests {
    use crate::save_editing;

    #[test]
    fn test_read_save() {
        let save_dir = std::env::current_dir()
            .unwrap()
            .join("src")
            .join("tests")
            .join("saves");

        for entry in std::fs::read_dir(&save_dir).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name().into_string().unwrap();

            if file_name.ends_with(".xml") {
                let save_name = file_name.clone();

                let result = save_editing::parser::read_save(save_name, &save_dir);

                assert!(result.is_ok());

                let game_save = result.unwrap();

                // Add additional tests for each field of the GameSave struct
                assert_eq!(game_save.name, file_name);
            }
        }
    } 
}