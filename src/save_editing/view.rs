use crate::config::Config;

use super::{gamesave::GameSave, parser::read_save, writer::write_save};

#[derive(Default)]
pub struct SaveEditingView {
    gamesave: GameSave,
}

impl SaveEditingView {
    pub fn new(&mut self, config: &Config) {
        
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Load Test").clicked() {
                self.gamesave = load_save();
            }
            if ui.button("Save Test").clicked() {
                save(&self.gamesave);
            }
        });
    }
}

fn load_save() -> GameSave {
    let start_time = std::time::Instant::now();
    let result = match read_save(
        "A Small Hope-1".to_string(),
        std::path::Path::new(
            r"C:\Program Files (x86)\Steam\steamapps\common\SpaceHaven\savegames\Basic Platform-1\save",
        ),
    ) {
        Ok(save_game) => {
            log::info!("Successful read ({} ms)", start_time.elapsed().as_millis());
            save_game
        }
        Err(err) => {
            log::error!(
                "Failed read ({} ms) Error: {:?}",
                start_time.elapsed().as_millis(),
                err
            );
            GameSave::default()
        }
    };
    result
}

fn save(gamesave: &GameSave) {
    let start_time = std::time::Instant::now();
    match write_save(gamesave) {
        Ok(()) => {
            log::info!("Successful save ({} ms)", start_time.elapsed().as_millis());
        }
        Err(err) => {
            log::error!(
                "Failed to save ({} ms) Error: {:?}",
                start_time.elapsed().as_millis(),
                err
            );
        }
    }
}
