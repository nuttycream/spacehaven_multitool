use egui::{RichText, Color32};

use crate::{ModdingView, SaveEditingView};

#[derive(Default)]
pub struct SaveEditingTab {
    save_editing: SaveEditingView,
}

impl eframe::App for SaveEditingTab {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.save_editing.ui(ctx);
    }
}

#[derive(Default)]
pub struct ModdingTab<'a> {
    modding: ModdingView<'a>,
}

impl eframe::App for ModdingTab<'_> {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.modding.ui(ctx, frame);
    }
}

#[derive(Default)]
struct State<'a> {
    save_editing: SaveEditingTab,
    modding: ModdingTab<'a>,

    selected_anchor: String,
    //settings_window: SettingsWindow,
}

#[derive(Default)]
pub struct App<'a> {
    state: State<'a>,
}

impl eframe::App for App<'_> {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.state.selected_anchor.is_empty() {
            let selected_anchor = self.apps_iter_mut().next().unwrap().0.to_owned();
            self.state.selected_anchor = selected_anchor;
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                self.bar_contents(ui, frame);
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel")
            .height_range(180.0..=180.0)
            .show(ctx, |ui| {
                crate::logger::logger_ui(ui);
            });

        self.show_selected_tool(ctx, frame);
    }
}

impl App<'_> {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn apps_iter_mut(&mut self) -> impl Iterator<Item = (&str, &str, &mut dyn eframe::App)> {
        vec![
            (
                "Save Editor",
                "save_editor",
                &mut self.state.save_editing as &mut dyn eframe::App,
            ),
            (
                "Mod Manager",
                "modding",
                &mut self.state.modding as &mut dyn eframe::App,
            ),
        ]
        .into_iter()
    }

    fn show_selected_tool(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut found_anchor = false;
        let selected_anchor = self.state.selected_anchor.clone();
        for (_name, anchor, app) in self.apps_iter_mut() {
            if anchor == selected_anchor || ctx.memory(|mem| mem.everything_is_visible()) {
                app.update(ctx, frame);
                found_anchor = true;
            }
        }

        if !found_anchor {
            self.state.selected_anchor = "save_editor".into();
        }
    }

    fn bar_contents(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut selected_anchor = self.state.selected_anchor.clone();
        for (name, anchor, _app) in self.apps_iter_mut() {
            if ui
                .selectable_label(selected_anchor == anchor, name)
                .clicked()
            {
                selected_anchor = anchor.to_owned();
            }
        }
        self.state.selected_anchor = selected_anchor;

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.hyperlink_to(
                env!("CARGO_PKG_VERSION"),
                "https://github.com/nuttycream/spacehaven-multitool",
            )
            .on_hover_ui(|ui| {
                ui.label("Check for Updates");
            });

            ui.separator();

            if ui
                .selectable_label(false, "🗄")
                .on_hover_text("Organize Windows")
                .clicked()
            {
                ui.ctx().memory_mut(|mem| mem.reset_areas());
            }

            ui.separator();

            if ui.button("LAUNCH GAME").clicked() {

            }

            ui.separator();

            ui.label(RichText::new("unmodified").color(Color32::YELLOW));

            //ui.toggle_value(&mut self.state.settings_window.show_window, "🔧")
            //.on_hover_text("Settings");
        });
    }
}
