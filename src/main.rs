mod ui;
use ui::ciunni_app::Ciunni;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "File Selector App",
        options,
        Box::new(|_cc| Ok(Box::new(Ciunni::default()))),
    )
}