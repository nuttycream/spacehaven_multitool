#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    //Init logger, should not panic.
    spacehaven_multitool::logger::init().unwrap();
    let native_options = eframe::NativeOptions {
        min_window_size: Some([900.0, 720.0].into()),
        ..Default::default()
    };

    eframe::run_native(
        "SpaceHaven Multi-Tool",
        native_options,
        Box::new(|cc| Box::new(spacehaven_multitool::App::new(cc))),
    )
}


