use std::error::Error;

use super::extract::extract;

#[derive(Default)]
pub struct ModdingView {

}

impl ModdingView {
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Extract test").clicked() {
                dothething();
            }
        });
    }
}

fn dothething() {
    match extract() {
        Ok(_) => {},
        Err(e) => log::info!("{:?}", e),
    }
    
}