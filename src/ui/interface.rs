use crate::back::core::{Database, Kanji};
use crate::back::localization::*;
use crate::ui::animator::KanjiAnimator;
use eframe::egui;
use std::path::Path;
use crate::ui::settings::Settings;
use crate::back::config::{Config, save_config};
use crate::back::romaji_kana::to_kana;
use crate::back::translation::*;


// Screens
#[derive(PartialEq, Clone)]
enum Screen {
    Home,
    Kanji(Kanji),
    Jlpt,
    All,
    Kana,
    RomajiToKana,
    TranslateKana,
}

// Hiragana and Katakana Struct
struct KanaItem {
    kana: &'static str,
    romaji: &'static str,
}

// Kanji List
enum KanjiList {
    All,
    Jlpt5,
    Jlpt4,
    Jlpt3,
    Jlpt2,
    Jlpt1,
}

// JLPT Levels
#[derive(Debug, PartialEq, Clone, Copy)]
enum JLPT {
    N5,
    N4,
    N3,
    N2,
    N1,
}

struct App {
    // Screen
    current_screen: Screen,

    // Kanji
    kanji: Vec<Kanji>,

    // Paths to Files
    paths: Paths,

    // Current JLPT
    current_jlpt: JLPT,

    // Search
    search: String,

    // Settings
    settings: Settings,

    // Kanji Animator
    animator: KanjiAnimator,

    // Localization
    localization: Localization,

    // Settings Window
    settings_window: bool,

    // Is Katakana
    is_katakana: bool,

    // Romaji Input
    romaji_input: String,

    // Translation State
    translate_state: TranslateState,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        // Set Font
        set_font(&cc.egui_ctx);
        // Load Base Localization
        let localization: Localization = load(&config.path_to_localization).expect("Erorr Load");
        // Load Kanji
        let kanji = Database::new(&config.path_to_db_core).get_kanji().expect("Error Read");

        let paths = Paths {
            path_to_db_core: config.path_to_db_core.clone(),
            path_to_localization: config.path_to_localization.clone(),
            path_to_kanji_localization: config.path_to_kanji_localization.clone(),
        };

        let message_data = MessageTranslationData {
            file_load_success: localization.local.top_bar.tools.translate_kanji_locale.file_load_success.clone(),
            file_not_found_or_invalid: localization.local.top_bar.tools.translate_kanji_locale.file_not_found_or_invalid.clone(),
            saved_at_id: localization.local.top_bar.tools.translate_kanji_locale.saved_at_id.clone(),
            error_searialize_json: localization.local.top_bar.tools.translate_kanji_locale.error_searialize_json.clone(),
            error_create_file: localization.local.top_bar.tools.translate_kanji_locale.error_create_file.clone(),
        };

        let mut translate_state = TranslateState::new(&config.path_to_kanji_localization, message_data);
        
        let loaded_successfully = translate_state.load(&config.path_to_kanji_localization);

        if let Some(first_kanji) = kanji.get(translate_state.current_index) {
            translate_state.sync_translation_buffers(&first_kanji);
        }

        // If the load was successful, update the current index
        if loaded_successfully {
            if let Some(pos) = kanji.iter().position(|k| k.id == translate_state.data.last_id) {
                translate_state.current_index = pos;
            }
            if let Some(first_kanji) = kanji.get(translate_state.current_index) {
                translate_state.sync_translation_buffers(&first_kanji);
            }
        }

        // If loading failed, reset the state
        let settings_window = !loaded_successfully;

