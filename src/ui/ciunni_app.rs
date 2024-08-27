use std::path::PathBuf;

pub struct Ciunni {
    selected_file: Option<PathBuf>,
}

impl eframe::App for Ciunni {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File Selector");

            if ui.button("Select File").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.selected_file = Some(path);
                }
            }

            if let Some(path) = &self.selected_file {
                ui.label(format!("Selected file: {}", path.display()));
            }
        });
    }
}

impl Default for Ciunni {
    fn default() -> Self {
        Self {
            selected_file: None,
        }
    }
}