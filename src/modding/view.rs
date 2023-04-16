

#[derive(Default)]
pub struct ModdingView {

}

impl ModdingView {
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Big Text");
        });
    }
    
}