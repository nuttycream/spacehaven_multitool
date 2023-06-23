use super::database::{load_mods, Mod};
use super::extract::extract;
use super::merge::init_mods;
use super::texture::explode;

#[derive(Default)]
pub struct ModdingView<'a> {
    mods: Vec<Mod>,
    selected_mod: Option<&'a Mod>,
}

impl ModdingView<'_> {
    pub fn ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left panel")
            .default_width(125.0)
            .min_width(125.0)
            .show(ctx, |ui| {
                ui.heading("Mods");
                ui.separator();
                for current_mod in &self.mods {
                    let is_selected = self
                        .selected_mod
                        .as_ref()
                        .map(|m| current_mod.name == m.name)
                        .unwrap_or(false);

                    if ui
                        .selectable_label(is_selected, &current_mod.name)
                        .clicked()
                    {
                        self.selected_mod = Some(current_mod);
                    }
                }
            });

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("extract test").clicked() {
                    extract_test();
                }
                if ui.button("explode test").clicked() {
                    explode_test();
                }
                if ui.button("load mods test").clicked() {
                    self.selected_mod = None;
                    self.mods = load_mods_test();
                    
                }

                if ui.button("merge test").clicked() {
                    merge_mods_test(&self.mods);
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Enable").clicked() {}
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                if let Some(selected_mod) = &self.selected_mod {
                    columns[0].vertical(|ui| {
                        ui.heading(&selected_mod.name);
                        ui.label(
                            &selected_mod
                                .author
                                .to_owned()
                                .unwrap_or("No Author".to_string()),
                        );
                    });
                    columns[1].vertical(|ui| {
                        if let Some(preview) = &selected_mod.preview {
                            preview.show_max_size(ui, ui.available_size());
                        } else {
                            ui.label("No Preview Available");
                        }
                        ui.separator();

                        let description = selected_mod
                            .description
                            .clone()
                            .unwrap_or("No description".to_string());
                        egui::ScrollArea::vertical()
                            .auto_shrink([true; 2])
                            .show_viewport(ui, |ui, _viewport| {
                                selectable_text(ui, &description);
                            });
                    });
                }
            });
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

fn load_mods_test() -> Vec<Mod> {
    match load_mods() {
        Ok(mods) => mods,
        Err(e) => {
            log::error!("{:?}", e);
            Vec::new()
        }
    }
}

fn merge_mods_test(active_mods: &Vec<Mod>) {
    let start = std::time::Instant::now();

    match init_mods(active_mods) {
        Ok(_) => {
            let duration = start.elapsed().as_millis();
            log::info!("Merge Completed in {} ms", duration);
        }
        Err(e) => {
            log::error!("{:?}", e);
        }
    }


    
}

fn selectable_text(ui: &mut egui::Ui, mut text: &str) {
    ui.add(
        egui::TextEdit::multiline(&mut text)
            .desired_width(f32::INFINITY)
            .font(egui::TextStyle::Monospace),
    );
}
