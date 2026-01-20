use crate::core::{Database, Kanji};
use crate::localization::*;
use eframe::egui;

#[derive(PartialEq)]
enum Screen {
    Home,
    Jlpt,
    Kanaken,
    Radicals,
    All,
    Kana,
}

enum KanjiList {
    All,
    Jlpt5,
    Jlpt4,
    Jlpt3,
    Jlpt2,
    Jlpt1,
    Kanaken,
    Radicals,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum JLPT {
    J5,
    J4,
    J3,
    J2,
    J1,
}

struct App {
    // Screen
    current_screen: Screen,
    // Kanji
    kanji: Vec<Kanji>,

    // Current JLPT
    current_jlpt: JLPT,

    // Search
    search: String,

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
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Set Font
        set_font(&cc.egui_ctx);
        // Load Base Localization
        let localization: Localization = load("en.json").expect("Erorr Load");
        // Load Kanji
        let kanji = Database::new("db/core.db").get_kanji().expect("Error Read");

        Self {
            current_screen: Screen::Home,
            kanji,
            current_jlpt: JLPT::J5,
            search: String::new(),
            localization,
            general_settings_window: false,
            interface_font_size: 16.0,
            kanji_font_size: 40.0,
            auto_save_progress: true,
            open_last_session_at_startup: false,
            confrim_card_delete: false,
            confrim_progress_reset: false,
        }
    }

