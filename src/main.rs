use ciunni::Ciunni;

fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Ciunni Hex Viewer",
        options,
        Box::new(|cc| Ok(Box::new(Ciunni::new(cc)))),
    )
}