        Self {
            current_screen: Screen::Home,
            kanji,
            paths,
            current_jlpt: JLPT::N5,
            search: String::new(),
            settings: Settings::new(localization.clone(), config),
            animator: KanjiAnimator::new(),
            localization,
            settings_window,
            is_katakana: false,
            romaji_input: String::new(),
            translate_state,
        }
    }

    // Name
    fn name() -> &'static str {
        "Kanji Master"
    }

    // Auxiliary function for reloading localization UI
    fn reload_interface_localization(&mut self) {
        if let Ok(new_loc) = load::<Localization>(&self.paths.path_to_localization) {
            self.localization = new_loc.clone();
            self.settings.update_localization(new_loc);
        }
    }

    // Auxiliary function for restarting when changing settings
    fn reload_translations(&mut self) {
        // Load translations from the new path
        if self.translate_state.load(&self.paths.path_to_kanji_localization) {
            // If successful, update the position
            if let Some(pos) = self.kanji.iter().position(|k| k.id == self.translate_state.data.last_id) {
                self.translate_state.current_index = pos;
            }
            // Sync UI buffers
            if let Some(k) = self.kanji.get(self.translate_state.current_index) {
                self.translate_state.sync_translation_buffers(k);
            }
        }
    }

    // Save Config
    fn save(&mut self) {
        let current_config = Config {
            interface_font_size: self.settings.interface_font_size,
            kanji_font_size: self.settings.kanji_font_size,
            auto_save_progress: self.settings.auto_save_progress,
            open_last_session_at_startup: self.settings.open_last_session_at_startup,
            confrim_card_delete: self.settings.confrim_card_delete,
            confrim_progress_reset: self.settings.confrim_progress_reset,
            animation_speed: self.settings.animation_speed,
            path_to_db_core: self.paths.path_to_db_core.clone(),
            path_to_localization: self.paths.path_to_localization.clone(),
            path_to_kanji_localization: self.paths.path_to_kanji_localization.clone(),
        };

        if let Err(e) = save_config("config.toml", &current_config) {
            eprintln!("Error saving config: {}", e.to_string());
        }
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

                    if ui.button(&local.kanji.all).clicked() {
                        self.current_screen = Screen::All;
                    }
                });

                // Tranning Menu
                ui.menu_button(&local.tranning.title, |ui| {
                    if ui.button(&local.tranning.jlpt).clicked() {
                        println!("JLPT");
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

                // Kana Button
                if ui.button(&local.kana.title).clicked() {
                    self.current_screen = Screen::Kana;
                }

                // Settings Button
                if ui.button(&self.localization.local.settings.title).clicked() {
                    self.settings_window = true;
                }

                // Tools Menu
                ui.menu_button(&local.tools.title, |ui| {
                    if ui.button(&local.tools.romaji_to_kana).clicked() {
                        self.current_screen = Screen::RomajiToKana;
                    }

                    if ui.button(&local.tools.translate_kanji).clicked() {
                        self.current_screen = Screen::TranslateKana;
                    }
                });
            });
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
                    || item.kanji.contains(&self.search)
                    || item.onyomi.contains(&self.search)
                    || item.kunyomi.contains(&self.search)
                    || item.onyomi_romaji.contains(&self.search)
                    || item.kunyomi_romaji.contains(&self.search);

                // Filter by JLPT category
                let matches_category = match kanji_set {
                    KanjiList::All => true,
                    KanjiList::Jlpt5 => item.jlpt == "N5",
                    KanjiList::Jlpt4 => item.jlpt == "N4",
                    KanjiList::Jlpt3 => item.jlpt == "N3",
                    KanjiList::Jlpt2 => item.jlpt == "N2",
                    KanjiList::Jlpt1 => item.jlpt == "N1",
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
                    .hint_text(&self.localization.local.screens.search)
                    .min_size(desired_size)
            );

            if self.current_screen == Screen::Jlpt {
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", self.current_jlpt))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.current_jlpt, JLPT::N5, "N5");
                        ui.selectable_value(&mut self.current_jlpt, JLPT::N4, "N4");
                        ui.selectable_value(&mut self.current_jlpt, JLPT::N3, "N3");
                        ui.selectable_value(&mut self.current_jlpt, JLPT::N2, "N2");
                        ui.selectable_value(&mut self.current_jlpt, JLPT::N1, "N1");
                    });
            }
        });

        ui.separator();

        let columns = 4;
        let spacing = 10.0;

        let available_width = ui.available_width();
        let card_width = (available_width - (spacing * (columns as f32 - 1.0))) / columns as f32;

        let card_height = (card_width / 3.0) * 4.0;
        let row_height = card_height + spacing;

        let total_rows = (filtered_kanji.len() + columns - 1) / columns;

        ui.separator();

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
                            let (rect, response) = ui.allocate_at_least(
                                egui::vec2(card_width, card_height),
                                egui::Sense::click(),
                            );

                            ui.painter().rect(
                                rect,
                                4.0,
                                ui.visuals().faint_bg_color,
                                ui.visuals().window_stroke,
                                egui::StrokeKind::Middle,
                            );

                            ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                                ui.vertical_centered(|ui| {
                                    let text_height = self.settings.kanji_font_size;
                                    ui.add_space((card_height - text_height) / 2.0);

                                    ui.label(
                                        egui::RichText::new(&item.kanji)
                                            .size(text_height)
                                            .strong()
                                    );
                                });
                            });

                            if response.clicked() {
                                let selected = (*item).clone();
                                let svg_path = format!("kanji-svg/0{}.svg", item.unicode.to_lowercase());
                                
                                if Path::new(&svg_path).exists() {
                                    if let Err(_) = self.animator.load_svg(&svg_path) {
                                        self.animator.clear();
                                    }
                                } else {
                                    self.animator.clear();
                                }
                        
                                self.current_screen = Screen::Kanji(selected);
                            }

                            if response.hovered() {
                                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                            }

                        }
                    }
                });
                ui.add_space(spacing);
            }
        });
    }

    // Show Current Kanji
    fn kanji_screen(&mut self, ui: &mut egui::Ui, kanji: &Kanji) {
        let local = &self.localization.local.screens.current_kanji;
        if ui.button(&local.back_button).clicked() {
            self.current_screen = Screen::All;
        }

        ui.add_space(20.0);

        ui.columns(2, |columns| {
            let available_w = columns[0].available_width().min(340.0);
            let card_w = available_w * 0.94;
            let card_h = card_w * (4.0 / 3.0);

            columns[0].vertical_centered(|ui| {
                ui.add_space(12.0);

                egui::Frame::canvas(ui.style())
                    .fill(ui.visuals().window_fill)
                    .stroke(ui.visuals().window_stroke)
                    .corner_radius(16.0)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(card_w, card_h),
                            egui::Sense::click()
                        );

                        if response.clicked() {
                            self.animator.replay();
                        }

                        if response.hovered() {
                            ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                        }

                        // ANIMATION
                        let anim_side = rect.width().min(rect.height()) * 0.88;
                        let center = rect.center();
                        let draw_rect = egui::Rect::from_center_size(
                            center,
                            egui::vec2(anim_side, anim_side),
                        );

                        self.animator.ui(ui, draw_rect, &kanji.kanji, self.settings.animation_speed);

                    });

                    ui.add_space(16.0);
            });

            columns[1].vertical_centered(|ui| {
                ui.add_space(8.0);
                ui.heading(&local.information);
                ui.add_space(12.0);
                ui.separator();

                egui::Grid::new("info_grid")
                    .num_columns(2)
                    .spacing([24.0, 12.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new(&format!("{}:", &local.meaning)).strong());
                        ui.end_row();

                        // Onyomi
                        ui.label(egui::RichText::new(format!("{}: {}", &local.onyomi, kanji.onyomi)).strong());
                        ui.end_row();
                        ui.label(egui::RichText::new(format!("{}: {}", &local.onyomi_romaji, kanji.onyomi_romaji)).strong());
                        ui.end_row();

                        // Kunyomi
                        ui.label(egui::RichText::new(format!("{}: {}", &local.kunyomi, kanji.kunyomi)).strong());
                        ui.end_row();
                        ui.label(egui::RichText::new(format!("{}: {}", &local.kunyomi_romaji, kanji.kunyomi_romaji)).strong());
                        ui.end_row();

                        // Kanji Info
                        ui.label(egui::RichText::new(format!("{}: {}", &local.strokes, kanji.strokes)).strong());
                        ui.end_row();
                        ui.label(egui::RichText::new(format!("{}: {}", &local.jlpt, kanji.jlpt)).strong());
                        ui.end_row();
                        ui.label(egui::RichText::new(format!("{}: {}", &local.grade, kanji.grade)).strong());
                        ui.end_row();
                        ui.label(egui::RichText::new(format!("{}: {}", &local.frequency, kanji.frequency)).strong());
                        ui.end_row();

                    });

                ui.add_space(28.0);

                // Examples
                ui.heading("Examples");
                ui.add_space(12.0);
                ui.separator();

                ui.vertical(|ui| {
                    for example in &kanji.example {
                        ui.horizontal_top(|ui| {
                            ui.add(egui::Label::new(egui::RichText::new(example)).wrap());
                        });

                        ui.add_space(10.0);
                    }
                });
            });
        });
    }

    // Show Kana Screen
    fn kana_screen(&mut self, ui: &mut egui::Ui) {
        let hiragana_list: Vec<KanaItem> = vec![
            KanaItem { kana: "あ", romaji: "a" }, KanaItem { kana: "い", romaji: "i" }, KanaItem { kana: "う", romaji: "u" }, KanaItem { kana: "え", romaji: "e" }, KanaItem { kana: "お", romaji: "o" },
            KanaItem { kana: "か", romaji: "ka" }, KanaItem { kana: "き", romaji: "ki" }, KanaItem { kana: "く", romaji: "ku" }, KanaItem { kana: "け", romaji: "ke" }, KanaItem { kana: "こ", romaji: "ko" },
            KanaItem { kana: "さ", romaji: "sa" }, KanaItem { kana: "し", romaji: "shi" }, KanaItem { kana: "す", romaji: "su" }, KanaItem { kana: "せ", romaji: "se" }, KanaItem { kana: "そ", romaji: "so" },
            KanaItem { kana: "た", romaji: "ta" }, KanaItem { kana: "ち", romaji: "chi" }, KanaItem { kana: "つ", romaji: "tsu" }, KanaItem { kana: "て", romaji: "te" }, KanaItem { kana: "と", romaji: "to" },
            KanaItem { kana: "な", romaji: "na" }, KanaItem { kana: "に", romaji: "ni" }, KanaItem { kana: "ぬ", romaji: "nu" }, KanaItem { kana: "ね", romaji: "ne" }, KanaItem { kana: "の", romaji: "no" },
            KanaItem { kana: "は", romaji: "ha" }, KanaItem { kana: "ひ", romaji: "hi" }, KanaItem { kana: "ふ", romaji: "fu" }, KanaItem { kana: "へ", romaji: "he" }, KanaItem { kana: "ほ", romaji: "ho" },
            KanaItem { kana: "ま", romaji: "ma" }, KanaItem { kana: "み", romaji: "mi" }, KanaItem { kana: "む", romaji: "mu" }, KanaItem { kana: "め", romaji: "me" }, KanaItem { kana: "も", romaji: "mo" },
            KanaItem { kana: "や", romaji: "ya" }, KanaItem { kana: "ゆ", romaji: "yu" }, KanaItem { kana: "よ", romaji: "yo" },
            KanaItem { kana: "ら", romaji: "ra" }, KanaItem { kana: "り", romaji: "ri" }, KanaItem { kana: "る", romaji: "ru" }, KanaItem { kana: "れ", romaji: "re" }, KanaItem { kana: "ろ", romaji: "ro" },
            KanaItem { kana: "わ", romaji: "wa" }, KanaItem { kana: "を", romaji: "wo" }, KanaItem { kana: "ん", romaji: "n" },
        ];

        let katakana_list: Vec<KanaItem> = vec![
            KanaItem { kana: "ア", romaji: "a" }, KanaItem { kana: "イ", romaji: "i" }, KanaItem { kana: "ウ", romaji: "u" }, KanaItem { kana: "エ", romaji: "e" }, KanaItem { kana: "オ", romaji: "o" },
            KanaItem { kana: "カ", romaji: "ka" }, KanaItem { kana: "キ", romaji: "ki" }, KanaItem { kana: "ク", romaji: "ku" }, KanaItem { kana: "ケ", romaji: "ke" }, KanaItem { kana: "コ", romaji: "ko" },
            KanaItem { kana: "サ", romaji: "sa" }, KanaItem { kana: "シ", romaji: "shi" }, KanaItem { kana: "ス", romaji: "su" }, KanaItem { kana: "セ", romaji: "se" }, KanaItem { kana: "ソ", romaji: "so" },
            KanaItem { kana: "タ", romaji: "ta" }, KanaItem { kana: "チ", romaji: "chi" }, KanaItem { kana: "ツ", romaji: "tsu" }, KanaItem { kana: "テ", romaji: "te" }, KanaItem { kana: "ト", romaji: "to" },
            KanaItem { kana: "ナ", romaji: "na" }, KanaItem { kana: "ニ", romaji: "ni" }, KanaItem { kana: "ヌ", romaji: "nu" }, KanaItem { kana: "ネ", romaji: "ne" }, KanaItem { kana: "ノ", romaji: "no" },
            KanaItem { kana: "ハ", romaji: "ha" }, KanaItem { kana: "ヒ", romaji: "hi" }, KanaItem { kana: "フ", romaji: "fu" }, KanaItem { kana: "ヘ", romaji: "he" }, KanaItem { kana: "ホ", romaji: "ho" },
            KanaItem { kana: "マ", romaji: "ma" }, KanaItem { kana: "ミ", romaji: "mi" }, KanaItem { kana: "ム", romaji: "mu" }, KanaItem { kana: "メ", romaji: "me" }, KanaItem { kana: "モ", romaji: "mo" },
            KanaItem { kana: "ヤ", romaji: "ya" }, KanaItem { kana: "ユ", romaji: "yu" }, KanaItem { kana: "ヨ", romaji: "yo" },
            KanaItem { kana: "ラ", romaji: "ra" }, KanaItem { kana: "リ", romaji: "ri" }, KanaItem { kana: "ル", romaji: "ru" }, KanaItem { kana: "レ", romaji: "re" }, KanaItem { kana: "ロ", romaji: "ro" },
            KanaItem { kana: "ワ", romaji: "wa" }, KanaItem { kana: "ヲ", romaji: "wo" }, KanaItem { kana: "ン", romaji: "n" },
        ];

        let current_list = if self.is_katakana { &katakana_list } else { &hiragana_list };

        ui.horizontal(|ui| {
            let local = &self.localization.local.kana;
            ui.heading(&local.title);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Buttons for switching
                if ui.selectable_label(self.is_katakana, &local.katakana).clicked() {
                    self.is_katakana = true;
                }
                if ui.selectable_label(!self.is_katakana, &local.hiragana).clicked() {
                    self.is_katakana = false;
                }
            });
        });

        ui.separator();

        let columns = 5;
        let spacing = 10.0;
        let available_width = ui.available_width();

        let card_width = (available_width - (spacing * (columns as f32 - 1.0))) / columns as f32;
        let card_height = card_width * 1.2;

        let total_rows = (current_list.len() + columns - 1) / columns;
        let row_height = card_height + spacing;

        egui::ScrollArea::vertical().show_rows(ui, row_height, total_rows, |ui, row_range| {
            for row_index in row_range {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = spacing;

                    let start_index = row_index * columns;
                    let end_index = (start_index + columns).min(current_list.len());

                    for i in start_index..end_index {
                        if let Some(item) = current_list.get(i) {
                            let (rect, response) = ui.allocate_at_least(
                                egui::vec2(card_width, card_height),
                                egui::Sense::hover(),
                            );

                            // Card background
                            ui.painter().rect(
                                rect,
                                8.0,
                                ui.visuals().faint_bg_color,
                                ui.visuals().window_stroke,
                                egui::StrokeKind::Middle,
                            );

                            // Card content
                            ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                                ui.vertical_centered(|ui| {
                                    // Calculate font sizes
                                    let kana_size = self.settings.kanji_font_size * 0.8;
                                    let romaji_size = self.settings.interface_font_size;
                                    
                                    // Simple calculation for vertical centering
                                    let total_text_h = kana_size + romaji_size + 4.0;
                                    ui.add_space((card_height - total_text_h) / 2.0);

                                    // Kana
                                    ui.label(
                                        egui::RichText::new(item.kana)
                                            .size(kana_size)
                                            .strong()
                                    );
                                    
                                    // Romaji
                                    ui.label(
                                        egui::RichText::new(item.romaji)
                                            .size(romaji_size)
                                            .color(ui.visuals().text_color().gamma_multiply(0.7))
                                    );
                                });
                            });

                            if response.hovered() {
                                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                            }
                        }
                    }
                });
                ui.add_space(spacing);
            }
        });
        
    }

    // Romaji to Kana Screen
    fn romaji_to_kana_screen(&mut self, ui: &mut egui::Ui) {
        let local = &self.localization.local.top_bar.tools.romaji_to_kana_locale;

        ui.heading(&local.title);
        ui.add_space(10.0);

        // Hiragana/Katakana toggle
        ui.horizontal(|ui| {
            ui.label(&format!("{}:", &local.output_mode));
            ui.radio_value(&mut self.is_katakana, false, &local.output_hiragana);
            ui.radio_value(&mut self.is_katakana, true, &local.output_katakana);
        });

        ui.add_space(15.0);

        ui.columns(2, |columns| {
            // Left column: Input (Romaji)
            columns[0].vertical(|ui| {
                ui.label(egui::RichText::new(&local.input).strong());
                ui.add(
                    egui::TextEdit::multiline(&mut self.romaji_input)
                        .hint_text(&local.hint_input)
                        .desired_width(f32::INFINITY)
                        .min_size(egui::vec2(0.0, 200.0)),
                );
            });

            // Right column: Output (Kana)
            columns[1].vertical(|ui| {
                ui.label(egui::RichText::new(&local.output).strong());
                
                // Convert in real time
                let output_text = to_kana(&self.romaji_input, self.is_katakana);

                // Display result (read-only)
                egui::ScrollArea::vertical().id_salt("output_scroll").show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut output_text.clone())
                            .desired_width(f32::INFINITY)
                            .min_size(egui::vec2(0.0, 200.0))
                            .font(egui::FontId::proportional(24.0)),
                    );
                });

                ui.add_space(5.0);
                
                // Copy button
                if ui.button(&local.copy_button).clicked() {
                    ui.ctx().copy_text(output_text);
                }
            });
        });
    }

    // Translate Tool Screen
    fn translate_kanji_screen(&mut self, ui: &mut egui::Ui) {
        let local = &self.localization.local.top_bar.tools.translate_kanji_locale;
        ui.heading(&local.title);

        ui.horizontal(|ui| {
            ui.label(&local.jump_to);
            let response = ui.add(
                egui::TextEdit::singleline(&mut self.translate_state.jump_search_buffer)
                    .desired_width(50.0).hint_text(&local.hint_kanji_input)
            );

            if ui.button(&local.go_button).clicked() || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                let search = self.translate_state.jump_search_buffer.trim();
                
                if let Some(pos) = self.kanji.iter().position(|k| k.kanji == search) {
                    self.translate_state.current_index = pos;

                    if let Some(k) = self.kanji.get(pos) {
                        self.translate_state.sync_translation_buffers(k);
                    }

                    self.translate_state.status_message = local.found.clone();
                } else {
                    self.translate_state.status_message = local.not_found.clone();
                }
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(&local.save_progress_button).clicked() {
                    self.translate_state.save_translations(&self.kanji);
                }
                ui.label(&self.translate_state.status_message);
            });
        });

        ui.separator();

        // Check if we've reached the end of the list
        if self.translate_state.current_index >= self.kanji.len() {
            ui.centered_and_justified(|ui| {
                ui.heading(&local.all_kanji_processed);
            });
            return;
        }

        let current_kanji = self.kanji[self.translate_state.current_index].clone();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_space(10.0);
            
            // Title: Kanji and ID
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new(&current_kanji.kanji).size(40.0).strong());
                ui.label(format!("(ID: {}, Index: {}/{})", current_kanji.id, self.translate_state.current_index + 1, self.kanji.len()));
            });

            ui.add_space(10.0);

            // Field: Meaning
            ui.label(egui::RichText::new(&local.meaning).strong());
            ui.add(
                egui::TextEdit::singleline(&mut self.translate_state.meaning_buffer)
                    .hint_text(&local.hint_meaning_input)
                    .desired_width(f32::INFINITY)
            );

            ui.add_space(20.0);
            ui.separator();
            ui.heading(&local.examples);
            ui.add_space(10.0);
            
            // Grid for alignment: Original | Translation
            egui::Grid::new("examples_grid")
                .num_columns(2)
                .spacing([20.0, 10.0])
                .striped(true)
                .show(ui, |ui| {
                    for (idx, original_ex) in current_kanji.example.iter().enumerate() {
                        // Колонка 1: Оригинал
                        ui.label(original_ex);

                        // Field: Input Translation
                        if idx < self.translate_state.examples_buffer.len() {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.translate_state.examples_buffer[idx])
                                    .hint_text(&local.hint_examples_input)
                                    .desired_width(300.0)
                            );
                        } else {
                            ui.label(&local.error_buffer_mismatch);
                        }
                        ui.end_row();
                    }
                });
            
            ui.add_space(20.0);
            
            // Buttons: Previous, Next
            ui.horizontal(|ui| {
                if ui.button(format!("⬅ {}", &local.previous_button)).clicked() {
                    if self.translate_state.current_index > 0 {
                        // Save current to memory (but not to disk to be faster)
                        let entry = KanjiTranslation {
                            meaning: self.translate_state.meaning_buffer.clone(),
                            translate_examples: self.translate_state.examples_buffer.clone(),
                        };
                        self.translate_state.data.entries.insert(current_kanji.kanji.clone(), entry);

                        // Backward switch
                        self.translate_state.current_index -= 1;
                        if let Some(k) = self.kanji.get(self.translate_state.current_index) {
                            self.translate_state.sync_translation_buffers(k);
                        }
                        self.translate_state.status_message.clear();
                    }
                }

                // Button NEXT (Saves to memory and switches)
                if ui.button(format!("{} ➡", &local.next_button)).clicked() {
                    // Save current data to memory
                    let entry = KanjiTranslation {
                        meaning: self.translate_state.meaning_buffer.clone(),
                        translate_examples: self.translate_state.examples_buffer.clone(),
                    };
                    self.translate_state.data.entries.insert(current_kanji.kanji.clone(), entry);
                    
                    // Update last_id
                    self.translate_state.data.last_id = current_kanji.id;

                    // (Optional) Auto-save to disk on each Next click
                    // self.save_translations(); 

                    // Next kanji
                    self.translate_state.current_index += 1;
                    
                    // Sync buffers for new kanji
                    if let Some(next_k) = self.kanji.get(self.translate_state.current_index) {
                        self.translate_state.sync_translation_buffers(next_k);
                    }
                    self.translate_state.status_message.clear();
                }
            });
        });
    }
}

