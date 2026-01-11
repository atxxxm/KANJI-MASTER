use eframe::egui;


struct App;

impl App {
    fn name() -> &'static str {
        "Kanji Master"
    }
}

impl Default for App {
    fn default() -> Self {
        Self 
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(App::name());
        });
    }
}

pub fn run() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((800.0, 600.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        App::name(),
        native_options,
        Box::new(|_| Ok(Box::new(App::default())))
    )
}