// src/ciunni.rs

use crate::ui::{file_selector, hex_viewer};
use egui::Context;
use std::path::PathBuf;

pub struct Ciunni {
    pub selected_file: Option<PathBuf>,
    pub file_bytes: Option<Vec<u8>>,
}

impl Ciunni {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for Ciunni {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            file_selector::ui(self, ui);
            hex_viewer::ui(self, ui);
        });
    }
}

impl Default for Ciunni {
    fn default() -> Self {
        Self {
            selected_file: None,
            file_bytes: None,
        }
    }
}
