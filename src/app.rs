use crate::ui::{file_selector, hex_viewer};
use egui::Context;
use std::path::PathBuf;

pub struct Ciunni {
    pub selected_file: Option<PathBuf>,
    pub file_bytes: Option<Vec<u8>>,
    pub error_message: Option<String>,
}

impl Default for Ciunni {
    fn default() -> Self {
        Self {
            selected_file: None,
            file_bytes: None,
            error_message: None,
        }
    }
}

impl Ciunni {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for Ciunni {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Ciunni Hex Viewer");
                file_selector::ui(self, ui);
                ui.add_space(10.0);

                let available_height = ui.available_height();
                egui::ScrollArea::vertical()
                    .max_height(available_height)
                    .show(ui, |ui| {
                        hex_viewer::ui(self, ui);
                    });
            });
        });
    }
}
