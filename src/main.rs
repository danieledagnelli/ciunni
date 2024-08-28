// main.rs
use ciunni::Ciunni;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Ciunni",
        native_options,
        Box::new(|cc| Ok(Box::new(Ciunni::new(cc)))),
    )
}