impl eframe::App for App {
    // Update App
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        let font_size = self.settings.interface_font_size;

        style.text_styles = [
            (
                egui::TextStyle::Small,
                egui::FontId::new(font_size * 0.75, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Body,
                egui::FontId::new(font_size, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Button,
                egui::FontId::new(font_size, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Heading,
                egui::FontId::new(font_size * 1.5, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Monospace,
                egui::FontId::new(font_size, egui::FontFamily::Monospace),
            ),
        ]
        .into();

        ctx.set_style(style);

        self.top_bar(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.current_screen == Screen::Home {
                ui.heading(App::name());

                if ui.button("Save").clicked() {
                    let localization = Localization::default();

                    save("test.json", &localization).expect("Error Save");
                }
            }

            match &self.current_screen {
                Screen::All => self.kanji_all(ui),
                Screen::Jlpt => {
                    match self.current_jlpt {
                        JLPT::N5 => self.kanji_n5(ui),
                        JLPT::N4 => self.kanji_n4(ui),
                        JLPT::N3 => self.kanji_n3(ui),
                        JLPT::N2 => self.kanji_n2(ui),
                        JLPT::N1 => self.kanji_n1(ui),
                    }
                }

                Screen::Kanji(selected_kanji) => {
                    let kanji_to_show = selected_kanji.clone();
                    self.kanji_screen(ui, &kanji_to_show);
                }
                Screen::Kana => {
                    self.kana_screen(ui);
                }

                Screen::RomajiToKana => {
                    self.romaji_to_kana_screen(ui);
                }

                Screen::TranslateKana => {
                    self.translate_kanji_screen(ui);
                }

                _ => {}
            }

        });

        if self.settings.setting(&mut self.settings_window, &mut self.paths, ctx) {
            self.reload_translations();
            self.reload_interface_localization();
            self.save();
        }
    }

    // On Exit and Save Config
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save();
    }
}

// Run App
pub fn run(config: Config) -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((800.0, 600.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        App::name(),
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc, config)))),
    )
}

// Set Font
fn set_font(ctx: &egui::Context) {
    static mut LOADED: bool = false;
    if !unsafe { LOADED } {
        let font_data = include_bytes!("../../font/NotoSansJP-VariableFont_wght.ttf");
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
