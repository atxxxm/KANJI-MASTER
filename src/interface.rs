use eframe::egui;
use crate::localization::*;

struct App {
    // Localization
    localization: Localization,
    // Settings Window
    general_settings_window: bool,
    // Interface Font Size Value
    interface_font_size: f32,
    // Kanji Font Size Value
    kanji_font_size: f32,
    // Auto Save Progress
    auto_save_progress: bool,

    // Open Last Session At Startup
    open_last_session_at_startup: bool,
    // Confrim Delete Card
    confrim_card_delete: bool,
    // Confrim Reset Progress
    confrim_progress_reset: bool,
}

impl App {
    fn new() -> Self {
        // Load Base Localization
        let localization: Localization = load("en.json").expect("Erorr Load");
        Self {
            localization,
            general_settings_window: false,
            interface_font_size: 16.0,
            kanji_font_size: 16.0,
            auto_save_progress: true,
            open_last_session_at_startup: false,
            confrim_card_delete: false,
            confrim_progress_reset: false,
        }
    }

    fn name() -> &'static str {
        "Kanji Master"
    }

    fn top_bar(&mut self, ctx: &egui::Context) {
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
                if ui.button(&self.localization.local.settings.title).clicked() {
                    self.general_settings_window = true;
                }

            });

        });
    }

    fn setting(&mut self, ctx: &egui::Context) {
        if !self.general_settings_window {
            return;
        }

        let local = &self.localization.local.settings;

        egui::Window::new(&local.title)
            .open(&mut self.general_settings_window)
            .resizable(true)
            .show(ctx, |ui| {
                ui.label(&local.lang);
                if ui.button(&local.lang_button).clicked() {
                    println!("Select");
                }

                ui.separator();

                ui.label(&local.interface_font_size);
                ui.add(egui::Slider::new(&mut self.interface_font_size, 10.0..=28.0));

                ui.separator();

                ui.label(&local.kanji_font_size);
                ui.add(egui::Slider::new(&mut self.kanji_font_size, 10.0..=32.0));

                ui.separator();

                ui.label(&local.auto_save_progress);
                ui.checkbox(&mut self.auto_save_progress, "");

                ui.separator();

                ui.label(&local.auto_save_frequency);
                
                //egui::ComboBox::from_label("")
                //    .selected_text("10 minutes")
                //    .show_ui(ui, |ui| {
                //        ui.selectable_value(
                //            &mut self.auto_save_frequency,
                //            "10 minutes".to_string(),
                //            "10 minutes",
                //        );
                //    });

                ui.separator();

                ui.label(&local.open_last_session_at_startup);
                ui.checkbox(&mut self.open_last_session_at_startup, "");

                ui.separator();

                ui.label(&local.startup_screen);
                //egui::ComboBox::from_label("")
                //    .selected_text("10 minutes")
                //    .show_ui(ui, |ui| {
                //        ui.selectable_value(
                //            &mut self.auto_save_frequency,
                //            "10 minutes".to_string(),
                //            "10 minutes",
                //        );
                //    });

                ui.separator();

                ui.label(&local.confrim_card_delete);
                ui.checkbox(&mut self.confrim_card_delete, "");

                ui.separator();

                ui.label(&local.confrim_progress_reset);
                ui.checkbox(&mut self.confrim_progress_reset, "");
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

            // Settings
            self.setting(ctx);
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