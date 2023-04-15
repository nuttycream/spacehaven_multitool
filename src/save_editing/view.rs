use super::{parser::read_save, gamesave::GameSave};

#[derive(Default)]
pub struct SaveEditingView {

}

impl SaveEditingView {
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            if ui.button("Test").clicked() {
                let _gamesave = load_save();
            }
        });
    }
}

fn load_save() -> GameSave {
    let start_time = std::time::Instant::now();
    let result = match read_save("A Small Hope-1".to_string(), std::path::Path::new(r"C:\Program Files (x86)\Steam\steamapps\common\SpaceHaven\savegames\A Small Hope-1\save")) {
        Ok(save_game) => {
            log::info!("Successful read ({} ms)", start_time.elapsed().as_millis());
            save_game
        }
        Err(err) => {
            log::error!("Failed read ({} ms) Error: {:?}", start_time.elapsed().as_millis(), err);
            GameSave::default()
        }
    };
    result

}
