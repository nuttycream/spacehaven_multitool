#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    //Init logger, should not panic.
    spacehaven_multitool::logger::init().unwrap();

    eframe::run_native(
        "SpaceHaven Multi-Tool",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(spacehaven_multitool::App::new(cc))),
    )
}
