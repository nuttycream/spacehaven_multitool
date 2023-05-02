pub mod gamesave;
pub mod parser;
pub mod writer;
pub mod view;

#[cfg(test)]
mod tests {
    use crate::save_editing;

    #[test]
    fn test_read_save() {
        let save_dir = std::env::current_dir().unwrap().join("src/tests/saves");

        for entry in std::fs::read_dir(&save_dir).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name().into_string().unwrap();

            if file_name.ends_with(".xml") {
                let save_name = file_name.clone();

                let result = save_editing::parser::read_save(save_name, &save_dir);

                assert!(result.is_ok());

                let game_save = result.unwrap();

                // TODO: Add additional tests for each field of the GameSave struct
                assert_eq!(game_save.name, file_name);
            }
        }
    }

    #[test]
    fn test_write_save() {

    }
}
