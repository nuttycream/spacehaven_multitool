use super::extract::extract;
use super::texture::explode;
use super::database::load_mods;

#[derive(Default)]
pub struct ModdingView {}

impl ModdingView {
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Extract test").clicked() {
                extract_test();
            }
            if ui.button("Explode test").clicked() {
                explode_test();
            }
            if ui.button("Load mods test").clicked() {
                load_mods_test();
            }
        });
    }
}

fn extract_test() {
    match extract() {
        Ok(_) => {}
        Err(e) => log::info!("{:?}", e),
    }
}

fn explode_test() {
    match explode() {
        Ok(_) => {}
        Err(e) => log::info!("{:?}", e),
    }
}

fn load_mods_test() {
    match load_mods() {
        Ok(mods) => {
            for mod_ in mods {
                log::info!("{}", mod_.name);
            }
        }
        Err(e) => log::error!("{:?}", e),
    }
}