    // Name
    fn name() -> &'static str {
        "Kanji Master"
    }

    // Top Bar
    fn top_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                let local = &self.localization.local.top_bar;
                // Kanji Menu
                ui.menu_button(&local.kanji.title, |ui| {
                    if ui.button(&local.kanji.jlpt).clicked() {
                        self.current_screen = Screen::Jlpt;
                    }

                    if ui.button(&local.kanji.kanaken).clicked() {
                        println!("Kanaken");
                    }

                    if ui.button(&local.kanji.radicals).clicked() {
                        println!("Radicals");
                    }

                    if ui.button(&local.kanji.all).clicked() {
                        self.current_screen = Screen::All;
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

    // Settings Window
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
                ui.add(egui::Slider::new(
                    &mut self.interface_font_size,
                    10.0..=28.0,
                ));

                ui.separator();

                ui.label(&local.kanji_font_size);
                ui.add(egui::Slider::new(&mut self.kanji_font_size, 10.0..=64.0));

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

    // Ui Screen All
    fn kanji_all(&mut self, ui: &mut egui::Ui) {
        self.show_kanji(ui, KanjiList::All);
    }

    // Ui Screen JLPT 5
    fn kanji_n5(&mut self, ui: &mut egui::Ui) {
        self.show_kanji(ui, KanjiList::Jlpt5);
    }

    // Ui Screen JLPT 4
    fn kanji_n4(&mut self, ui: &mut egui::Ui) {
        self.show_kanji(ui, KanjiList::Jlpt4);
    }

    // Ui Screen JLPT 3
    fn kanji_n3(&mut self, ui: &mut egui::Ui) {
        self.show_kanji(ui, KanjiList::Jlpt3);
    }

    // Ui Screen JLPT 2
    fn kanji_n2(&mut self, ui: &mut egui::Ui) {
        self.show_kanji(ui, KanjiList::Jlpt2);
    }

    // Ui Screen JLPT 1
    fn kanji_n1(&mut self, ui: &mut egui::Ui) {
        self.show_kanji(ui, KanjiList::Jlpt1);
    }

    // Show Kanji
    fn show_kanji(&mut self, ui: &mut egui::Ui, kanji_set: KanjiList) {
        // Sorted kanji list
        let filtered_kanji: Vec<&Kanji> = self.kanji
            .iter()
            .filter(|item| {
                // Filter by search (if search is not empty)
                let matches_search = self.search.is_empty() 
                    || item.kanji.contains(&self.search);

                // Filter by JLPT category
                let matches_category = match kanji_set {
                    KanjiList::All => true,
                    KanjiList::Jlpt5 => item.jlpt == "N5",
                    KanjiList::Jlpt4 => item.jlpt == "N4",
                    KanjiList::Jlpt3 => item.jlpt == "N3",
                    KanjiList::Jlpt2 => item.jlpt == "N2",
                    KanjiList::Jlpt1 => item.jlpt == "N1",
                    _ => true,
                };

                matches_search && matches_category
            })
            .collect();


        // Search Field
        ui.horizontal(|ui| {
            let desired_size = if self.current_screen == Screen::All {
                egui::vec2(ui.available_width(), 30.0)
            } else {
                egui::vec2(ui.available_width() - 150.0, 30.0)
            };

            ui.add(
                egui::TextEdit::singleline(&mut self.search)
                    .hint_text("Search Kanji...")
                    .min_size(desired_size)
            );

            if self.current_screen == Screen::Jlpt {
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", self.current_jlpt))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.current_jlpt, JLPT::J5, "N5");
                        ui.selectable_value(&mut self.current_jlpt, JLPT::J4, "N4");
                        ui.selectable_value(&mut self.current_jlpt, JLPT::J3, "N3");
                        ui.selectable_value(&mut self.current_jlpt, JLPT::J2, "N2");
                        ui.selectable_value(&mut self.current_jlpt, JLPT::J1, "N1");
                    });
            }
        });

        ui.separator();

        let columns = 4;
        let spacing = 10.0;
        let card_height = 150.0;
        let row_height = card_height + spacing;

        let total_rows = (filtered_kanji.len() + columns - 1) / columns;

        // Scroll Area
        egui::ScrollArea::vertical().show_rows(ui, row_height, total_rows, |ui, row_range| {
            let available_width = ui.available_width();
            let card_width = (available_width - (spacing * (columns as f32 - 1.0))) / columns as f32;

            // Rows
            for row_index in row_range {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = spacing;

                    let start_index = row_index * columns;
                    let end_index = (start_index + columns).min(self.kanji.len());

                    // Cards
                    for i in start_index..end_index {
                        if let Some(item) = filtered_kanji.get(i) {
                            egui::Frame::new()
                                .fill(ui.visuals().faint_bg_color)
                                .stroke(ui.visuals().window_stroke)
                                .corner_radius(4.0)
                                .show(ui, |ui| {
                                    ui.set_width(card_width);
                                    ui.set_height(card_height);

                                    ui.vertical_centered(|ui| {
                                        let center_offset = (card_height / 2.0) - (self.kanji_font_size / 2.0) - 5.0;
                                        if center_offset > 0.0 {
                                            ui.add_space(center_offset);
                                        }

                                        ui.label(
                                            egui::RichText::new(&item.kanji)
                                                .size(self.kanji_font_size)
                                                .strong()
                                        );
                                        
                                    });
                                });
                        }
                    }
                });
                ui.add_space(spacing);
            }
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.top_bar(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.current_screen == Screen::Home {
                ui.heading(App::name());

                if ui.button("Save").clicked() {
                    let localization = Localization::default();

                    save("test.json", &localization).expect("Error Save");
                }
            }

            match self.current_screen {
                Screen::All => self.kanji_all(ui),
                Screen::Jlpt => {
                    match self.current_jlpt {
                        JLPT::J5 => self.kanji_n5(ui),
                        JLPT::J4 => self.kanji_n4(ui),
                        JLPT::J3 => self.kanji_n3(ui),
                        JLPT::J2 => self.kanji_n2(ui),
                        JLPT::J1 => self.kanji_n1(ui),
                    }
                }
                _ => {}
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
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

// Set Font
fn set_font(ctx: &egui::Context) {
    static mut LOADED: bool = false;
    if !unsafe { LOADED } {
        let font_data = include_bytes!("../font/NotoSansJP-VariableFont_wght.ttf");
        let font = egui::FontData::from_static(font_data).tweak(egui::FontTweak {
            scale: 1.0,
            y_offset_factor: 0.0,
            y_offset: 0.0,
            ..Default::default()
        });

        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert("noto_cjk".to_owned(), font.into());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_insert_with(Vec::new)
            .push("noto_cjk".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_insert_with(Vec::new)
            .push("noto_cjk".to_owned());

        ctx.set_fonts(fonts);

        unsafe {
            LOADED = true;
        }
    }
}
