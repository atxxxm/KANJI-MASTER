use crate::back::core::{Database, Kanji};
use crate::back::localization::*;
use crate::ui::animator::KanjiAnimator;
use eframe::egui;
use std::path::Path;
use crate::ui::settings::Settings;
use crate::back::config::{Config, save_config};
use crate::back::romaji_kana::to_kana;
use crate::back::translation::*;
use crate::back::cards::*;


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
    CardsSetup,
    CardsActive,
    DeckManager,
}

// Hiragana and Katakana Struct
struct KanaItem {
    kana: &'static str,
    romaji: &'static str,
}

// Kanji List
#[derive(Debug, PartialEq, Clone, Copy)]
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

    // Config
    config: Config,

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

    // Cards
    cards_session: Option<CardsSession>,

    // Deck Builder
    deck_builder: DeckBuilderState,

    // Card limit
    card_limit: usize,

    // Selected Deck Name
    selected_deck_name: String,

    // Selected Kanji List
    selected_kanji_list: KanjiList, 
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
            settings: Settings::new(localization.clone(), &config),
            config,
            paths,
            current_jlpt: JLPT::N5,
            search: String::new(),
            animator: KanjiAnimator::new(),
            localization,
            settings_window,
            is_katakana: false,
            romaji_input: String::new(),
            translate_state,
            cards_session: None,
            deck_builder: DeckBuilderState::default(),
            card_limit: 10,
            selected_deck_name: String::new(),
            selected_kanji_list: KanjiList::All,
        }
    }

    // Name
    fn name() -> &'static str {
        "Kanji Master"
    }

    // Home Screen
    fn home_screen(&mut self, ui: &mut egui::Ui) {
        let translations = &self.translate_state.data.entries;
        let search_query = self.search.trim().to_lowercase();
        let is_search_empty = self.search.trim().is_empty();

        let filtered_kanji: Vec<&Kanji> = self.kanji
            .iter()
            .filter(|item| {
                if is_search_empty { return false; }

                let matches_basic = item.kanji.contains(&self.search)
                    || item.onyomi.contains(&self.search)
                    || item.kunyomi.contains(&self.search)
                    || item.onyomi_romaji.contains(&self.search)
                    || item.kunyomi_romaji.contains(&self.search);

                let matches_meaning = if let Some(entry) = translations.get(&item.kanji) {
                    entry.meaning.to_lowercase().contains(&search_query)
                } else {
                    false
                };

                matches_basic || matches_meaning
            })
            .collect();

        let max_width = 800.0;
        let available_width = ui.available_width();
        
        ui.vertical_centered(|ui| {
            let top_spacer = if is_search_empty {
                ui.available_height() * 0.3
            } else {
                40.0
            };
            ui.add_space(top_spacer);

            if is_search_empty {
                ui.label(
                    egui::RichText::new("Kanji Master")
                        .size(48.0)
                        .strong()
                        .family(egui::FontFamily::Proportional)
                        .color(ui.visuals().strong_text_color())
                );
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new(&self.localization.local.screens.search)
                        .size(18.0)
                        .color(ui.visuals().text_color().gamma_multiply(0.6))
                );
                ui.add_space(30.0);
            } else {
                ui.label(egui::RichText::new("Kanji Master").size(24.0).strong().color(ui.visuals().weak_text_color()));
                ui.add_space(15.0);
            }

            let base_width = if is_search_empty { 500.0 as f32 } else { 600.0 as f32 };
            let search_bar_width = base_width.min(available_width - 40.0);

            let search_bg = ui.visuals().widgets.inactive.bg_fill;
            let search_frame = egui::Frame::NONE
                .fill(search_bg)
                .corner_radius(egui::CornerRadius::same(24)) 
                .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                .inner_margin(egui::Margin::symmetric(15, 12)); 

            search_frame.show(ui, |ui| {
                ui.set_width(search_bar_width);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("🔍").size(18.0).color(ui.visuals().text_color().gamma_multiply(0.5)));
                    ui.add_space(5.0);
                    
                    let text_edit = egui::TextEdit::singleline(&mut self.search)
                        .id_source("home_search_field")
                        .hint_text(&self.localization.local.home.kanji_search_hint)
                        .frame(false)
                        .desired_width(f32::INFINITY)
                        .font(egui::FontId::proportional(20.0));

                    let output = ui.add(text_edit);

                    if self.settings.focus_on_search {
                            if self.search.is_empty() && !ui.memory(|m| m.has_focus(output.id)) {
                            output.request_focus();
                        }
                    }
                });
            });

            ui.add_space(30.0);
        });

        if is_search_empty {
            return;
        }

        if filtered_kanji.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.label(egui::RichText::new(&self.localization.local.home.kanji_not_found).weak().size(18.0));
            });
            return;
        }

        ui.separator();
        ui.add_space(10.0);

        let item_spacing = 15.0;
        let scroll_width = ui.available_width();
        
        let content_width = if scroll_width > max_width { max_width } else { scroll_width - 20.0 };
        let side_padding = (scroll_width - content_width) / 2.0;

        let columns = (content_width / 160.0).floor().max(2.0) as usize;
        let card_width = (content_width - (item_spacing * (columns as f32 - 1.0))) / columns as f32;
        let card_height = card_width * 1.1;
        let row_height = card_height + item_spacing;
        let total_rows = (filtered_kanji.len() + columns - 1) / columns;

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show_rows(ui, row_height, total_rows, |ui, row_range| {
            
            ui.spacing_mut().item_spacing.x = item_spacing;
            ui.spacing_mut().item_spacing.y = item_spacing;

            for row_index in row_range {
                ui.horizontal(|ui| {
                    ui.add_space(side_padding);

                    let start_index = row_index * columns;
                    let end_index = (start_index + columns).min(filtered_kanji.len());

                    for i in start_index..end_index {
                        if let Some(item) = filtered_kanji.get(i) {
                            let (rect, response) = ui.allocate_at_least(
                                egui::vec2(card_width, card_height),
                                egui::Sense::click(),
                            );

                            let bg_color = if response.hovered() {
                                ui.visuals().widgets.hovered.bg_fill
                            } else {
                                ui.visuals().faint_bg_color
                            };
                            
                            let stroke = if response.hovered() {
                                ui.visuals().widgets.hovered.fg_stroke
                            } else {
                                egui::Stroke::NONE
                            };

                            ui.painter().rect(
                                rect,
                                egui::CornerRadius::same(12), 
                                bg_color,
                                stroke,
                                egui::StrokeKind::Outside,
                            );

                            ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                                ui.vertical_centered(|ui| {
                                    let kanji_size = self.settings.kanji_font_size;
                                    let content_height = kanji_size + 20.0;
                                    ui.add_space((card_height - content_height) / 2.0 - 5.0);

                                    ui.label(
                                        egui::RichText::new(&item.kanji)
                                            .size(kanji_size)
                                            .color(ui.visuals().strong_text_color())
                                    );

                                    if self.settings.show_kanji_meaning {
                                        if let Some(entry) = translations.get(&item.kanji) {
                                            if !entry.meaning.is_empty() {
                                                let meaning = if entry.meaning.len() > 20 {
                                                    format!("{}...", &entry.meaning[0..18])
                                                } else {
                                                    entry.meaning.clone()
                                                };

                                                ui.label(
                                                    egui::RichText::new(meaning)
                                                        .size(12.0)
                                                        .color(ui.visuals().text_color().gamma_multiply(0.6))
                                                );
                                            }
                                        }
                                    }
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
            }
        });
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
            animation_speed: self.settings.animation_speed,
            path_to_db_core: self.paths.path_to_db_core.clone(),
            path_to_localization: self.paths.path_to_localization.clone(),
            path_to_kanji_localization: self.paths.path_to_kanji_localization.clone(),
            show_kanji_meaning: self.settings.show_kanji_meaning,
            focus_on_search: self.settings.focus_on_search,
            custom_decks: self.config.custom_decks.clone(),
        };

        if let Err(e) = save_config("config.toml", &current_config) {
            eprintln!("Error saving config: {}", e.to_string());
        }
    }

    // Top Bar
    fn top_bar(&mut self, ctx: &egui::Context) {
        let local = self.localization.local.top_bar.clone();

        let panel_frame = egui::Frame::NONE
            .fill(ctx.style().visuals.window_fill)
            .inner_margin(egui::Margin::symmetric(20, 15))
            .stroke(egui::Stroke::new(1.0, ctx.style().visuals.widgets.noninteractive.bg_stroke.color));

        egui::TopBottomPanel::top("top_panel")
            .frame(panel_frame)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    
                    let logo_text = egui::RichText::new("Kanji Master")
                        .family(egui::FontFamily::Proportional)
                        .size(22.0)
                        .strong();

                    if ui.add(egui::Label::new(logo_text).sense(egui::Sense::click())).clicked() {
                        self.current_screen = Screen::Home;
                    }

                    ui.add_space(30.0);
                    ui.add(egui::Separator::default().vertical().spacing(20.0));
                    ui.add_space(10.0);

                    self.nav_button(ui, &local.kanji.title, Screen::All);
                    self.nav_button(ui, &local.kana.title, Screen::Kana);
                    self.nav_button(ui, &local.cards.title, Screen::CardsSetup);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        
                        let settings_btn = ui.add(
                            egui::Button::new(egui::RichText::new("⚙").size(18.0))
                            .frame(false)
                        );

                        if settings_btn.clicked() {
                            self.settings_window = true;
                        }

                        if settings_btn.hovered() {
                            ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                        }

                        ui.add_space(15.0);

                        ui.menu_button(egui::RichText::new(&local.tools.title), |ui| {
                            ui.set_min_width(150.0);

                            if ui.button(&local.tools.romaji_to_kana).clicked() {
                                self.current_screen = Screen::RomajiToKana;
                                ui.close();
                            }

                            if ui.button(&local.tools.translate_kanji).clicked() {
                                self.current_screen = Screen::TranslateKana;
                                ui.close();
                            }
                        });
                    });

                });
            });
    }

    // Auxiliary function for navigation buttons
    fn nav_button(&mut self, ui: &mut egui::Ui, text: &str, target_screen: Screen) {
        let is_active = match (&self.current_screen, &target_screen) {
            (Screen::All, Screen::All) => true,
            (Screen::Kanji(_), Screen::All) => true,
            (Screen::Jlpt, Screen::All) => true,
            (Screen::Kana, Screen::Kana) => true,
            (Screen::CardsSetup, Screen::CardsSetup) => true,
            (Screen::CardsActive, Screen::CardsSetup) => true,
            (Screen::DeckManager, Screen::CardsSetup) => true,
            _ => false,
        };

        let text_color = if is_active {
            ui.visuals().text_color()
        } else {
            ui.visuals().text_color().gamma_multiply(0.6)
        };

        let rich_text = egui::RichText::new(text)
            .size(17.0)
            .color(text_color)
            .strong();

        let response = ui.add(egui::Button::new(rich_text).frame(false));

        if is_active || response.hovered() {
            let rect = response.rect;
            let stroke = egui::Stroke::new(2.0, text_color);

            ui.painter().line_segment(
                [
                    egui::pos2(rect.min.x + 4.0, rect.max.y - 2.0),
                    egui::pos2(rect.max.x - 4.0, rect.max.y - 2.0),
                ],
                stroke,
            );
        }

        if response.clicked() {
            self.current_screen = target_screen;
        }
        
        if response.hovered() {
            ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
        }

        ui.add_space(20.0);
    }

    // Ui Screen All
    fn kanji_all(&mut self, ui: &mut egui::Ui) {
        self.show_kanji(ui);
    }

    // Ui Screen JLPT 5
    fn kanji_n5(&mut self, ui: &mut egui::Ui) {
        self.selected_kanji_list = KanjiList::Jlpt5;
        self.show_kanji(ui);
    }

    // Ui Screen JLPT 4
    fn kanji_n4(&mut self, ui: &mut egui::Ui) {
        self.selected_kanji_list = KanjiList::Jlpt4;
        self.show_kanji(ui);
    }

    // Ui Screen JLPT 3
    fn kanji_n3(&mut self, ui: &mut egui::Ui) {
        self.selected_kanji_list = KanjiList::Jlpt3;
        self.show_kanji(ui);
    }

    // Ui Screen JLPT 2
    fn kanji_n2(&mut self, ui: &mut egui::Ui) {
        self.selected_kanji_list = KanjiList::Jlpt2;
        self.show_kanji(ui);
    }

    // Ui Screen JLPT 1
    fn kanji_n1(&mut self, ui: &mut egui::Ui) {
        self.selected_kanji_list = KanjiList::Jlpt1;
        self.show_kanji(ui);
    }

    // Show Kanji
    fn show_kanji(&mut self, ui: &mut egui::Ui) {
        let kanji_set = self.selected_kanji_list;
        let translations = &self.translate_state.data.entries;
        let search_query = self.search.trim().to_lowercase();
        let is_search_empty = self.search.trim().is_empty();

        let filtered_kanji: Vec<&Kanji> = self.kanji
            .iter()
            .filter(|item| {
                let matches_category = match kanji_set {
                    KanjiList::All => true,
                    KanjiList::Jlpt5 => item.jlpt == "N5",
                    KanjiList::Jlpt4 => item.jlpt == "N4",
                    KanjiList::Jlpt3 => item.jlpt == "N3",
                    KanjiList::Jlpt2 => item.jlpt == "N2",
                    KanjiList::Jlpt1 => item.jlpt == "N1",
                };

                if !matches_category {
                    return false;
                }

                if is_search_empty {
                    return true;
                }

                let matches_basic = item.kanji.contains(&self.search)
                    || item.onyomi.contains(&self.search)
                    || item.kunyomi.contains(&self.search)
                    || item.onyomi_romaji.contains(&self.search)
                    || item.kunyomi_romaji.contains(&self.search);

                let matches_meaning = if let Some(entry) = translations.get(&item.kanji) {
                    entry.meaning.to_lowercase().contains(&search_query)
                } else {
                    false
                };

                matches_basic || matches_meaning
            })
            .collect();

        ui.add_space(15.0);
        
        let max_width = 800.0f32;
        let available_width = ui.available_width();

        ui.vertical_centered(|ui| {
            let content_width = max_width.min(available_width - 40.0);
            ui.set_width(content_width);

            let pill_color = ui.visuals().widgets.inactive.bg_fill;
            let pill_stroke = ui.visuals().widgets.noninteractive.bg_stroke;
            let pill_rounding = egui::CornerRadius::same(20);
            
            let pill_frame = egui::Frame::NONE
                .fill(pill_color)
                .corner_radius(pill_rounding)
                .stroke(pill_stroke)
                .inner_margin(egui::Margin::symmetric(15, 8));

            ui.horizontal(|ui| {
                let filter_width = 110.0;
                let spacing = 10.0;
                let search_width = ui.available_width() - filter_width - spacing;

                // === A. SEARCH BAR (Left Pill) ===
                pill_frame.show(ui, |ui| {
                    ui.set_width(search_width);
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("🔍").size(16.0).color(ui.visuals().text_color().gamma_multiply(0.5)));
                        ui.add_space(5.0);
                        
                        let text_edit = egui::TextEdit::singleline(&mut self.search)
                            .id_source("list_search_field")
                            .hint_text(&self.localization.local.screens.search)
                            .frame(false) 
                            .desired_width(f32::INFINITY)
                            .font(egui::FontId::proportional(18.0))
                            .margin(egui::vec2(0.0, 2.0));
                        
                        ui.add(text_edit);
                    });
                });

                ui.add_space(spacing);

                pill_frame.show(ui, |ui| {
                    ui.set_width(filter_width);
                    ui.set_min_height(28.0); 

                    let combo_label = match self.selected_kanji_list {
                        KanjiList::All => "All",
                        KanjiList::Jlpt5 => "N5",
                        KanjiList::Jlpt4 => "N4",
                        KanjiList::Jlpt3 => "N3",
                        KanjiList::Jlpt2 => "N2",
                        KanjiList::Jlpt1 => "N1",
                    };

                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(2.0); 
                        
                        egui::ComboBox::from_id_salt("kanji_filter_combo")
                            .selected_text(egui::RichText::new(format!("⚡ {}", combo_label)).strong())
                            .width(filter_width - 20.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.selected_kanji_list, KanjiList::All, &self.localization.local.top_bar.kanji.all);
                                ui.separator();
                                ui.selectable_value(&mut self.selected_kanji_list, KanjiList::Jlpt5, "JLPT N5");
                                ui.selectable_value(&mut self.selected_kanji_list, KanjiList::Jlpt4, "JLPT N4");
                                ui.selectable_value(&mut self.selected_kanji_list, KanjiList::Jlpt3, "JLPT N3");
                                ui.selectable_value(&mut self.selected_kanji_list, KanjiList::Jlpt2, "JLPT N2");
                                ui.selectable_value(&mut self.selected_kanji_list, KanjiList::Jlpt1, "JLPT N1");
                            });
                    });
                });
            });
        });

        ui.add_space(20.0);

        if filtered_kanji.is_empty() {
            ui.centered_and_justified(|ui| {
            ui.label(egui::RichText::new(&self.localization.local.home.kanji_not_found).weak().size(18.0));
            });
            return;
        }

        let item_spacing = 15.0;
        let content_width = if available_width > max_width { max_width } else { available_width - 20.0 };
        let side_padding = (available_width - content_width) / 2.0;
        let columns = (content_width / 150.0).floor().max(2.0) as usize;
        let card_width = (content_width - (item_spacing * (columns as f32 - 1.0))) / columns as f32;
        let card_height = card_width * 1.1;
        let row_height = card_height + item_spacing;
        let total_rows = (filtered_kanji.len() + columns - 1) / columns;

        egui::ScrollArea::vertical()
            .auto_shrink([false, false]) 
            .show_rows(ui, row_height, total_rows, |ui, row_range| {
            
            ui.spacing_mut().item_spacing.x = item_spacing;
            ui.spacing_mut().item_spacing.y = item_spacing;

            for row_index in row_range {
                ui.horizontal(|ui| {
                    ui.add_space(side_padding); 
                    let start_index = row_index * columns;
                    let end_index = (start_index + columns).min(filtered_kanji.len());

                    for i in start_index..end_index {
                        if let Some(item) = filtered_kanji.get(i) {
                            let (rect, response) = ui.allocate_at_least(
                                egui::vec2(card_width, card_height),
                                egui::Sense::click(),
                            );
                            
                            let bg_color = if response.hovered() {
                                ui.visuals().widgets.hovered.bg_fill
                            } else {
                                ui.visuals().faint_bg_color
                            };
                            
                            let stroke = if response.hovered() {
                                ui.visuals().widgets.hovered.fg_stroke
                            } else {
                                egui::Stroke::NONE
                            };

                            ui.painter().rect(
                                rect,
                                egui::CornerRadius::same(12),
                                bg_color,
                                stroke,
                                egui::StrokeKind::Outside,
                            );

                            ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                                ui.vertical_centered(|ui| {
                                    let kanji_size = self.settings.kanji_font_size;
                                    let content_height = kanji_size + 20.0;
                                    ui.add_space((card_height - content_height) / 2.0 - 5.0);

                                    ui.label(
                                        egui::RichText::new(&item.kanji)
                                            .size(kanji_size)
                                            .strong()
                                            .color(ui.visuals().strong_text_color())
                                    );

                                    if self.settings.show_kanji_meaning {
                                        if let Some(entry) = translations.get(&item.kanji) {
                                            if !entry.meaning.is_empty() {
                                                let meaning_text = if entry.meaning.len() > 18 {
                                                    format!("{}...", &entry.meaning[0..16])
                                                } else {
                                                    entry.meaning.clone()
                                                };

                                                ui.label(
                                                    egui::RichText::new(meaning_text)
                                                        .size(self.settings.interface_font_size * 0.8)
                                                        .color(ui.visuals().text_color().gamma_multiply(0.7))
                                                );
                                            }
                                        }
                                    }
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
            }
        });
    }

    // Show Current Kanji
    fn kanji_screen(&mut self, ui: &mut egui::Ui, kanji: &Kanji) {
        let local = &self.localization.local.screens.current_kanji;
        let translation_entry = self.translate_state.data.entries.get(&kanji.kanji);

        let panel_rounding = egui::CornerRadius::same(16);
        let panel_fill = ui.visuals().faint_bg_color;
        let panel_stroke = ui.visuals().widgets.noninteractive.bg_stroke;
        
        let content_frame = egui::Frame::NONE
            .fill(panel_fill)
            .corner_radius(panel_rounding)
            .stroke(panel_stroke)
            .inner_margin(15.0);

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            let btn_text = egui::RichText::new(format!("⬅ {}", &local.back_button))
                .size(16.0)
                .strong();
            
            if ui.add(egui::Button::new(btn_text).frame(false)).clicked() {
                self.current_screen = Screen::All;
            }
        });
        ui.add_space(15.0);

        ui.columns(2, |columns| {
            let available_w = columns[0].available_width().min(340.0);
            let card_w = available_w * 0.94;
            let card_h = card_w * (4.0 / 3.0);

            columns[0].vertical_centered(|ui| {
                ui.add_space(5.0);

                // Card Frame
                egui::Frame::canvas(ui.style())
                    .fill(ui.visuals().window_fill)
                    .stroke(egui::Stroke::new(1.5, ui.visuals().widgets.noninteractive.bg_stroke.color))
                    .corner_radius(20)
                    .shadow(egui::Shadow::NONE) 
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

                        let anim_side = rect.width().min(rect.height()) * 0.88;
                        let center = rect.center();
                        let draw_rect = egui::Rect::from_center_size(
                            center,
                            egui::vec2(anim_side, anim_side),
                        );

                        self.animator.ui(ui, draw_rect, &kanji.kanji, self.settings.animation_speed);
                    });
                
                ui.add_space(10.0);
                ui.label(egui::RichText::new(&local.click_to_replay).size(10.0).weak());
            });

            columns[1].vertical(|ui| {
                ui.add_space(5.0);

                let meaning = if let Some(entry) = translation_entry {
                    if !entry.meaning.is_empty() { &entry.meaning } else { "" }
                } else {
                    ""
                };

                ui.vertical_centered_justified(|ui| {
                    ui.label(
                        egui::RichText::new(meaning)
                            .size(28.0)
                            .strong()
                            .color(ui.visuals().strong_text_color())
                    );
                });
                ui.add_space(15.0);

                let badge = |ui: &mut egui::Ui, label: &str, value: &str| {
                    let bg = ui.visuals().widgets.inactive.bg_fill;
                    egui::Frame::NONE
                        .fill(bg)
                        .corner_radius(12)
                        .inner_margin(egui::Margin::symmetric(12, 6))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(label).size(11.0).weak());
                                ui.label(egui::RichText::new(value).size(12.0).strong());
                            });
                        });
                };

                ui.horizontal_wrapped(|ui| {
                    badge(ui, &local.jlpt, &kanji.jlpt);
                    ui.add_space(5.0);
                    badge(ui, &local.grade, &kanji.grade);
                    ui.add_space(5.0);
                    badge(ui, &local.strokes, &format!("{}", &kanji.strokes));
                    ui.add_space(5.0);
                    badge(ui, &local.frequency, &kanji.frequency);
                });

                ui.add_space(15.0);

                content_frame.show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new(&local.onyomi).size(12.0).weak().italics());
                            ui.add_space(2.0);
                            ui.label(egui::RichText::new(&kanji.onyomi).size(16.0).strong());
                            ui.label(egui::RichText::new(&kanji.onyomi_romaji).size(12.0).color(ui.visuals().text_color().gamma_multiply(0.6)));
                        });

                        ui.add_space(20.0);
                        ui.separator();
                        ui.add_space(20.0);

                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new(&local.kunyomi).size(12.0).weak().italics());
                            ui.add_space(2.0);
                            ui.label(egui::RichText::new(&kanji.kunyomi).size(16.0).strong());
                            ui.label(egui::RichText::new(&kanji.kunyomi_romaji).size(12.0).color(ui.visuals().text_color().gamma_multiply(0.6)));
                        });
                    });
                });

                ui.add_space(20.0);

                ui.label(egui::RichText::new(&local.examples).strong().size(18.0));
                ui.add_space(8.0);

                let scroll_height = ui.available_height() - 20.0;
                
                egui::ScrollArea::vertical()
                    .max_height(scroll_height)
                    .id_salt("examples_scroll")
                    .show(ui, |ui| {
                        for (index, example_jp) in kanji.example.iter().enumerate() {
                            let translation_text = if let Some(entry) = translation_entry {
                                entry.translate_examples.get(index).cloned().unwrap_or_default()
                            } else {
                                String::new()
                            };

                            egui::Frame::NONE
                                .fill(ui.visuals().widgets.open.weak_bg_fill)
                                .corner_radius(8)
                                .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color.gamma_multiply(0.5)))
                                .inner_margin(10.0)
                                .show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new("•").color(ui.visuals().warn_fg_color)); 
                                        
                                        ui.vertical(|ui| {
                                            ui.label(egui::RichText::new(example_jp).size(15.0));
                                            
                                            if !translation_text.is_empty() {
                                                ui.add_space(2.0);
                                                ui.label(
                                                    egui::RichText::new(&translation_text)
                                                        .italics()
                                                        .size(13.0)
                                                        .color(ui.visuals().text_color().gamma_multiply(0.7))
                                                );
                                            }
                                        });
                                    });
                                });
                            
                            ui.add_space(8.0);
                        }
                        
                        if kanji.example.is_empty() {
                             ui.label(egui::RichText::new(&local.no_examples_available).weak().italics());
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

    // Cards Setup Screen
    fn ui_card_setup(&mut self, ui: &mut egui::Ui) {
        let local = self.localization.local.top_bar.cards.clone();
        ui.heading(&local.card_setup_title);
        ui.add_space(20.0);

        ui.columns(2, |ui| {
            // JLPT Levels
            ui[0].vertical(|ui| {
                ui.heading(&local.jlpt_level);
                ui.add_space(10.0);
                
                // Select card limit
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", &local.card_count));
                    ui.add(egui::DragValue::new(&mut self.card_limit).range(0..=1000));
                });

                ui.add_space(10.0);

                let levels = vec!["N5", "N4", "N3", "N2", "N1"];

                for level in levels {
                    if ui.button(format!("{} {}", &local.card_setup_start, level)).clicked() {
                        self.selected_deck_name = format!("JLPT {}", level);
                        // Filter kanji by level
                        let filtered: Vec<Kanji> = self.kanji.iter()
                            .filter(|k| k.jlpt == level)
                            .cloned()
                            .collect();
                        
                        if !filtered.is_empty() {
                            self.cards_session = Some(CardsSession::new(filtered, self.card_limit));
                            self.current_screen = Screen::CardsActive;
                        }
                    }

                    ui.add_space(5.0);
                }

            });

            // Custom Decks
            ui[1].vertical(|ui| {
                ui.heading(&local.custom_deck_title);
                ui.add_space(10.0);

                if ui.button(&local.manage_decks).clicked() {
                    self.current_screen = Screen::DeckManager;
                    self.deck_builder = DeckBuilderState::default(); // Reset
                    self.deck_builder.is_editing = false;
                }

                ui.separator();
                ui.label(format!("{}:", &local.my_decks));

                let deck_names: Vec<String> = self.config.custom_decks.keys().cloned().collect();

                if deck_names.is_empty() {
                    ui.label(&local.no_custom_decks_created);
                } else {
                    egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                        for name in deck_names {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(&name).strong());

                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if ui.button(egui::RichText::new("🗑").color(egui::Color32::RED))
                                            .on_hover_text(&local.delete_deck).clicked() {
                                                self.config.custom_decks.remove(&name);
                                                self.save();
                                            }

                                        if ui.button("✏").on_hover_text(&local.edit_deck).clicked() {
                                            if let Some(ids) = self.config.custom_decks.get(&name) {
                                                self.deck_builder.deck_name_buffer = name.clone();
                                                self.deck_builder.selected_kanji_idx = ids.clone();
                                                self.deck_builder.is_editing = true;
                                                self.current_screen = Screen::DeckManager;
                                            }
                                        }

                                        if ui.button(format!("▶ {}", &local.play_deck)).clicked() {
                                            self.selected_deck_name = name.clone();

                                            if let Some(ids) = self.config.custom_decks.get(&name) {
                                                let deck_kanji: Vec<Kanji> = self.kanji.iter()
                                                    .filter(|k| ids.contains(&k.id))
                                                    .cloned()
                                                    .collect();

                                                if !deck_kanji.is_empty() {
                                                    self.cards_session = Some(CardsSession::new(deck_kanji, 0));

                                                    self.current_screen = Screen::CardsActive;
                                                }
                                            }
                                        }
                                    });
                                });
                            });

                            ui.add_space(4.0);
                        }
                    });
                }
            })
        });

    }

    // Cards Active Screen
    fn ui_card_session(&mut self, ui: &mut egui::Ui) {
        let local = self.localization.local.top_bar.cards.clone();

        let local_common  =&self.localization.local.screens.current_kanji;

        let state_snapshot = if let Some(session) = &self.cards_session {
            if session.finished {
                Some((true, None, 0, 0, false)) // finished, no kanji
            } else {
                let current_kanji = session.queue.get(session.current_index).cloned();
                Some((
                    false, 
                    current_kanji, 
                    session.total_count, 
                    session.current_index, 
                    session.is_card_flipped
                ))
            }
        } else {
            None
        };


        if state_snapshot.is_none() {
            self.current_screen = Screen::CardsSetup;
            return;
        }

        let (finished, current_kanji_opt, total, current_idx, is_flipped) = state_snapshot.unwrap();

        if finished {
            ui.centered_and_justified(|ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(&local.session_complete);
                    if ui.button(&local.return_to_menu).clicked() {
                        self.current_screen = Screen::CardsSetup;
                        self.cards_session = None;
                    }
                });
            });

            return;
        }

        // Top panel: Progress and Exit
        ui.horizontal(|ui| {
            ui.heading(&self.selected_deck_name);
            ui.label(format!("{} {} / {}", &local.card, current_idx + 1, total));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(&local.exit_training).clicked() {
                    self.current_screen = Screen::CardsSetup;
                    self.cards_session = None;
                }
            });
        });

        ui.add_space(20.0);

        // Current Card
        if let Some(kanji) = current_kanji_opt {
            let card_size = egui::vec2(300.0, 450.0);
            
            ui.vertical_centered(|ui| {
                let (rect, response) = ui.allocate_exact_size(card_size, egui::Sense::click());

                // Draw card background
                ui.painter().rect(
                    rect,
                    10.0,
                    ui.visuals().window_fill,
                    egui::Stroke::new(2.0, ui.visuals().window_stroke.color),
                    egui::StrokeKind::Middle,
                );

                // Process click (flip card)
                if response.clicked() {
                    if let Some(session) = &mut self.cards_session {
                        session.is_card_flipped = !session.is_card_flipped;
                    }
                }
                
                // Card content
                ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(40.0);
                        
                        // Always show Kanji
                        ui.label(egui::RichText::new(&kanji.kanji).size(80.0).strong());
                        
                        ui.add_space(20.0);

                        if is_flipped {
                            // Show answer
                            ui.separator();
                            ui.add_space(10.0);

                            let meaning_text = if let Some(entry) = self.translate_state.data.entries.get(&kanji.kanji) {
                                &entry.meaning
                            } else {
                                ""
                            };

                            if !meaning_text.is_empty() {
                                ui.label(egui::RichText::new(&local_common.meaning).strong());
                                ui.label(egui::RichText::new(meaning_text).size(18.0)); 
                                ui.add_space(10.0);
                                ui.separator();
                                ui.add_space(5.0);
                            }
                            
                            ui.label(egui::RichText::new(&local.onyomi).strong());
                            ui.label(&kanji.onyomi);
                            ui.label(egui::RichText::new(&kanji.onyomi_romaji).weak());
                            
                            ui.add_space(10.0);
                            
                            ui.label(egui::RichText::new(&local.kunyomi).strong());
                            ui.label(&kanji.kunyomi);
                            ui.label(egui::RichText::new(&kanji.kunyomi_romaji).weak());

                        } else {
                            // Front side
                            ui.add_space(40.0);
                            ui.label(egui::RichText::new(&local.click_to_flip).weak().italics());
                        }
                    });
                });

                ui.add_space(20.0);

                // Button "Next"
                if ui.button(egui::RichText::new(format!("{} ➡", &local.next_card)).size(20.0)).clicked() {
                    if let Some(session) = &mut self.cards_session {
                        session.next();
                    }
                }
            });
        }

    }

    // Deck Manager Screen
    fn ui_deck_manager(&mut self, ui: &mut egui::Ui) {
        let local = self.localization.local.top_bar.cards.clone();
        ui.horizontal(|ui| {
            if ui.button(format!("⬅ {}", &local.back)).clicked() {
                self.current_screen = Screen::CardsSetup;
            }

            if self.deck_builder.is_editing {
                ui.heading(&local.edit_deck);
            } else {
                ui.heading(&local.deck_manager);
            }
        });
        ui.separator();

        // Top panel: Deck Name and Save
        ui.horizontal(|ui| {
            ui.label(format!("{}:", &local.deck_name));
            ui.text_edit_singleline(&mut self.deck_builder.deck_name_buffer);

            ui.add_enabled(!self.deck_builder.is_editing, egui::TextEdit::singleline(&mut self.deck_builder.deck_name_buffer));

            let btn_text = if self.deck_builder.is_editing { &local.update_deck } else { &local.save_deck };

            if ui.button(btn_text).clicked() {
                if !self.deck_builder.deck_name_buffer.is_empty() && !self.deck_builder.selected_kanji_idx.is_empty() {
                    // Save to config
                    self.config.custom_decks.insert(
                        self.deck_builder.deck_name_buffer.clone(),
                        self.deck_builder.selected_kanji_idx.clone()
                    );
                    self.save();
                    self.deck_builder = DeckBuilderState::default(); 
                }
            }
        });

        ui.add_space(10.0);
        ui.separator();

        // Two columns: Search and Deck Content
        ui.columns(2, |cols| {
            // Left column: Search
            cols[0].vertical(|ui| {
                ui.heading(&local.avaliable_kanji);
                ui.text_edit_singleline(&mut self.deck_builder.search_buffer).on_hover_text(&local.search_kanji_hint);
                
                let search = self.deck_builder.search_buffer.to_lowercase();
                
                egui::ScrollArea::vertical().id_salt("source_list").max_height(400.0).show(ui, |ui| {
                    // Filter all kanji
                    let filtered: Vec<&Kanji> = self.kanji.iter()
                        .filter(|k| {
                            if search.is_empty() { return false; }
                            k.kanji.contains(&search) || k.onyomi_romaji.contains(&search) || k.kunyomi_romaji.contains(&search)
                        })
                        .take(50) 
                        .collect();

                    for k in filtered {
                        ui.horizontal(|ui| {
                            ui.label(&k.kanji);
                            if ui.button(format!("{} ➡", &local.add_kanji)).clicked() {
                                if !self.deck_builder.selected_kanji_idx.contains(&k.id) {
                                    self.deck_builder.selected_kanji_idx.push(k.id);
                                }
                            }
                        });
                    }
                    if search.is_empty() {
                        ui.label(&local.type_search_hint);
                    }
                });
            });

            // Right column: Deck Content
            cols[1].vertical(|ui| {
                ui.heading(format!("{} ({})", &local.deck_content, self.deck_builder.selected_kanji_idx.len()));
                
                egui::ScrollArea::vertical().id_salt("deck_list").max_height(400.0).show(ui, |ui| {
                    
                    let mut ids_to_remove = Vec::new();

                    for (idx, id) in self.deck_builder.selected_kanji_idx.iter().enumerate() {
                        if let Some(k) = self.kanji.iter().find(|k| k.id == *id) {
                            ui.horizontal(|ui| {
                                if ui.button("❌").clicked() {
                                    ids_to_remove.push(idx);
                                }
                                ui.label(&k.kanji);
                                ui.label(egui::RichText::new(&k.onyomi).size(10.0));
                            });
                        }
                    }

                    // Delete marked
                    for idx in ids_to_remove.iter().rev() {
                        self.deck_builder.selected_kanji_idx.remove(*idx);
                    }
                });
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
                self.home_screen(ui); 
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
                Screen::Kana => self.kana_screen(ui),
                Screen::RomajiToKana => self.romaji_to_kana_screen(ui),
                Screen::TranslateKana => self.translate_kanji_screen(ui),
                Screen::CardsSetup => self.ui_card_setup(ui),
                Screen::CardsActive => self.ui_card_session(ui),
                Screen::DeckManager => self.ui_deck_manager(ui),

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
