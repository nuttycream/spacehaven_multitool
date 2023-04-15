/*
Originally egui_logger.
https://github.com/RegenJacob/egui_logger
made some visual modifications and updated egui.
*/

use std::sync::Mutex;

use egui::Color32;
use log::SetLoggerError;

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= log::STATIC_MAX_LEVEL
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            //println!("{}: {}", record.level(), record.args());
            let mut log = LOG.lock().unwrap();

            let mut l: Vec<(log::Level, String)> = log.clone();
            l.push((record.level(), record.args().to_string()));

            *log = l;
        }
    }

    fn flush(&self) {}
}

/// Initilizes the global logger.
/// Should be called very early in the program
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&Logger).map(|()| log::set_max_level(log::LevelFilter::Info))
}

static LOG: Mutex<Vec<(log::Level, String)>> = Mutex::new(Vec::new());

static LOGGER_UI: once_cell::sync::Lazy<Mutex<LoggerUi>> =
    once_cell::sync::Lazy::new(Default::default);

struct LoggerUi {
    loglevels: [bool; log::Level::Trace as usize],
    copy_text: String,
    max_log_length: usize,
}

impl Default for LoggerUi {
    fn default() -> Self {
        Self {
            loglevels: [true, true, true, false, false],
            copy_text: String::new(),
            max_log_length: 1000,
        }
    }
}

impl LoggerUi {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let mut logs = LOG.lock().unwrap();

        logs.reverse();
        logs.truncate(self.max_log_length);
        logs.reverse();

        let mut logs_displayed: usize = 0;

        egui::ScrollArea::vertical()
            .auto_shrink([false, true])
            .max_height(ui.available_height() - 30.0)
            .stick_to_bottom(true)
            .show(ui, |ui| {
                logs.iter().for_each(|(level, string)| {
                    let string_format = format!("[{}]: {}", level, string);

                    if !(self.loglevels[*level as usize - 1]) {
                        return;
                    }

                    match level {
                        log::Level::Warn => {
                            ui.colored_label(Color32::YELLOW, string_format);
                        }
                        log::Level::Error => {
                            ui.colored_label(Color32::RED, string_format);
                        }
                        _ => {
                            ui.label(string_format);
                        }
                    }

                    self.copy_text += &format!("{string} \n");
                    logs_displayed += 1;
                });
            });

        ui.horizontal(|ui| {
            ui.label(format!("Log size: {}", logs.len()));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Clear").clicked() {
                    *logs = vec![];
                }
                if ui.button("Copy").clicked() {
                    ui.output_mut(|o| o.copied_text = self.copy_text.to_string());
                }
            });
        });

        // has to be cleared after every frame
        self.copy_text.clear();
    }
}

/// Draws the logger ui
/// has to be called after [`init()`](init());
pub fn logger_ui(ui: &mut egui::Ui) {
    let mut logger_ui = LOGGER_UI.lock().unwrap();

    logger_ui.ui(ui);
}
