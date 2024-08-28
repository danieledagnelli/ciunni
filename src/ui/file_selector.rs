use crate::Ciunni;
use egui::Ui;
use std::fs;

pub fn ui(ciunni: &mut Ciunni, ui: &mut Ui) {
    ui.horizontal(|ui| {
        if ui.button("Select File").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                match fs::read(&path) {
                    Ok(bytes) => {
                        ciunni.selected_file = Some(path.clone());
                        ciunni.file_bytes = Some(bytes);
                        ui.memory_mut(|mem| {
                            mem.data.insert_persisted(
                                egui::Id::new("file_path"),
                                path.to_string_lossy().to_string(),
                            )
                        });
                    }
                    Err(e) => {
                        eprintln!("Error reading file: {}", e);
                        ui.memory_mut(|mem| {
                            mem.data
                                .insert_persisted(egui::Id::new("file_error"), e.to_string())
                        });
                    }
                }
            }
        }

        // Display selected file path
        if let Some(path) = &ciunni.selected_file {
            ui.label(path.to_string_lossy());
        }
    });

    // Display error message
    let error = ui.memory_mut(|mem| {
        mem.data
            .get_persisted::<String>(egui::Id::new("file_error"))
            .map(|s| s.clone())
    });
    if let Some(error) = error {
        ui.colored_label(egui::Color32::RED, error);
        if ui.button("Clear Error").clicked() {
            ui.memory_mut(|mem| mem.data.remove::<String>(egui::Id::new("file_error")));
        }
    }
}
