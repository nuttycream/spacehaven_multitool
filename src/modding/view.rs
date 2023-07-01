use egui_extras::{Column, TableBuilder};

use super::database::{load_mods, Mod};
use super::extract::extract;
use super::merge::init_mods;
use super::texture::explode;

#[derive(Default)]
pub struct ModdingView {
    mods: Vec<Mod>,
    selected_mod: Option<usize>,
}

impl ModdingView {
    pub fn new(&mut self, config: &crate::config::Config) {
        self.mods = load_mods_test();
    }

    pub fn ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left panel")
            .default_width(125.0)
            .min_width(125.0)
            .show(ctx, |ui| {
                ui.heading("Mods");
                ui.separator();

                for (index, current_mod) in self.mods.iter().enumerate() {
                    let is_selected = self.selected_mod.map(|i| index == i).unwrap_or(false);

                    if ui
                        .selectable_label(is_selected, &current_mod.name)
                        .clicked()
                    {
                        self.selected_mod = Some(index);
                    }
                }
            });

        egui::TopBottomPanel::top("tops").show(ctx, |ui| {
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
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {
            if let Some(index) = self.selected_mod {
                let selected_mod = &mut self.mods[index];
                egui::TopBottomPanel::bottom("mod-bottom").show(ctx, |ui| {
                    ui.add_space(2.5);
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            if ui.small_button("Reset To Default").clicked() {
                                selected_mod.reset_to_defaults();
                                
                            }
                            ui.separator();
                            if ui.small_button("Save To File").clicked() {
                                selected_mod.save_to_file();
                            }
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("NexusMods").clicked() {}
                            if ui.small_button("GitHub").clicked() {}
                        });
                    });
                });
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.columns(2, |columns| {
                        columns[0].vertical(|ui| {
                            let ver = &selected_mod.version.clone().unwrap_or("1".to_string());
                            ui.heading(format!("{} ver: {}", &selected_mod.name, ver));
                            ui.horizontal(|ui| {
                                ui.label(
                                    &selected_mod
                                        .author
                                        .to_owned()
                                        .unwrap_or("No Author".to_string()),
                                );
                                ui.label(format!("Mod ID: {}", &selected_mod.prefix.to_string()));
                                let vers = selected_mod
                                    .supported_game_vers
                                    .iter()
                                    .map(|version| version.to_string())
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                ui.label(format!("Game Versions: {}", vers));
                            });
                            ui.separator();
                            egui::ScrollArea::vertical()
                                .id_source("config-scroll-area")
                                .auto_shrink([true; 2])
                                .show_viewport(ui, |ui, _viewport| {
                                    show_config(ui, selected_mod);
                                });
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
                                .id_source("desc-scroll-area")
                                .auto_shrink([false; 2])
                                .show_viewport(ui, |ui, _viewport| {
                                    selectable_text(ui, &description);
                                });
                        });
                    });
                });
            }
        });
    }
}

fn extract_test() {
    let start = std::time::Instant::now();
    match extract() {
        Ok(_) => {
            let duration = start.elapsed().as_millis();
            log::info!("Extract Completed in {} ms", duration);
        }
        Err(e) => log::info!("{:?}", e),
    }
}

fn explode_test() {
    let start = std::time::Instant::now();
    match explode() {
        Ok(_) => {
            let duration = start.elapsed().as_millis();
            log::info!("Explode Completed in {} ms", duration);
        }
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

fn show_config(ui: &mut egui::Ui, selected_mod: &mut Mod) {
    if let Some(mod_config) = &mut selected_mod.config_variables {
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .auto_shrink([true; 2])
            .column(Column::remainder())
            .column(Column::auto().resizable(false))
            .cell_layout(egui::Layout::centered_and_justified(
                egui::Direction::LeftToRight,
            ))
            .min_scrolled_height(0.0);

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Variable Name");
                });
                header.col(|ui| {
                    ui.strong("Variable Value");
                });
            })
            .body(|mut body| {
                for (_, config_var) in mod_config.iter_mut() {
                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            ui.label(config_var.text.to_string());
                        });
                        row.col(|ui| {
                            match &mut config_var.var_type {
                                super::database::ConfigVarValue::Str(var) => {
                                    ui.text_edit_singleline(var);
                                }
                                super::database::ConfigVarValue::Int(var) => {
                                    ui.add(egui::DragValue::new(var).speed(1.0));
                                }
                                super::database::ConfigVarValue::Float(var) => {
                                    ui.add(egui::DragValue::new(var).speed(1.0));
                                }
                                super::database::ConfigVarValue::Bool(var) => {
                                    let toggle_text = if *var { "Enabled" } else { "Disabled" };
                                    if ui.toggle_value(var, toggle_text).clicked() {}
                                }
                            };
                        });
                    });
                }
            });
    } else {
        ui.heading("No Mod Config Variables Available");
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
        egui::TextEdit::multiline(&mut text).desired_width(f32::INFINITY), //.font(egui::TextStyle::Monospace),
    );
}
