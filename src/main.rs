use log::Level;
use console_log;
use wasm_bindgen_futures;
use eframe::egui;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() -> eframe::Result<()> {
    console_log::init_with_level(Level::Debug).expect("error initializing logger");

    let runner = eframe::WebRunner::new();
    wasm_bindgen_futures::spawn_local(async move {
        runner.start(
            "the_canvas_id", // hardcode it in your index.html
            eframe::WebOptions::default(),
            Box::new(|cc| {
                // This gives us image support:
                egui_extras::install_image_loaders(&cc.egui_ctx);

                Ok(Box::<MyApp>::default())
            }),
        ).await.expect("Failed to start eframe web runner");
    });

    Ok(())
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Hello, {}!", self.name));
            ui.label(format!("You are {} years old.", self.age));
        });
    }
}