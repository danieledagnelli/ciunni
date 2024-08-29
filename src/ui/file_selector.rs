use crate::Ciunni;
use egui::Ui;
use std::fs;
use std::path::PathBuf;

pub fn ui(app: &mut Ciunni, ui: &mut Ui) {
    ui.horizontal(|ui| {
        if ui.button("Select File").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                match fs::read(&path) {
                    Ok(bytes) => {
                        app.selected_file = Some(path);
                        app.file_bytes = Some(bytes);
                        app.error_message = None;
                    }
                    Err(e) => {
                        app.error_message = Some(format!("Error reading file: {}", e));
                    }
                }
            }
        }

        if let Some(path) = &app.selected_file {
            ui.label(path.to_string_lossy());
        }
    });

    if let Some(error) = &app.error_message {
        ui.colored_label(egui::Color32::RED, error);
    }
}
