use eframe::egui;
use crate::localization::*;

struct App {
    localization: Localization,
}

impl App {
    fn new() -> Self {
        // Load Base Localization
        let localization: Localization = load("en.json").expect("Erorr Load");
        Self {
            localization
        }
    }

    fn name() -> &'static str {
        "Kanji Master"
    }

    fn top_bar(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                let local = &self.localization.local.top_bar;
                // Kanji Menu
                ui.menu_button(&local.kanji.title, |ui| {
                    if ui.button(&local.kanji.jlpt).clicked() {
                        println!("JLPT");
                    }

                    if ui.button(&local.kanji.kanaken).clicked() {
                        println!("Kanaken");
                    }

                    if ui.button(&local.kanji.radicals).clicked() {
                        println!("Radicals");
                    }

                    if ui.button(&local.kanji.all).clicked() {
                        println!("All");
                    }
                });

                // Tranning Menu
                ui.menu_button(&local.tranning.title, |ui| {
                    if ui.button(&local.tranning.jlpt).clicked() {
                        println!("JLPT");
                    }

                    if ui.button(&local.tranning.kanaken).clicked() {
                        println!("Kanaken");
                    }

                    if ui.button(&local.tranning.custom).clicked() {
                        println!("Custom");
                    }
                });

                // Card Menu
                ui.menu_button(&local.card.title, |ui| {
                    if ui.button(&local.card.new).clicked() {
                        println!("New");
                    }

                    if ui.button(&local.card.open).clicked() {
                        println!("Open");
                    }
                });

                // Kana Menu
                ui.menu_button(&local.kana.title, |ui| {
                    if ui.button(&local.kana.hiragana).clicked() {
                        println!("Hiragana");
                    }

                    if ui.button(&local.kana.katakana).clicked() {
                        println!("Katakana");
                    }
                });

                // Settings Menu
                ui.menu_button(&local.settings.title, |ui| {
                    if ui.button(&local.settings.general).clicked() {
                        println!("General");
                    }
                });

            });

        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        self.top_bar(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(App::name());

            if ui.button("Save").clicked() {
                let localization = Localization::default();

                save("test.json", &localization).expect("Error Save");
            }
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
        Box::new(|_| Ok(Box::new(App::new())))
    )
}