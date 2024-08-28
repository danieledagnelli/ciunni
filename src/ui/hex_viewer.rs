use crate::Ciunni;
use egui::{RichText, ScrollArea, TextStyle, Ui};
use crate::utils::format_chunk;

fn display_hex_row(ui: &mut Ui, offset: usize, chunk: &[u8]) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(format!("{:08X}:", offset)).monospace());
        let (hex_string, ascii_string) = format_chunk(chunk);
        ui.label(RichText::new(hex_string).monospace());
        ui.label(RichText::new(ascii_string).monospace());
    });
}

pub fn ui(ciunni: &Ciunni, ui: &mut Ui) {
    if let Some(bytes) = &ciunni.file_bytes {
        ui.collapsing("File bytes", |ui| {
            scrollable_hex_view(ui, bytes);
        });
    }
}

fn scrollable_hex_view(ui: &mut Ui, bytes: &[u8]) {
    let (row_height, num_rows) = calculate_view_dimensions(ui, bytes);
    ScrollArea::vertical()
        .auto_shrink([false; 2]) // Prevent shrinking
        .max_height(f32::INFINITY) // Allow infinite height
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
        .show_rows(ui, row_height, num_rows, |ui, row_range| {
            display_hex_rows(ui, bytes, row_range);
        });
}

fn calculate_view_dimensions(ui: &Ui, bytes: &[u8]) -> (f32, usize) {
    let text_style = TextStyle::Monospace;
    let row_height = ui.text_style_height(&text_style);
    let num_rows = (bytes.len() + 15) / 16;
    (row_height, num_rows)
}

fn display_hex_rows(ui: &mut Ui, bytes: &[u8], row_range: std::ops::Range<usize>) {
    for row in row_range {
        let offset = row * 16;
        let chunk = &bytes[offset..std::cmp::min(offset + 16, bytes.len())];
        display_hex_row(ui, offset, chunk);
    }
}
