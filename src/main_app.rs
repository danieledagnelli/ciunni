fn get_grid_size() -> (usize, usize) {
    (usize::MAX, usize::MAX) // Maximum possible rows, 10 columns
}


struct GridApp {
    grid_size: (usize, usize),
}

impl GridApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            grid_size: get_grid_size(),
        }
    }
}

impl eframe::App for GridApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Grid Application");
            
            let (rows, cols) = self.grid_size;
            let available_size = ui.available_size();
            let grid_size = 1000.0_f32.min(available_size.x.min(available_size.y));
            let cell_size = grid_size / (rows.max(cols) as f32);
            
            let (response, painter) = ui.allocate_painter(
                egui::vec2(grid_size, grid_size),
                egui::Sense::hover(),
            );
            
            let rect = response.rect;
            
            for row in 0..rows {
                for col in 0..cols {
                    let cell_rect = egui::Rect::from_min_size(
                        rect.min + egui::vec2(col as f32 * cell_size, row as f32 * cell_size),
                        egui::vec2(cell_size, cell_size),
                    );
                    painter.rect_stroke(cell_rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
                }
            }
        });
    }
}