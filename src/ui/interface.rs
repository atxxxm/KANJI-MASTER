use crate::back::cards::{CardsSession, DeckBuilderState};
use crate::back::config::{Config, save_config, get_config_path};
use crate::back::core::{Database, Kanji};
use crate::back::localization::*;
use crate::back::translation::*;
use crate::back::romaji_kana::to_kana;
use crate::ui::animator::KanjiAnimator;
use crate::ui::settings::Settings;
use eframe::egui;
use std::path::{Path, PathBuf};
use crate::ui::tabs::{KanjiDetailState, KanjiListState, RomajiKanaState, TabManager,
    TabType, TranslateTabState, CardsSetupState
}; 


pub struct AppContext<'a> {
    pub kanji: &'a Vec<Kanji>,
    pub config: &'a mut Config,
    pub paths: &'a Paths,
    pub settings: &'a Settings,
    pub localization: &'a Localization,
    pub translate_state: &'a mut TranslateState,
}

// Hiragana and Katakana Struct
struct KanaItem {
    kana: &'static str,
    romaji: &'static str,
}

// Kanji List
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KanjiList {
    All,
    Jlpt5,
    Jlpt4,
    Jlpt3,
    Jlpt2,
    Jlpt1,
}

// JLPT Levels
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(dead_code)]
enum JLPT {
    N5,
    N4,
    N3,
    N2,
    N1,
}

struct App {
    tab_manager: TabManager,

    kanji: Vec<Kanji>,
    config: Config,
    paths: Paths,
    settings: Settings,
    localization: Localization,
    translate_state: TranslateState,
    
    settings_window: bool,
    config_path: PathBuf,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        // Set Font
        set_font(&cc.egui_ctx);

        let config_path = get_config_path().expect("cannot get config path");

        // Load Base Localization
        let localization: Localization = load(&config.path_to_localization).expect("Erorr Load");
        // Load Kanji
        let kanji = Database::new(&config.path_to_db_core)
            .get_kanji()
            .expect("Error Read");

        let paths = Paths {
            path_to_db_core: config.path_to_db_core.clone(),
            path_to_localization: config.path_to_localization.clone(),
            path_to_kanji_localization: config.path_to_kanji_localization.clone(),
            path_to_svg_images: config.path_to_svg_images.clone(),
        };

        let message_data = MessageTranslationData {
            file_load_success: localization
                .local
                .top_bar
                .tools
                .translate_kanji_locale
                .file_load_success
                .clone(),
            file_not_found_or_invalid: localization
                .local
                .top_bar
                .tools
                .translate_kanji_locale
                .file_not_found_or_invalid
                .clone(),
            saved_at_id: localization
                .local
                .top_bar
                .tools
                .translate_kanji_locale
                .saved_at_id
                .clone(),
            error_searialize_json: localization
                .local
                .top_bar
                .tools
                .translate_kanji_locale
                .error_searialize_json
                .clone(),
            error_create_file: localization
                .local
                .top_bar
                .tools
                .translate_kanji_locale
                .error_create_file
                .clone(),
        };

        let mut translate_state =
            TranslateState::new(&config.path_to_kanji_localization, message_data);

        let loaded_successfully = translate_state.load(&config.path_to_kanji_localization);

        if let Some(first_kanji) = kanji.get(translate_state.current_index) {
            translate_state.sync_translation_buffers(&first_kanji);
        }

        // If the load was successful, update the current index
        if loaded_successfully {
            if let Some(pos) = kanji
                .iter()
                .position(|k| k.id == translate_state.data.last_id)
            {
                translate_state.current_index = pos;
            }
            if let Some(first_kanji) = kanji.get(translate_state.current_index) {
                translate_state.sync_translation_buffers(&first_kanji);
            }
        }

        // If loading failed, reset the state
        let settings_window = !loaded_successfully;

        Self {
            tab_manager: TabManager::new(),
            kanji,
            settings: Settings::new(localization.clone(), &config),
            config,
            paths,
            localization,
            settings_window,
            translate_state,
            config_path,
        }
    }

    // Name
    fn name() -> &'static str {
        "Kanji Master"
    }

    // Create Kanji Tab
    fn create_kanji_tab(kanji: Kanji, paths: &Paths) -> TabType {
        let mut animator = KanjiAnimator::new();
        let svg_path = format!("{}/0{}.svg", paths.path_to_svg_images, kanji.unicode.to_lowercase());

        if Path::new(&svg_path).exists() {
            if let Err(_) = animator.load_svg(&svg_path) {
                animator.clear();
            }
        } else {
            animator.clear();
        }

        TabType::KanjiDetail(KanjiDetailState {
            kanji,
            animator,
            last_copy_time: None,
        })
    }

    // Render Home
    fn render_home(ui: &mut egui::Ui, search_query: &mut String, ctx: &AppContext) -> Option<(TabType, bool)> {
        let mut tab_action = None;
        let translations = &ctx.translate_state.data.entries;
        let search_text = search_query.trim().to_lowercase();
        let is_search_empty = search_query.trim().is_empty();

        let filtered_kanji: Vec<&Kanji> = ctx.kanji
            .iter()
            .filter(|item| {
                if is_search_empty {
                    return false;
                }

                let matches_basic = item.kanji.contains(&search_query.as_str())
                    || item.onyomi.contains(&search_query.as_str())
                    || item.kunyomi.contains(&search_query.as_str())
                    || item.onyomi_romaji.contains(&search_query.as_str())
                    || item.kunyomi_romaji.contains(&search_query.as_str());

                let matches_meaning = if let Some(entry) = translations.get(&item.kanji) {
                    entry.meaning.to_lowercase().contains(&search_text)
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
                        .color(ui.visuals().strong_text_color()),
                );
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new(&ctx.localization.local.screens.search)
                        .size(18.0)
                        .color(ui.visuals().text_color().gamma_multiply(0.6)),
                );
                ui.add_space(30.0);
            } else {
                ui.label(
                    egui::RichText::new("Kanji Master")
                        .size(24.0)
                        .strong()
                        .color(ui.visuals().weak_text_color()),
                );
                ui.add_space(15.0);
            }

            let base_width = if is_search_empty {
                500.0 as f32
            } else {
                600.0 as f32
            };
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
                    ui.label(
                        egui::RichText::new("🔍")
                            .size(18.0)
                            .color(ui.visuals().text_color().gamma_multiply(0.5)),
                    );
                    ui.add_space(5.0);

                    let text_edit = egui::TextEdit::singleline(search_query)
                        .id_source("home_search_field")
                        .hint_text(&ctx.localization.local.home.kanji_search_hint)
                        .frame(false)
                        .desired_width(f32::INFINITY)
                        .font(egui::FontId::proportional(20.0));

                    let output = ui.add(text_edit);

                    if ctx.settings.focus_on_search {
                        if search_query.is_empty() && !ui.memory(|m| m.has_focus(output.id)) {
                            output.request_focus();
                        }
                    }
                });
            });

            ui.add_space(30.0);
        });

        if is_search_empty {
            return None;
        }

        if filtered_kanji.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new(&ctx.localization.local.home.kanji_not_found)
                        .weak()
                        .size(18.0),
                );
            });
            return None;
        }

        ui.separator();
        ui.add_space(10.0);

        let item_spacing = 15.0;
        let scroll_width = ui.available_width();

        let content_width = if scroll_width > max_width {
            max_width
        } else {
            scroll_width - 20.0
        };
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
                                        let kanji_size = ctx.settings.kanji_font_size;
                                        let content_height = kanji_size + 20.0;
                                        ui.add_space((card_height - content_height) / 2.0 - 5.0);

                                        ui.label(
                                            egui::RichText::new(&item.kanji)
                                                .size(kanji_size)
                                                .color(ui.visuals().strong_text_color()),
                                        );

                                        if ctx.settings.show_kanji_meaning {
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
                                                            .color(
                                                                ui.visuals()
                                                                    .text_color()
                                                                    .gamma_multiply(0.6),
                                                            ),
                                                    );
                                                }
                                            }
                                        }
                                    });
                                });

                                if response.clicked() {
                                    let new_content = Self::create_kanji_tab((*item).clone(), ctx.paths);
                                    tab_action = Some((new_content, true));
                                }

                                if response.hovered() {
                                    ui.output_mut(|o| {
                                        o.cursor_icon = egui::CursorIcon::PointingHand
                                    });
                                }
                            }
                        }
                    });
                }
            });

        tab_action
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
        if self
            .translate_state
            .load(&self.paths.path_to_kanji_localization)
        {
            // If successful, update the position
            if let Some(pos) = self
                .kanji
                .iter()
                .position(|k| k.id == self.translate_state.data.last_id)
            {
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
            path_to_svg_images: self.paths.path_to_svg_images.clone(),
        };

        if let Err(e) = save_config(&self.config_path,&current_config) {
            eprintln!("Error saving config: {}", e.to_string());
        }
    }

    // Top Bar
    fn top_bar(&mut self, ctx: &egui::Context) {
        let local = self.localization.local.top_bar.clone();

        let active_tab_type = self.tab_manager.tabs
            .get(self.tab_manager.active_tab_index)
            .map(|t| &t.content);


        let is_kanji_active = matches!(active_tab_type, Some(TabType::KanjiList(_)));
        let is_kana_active = matches!(active_tab_type, Some(TabType::Kana(_)));
        let is_cards_active = matches!(active_tab_type, Some(TabType::CardsSetup(_)) | Some(TabType::CardsActive(_, _)) | Some(TabType::DeckManager(_)));

        let panel_frame = egui::Frame::NONE
            .fill(ctx.style().visuals.window_fill)
            .inner_margin(egui::Margin::symmetric(20, 15))
            .stroke(egui::Stroke::new(
                1.0,
                ctx.style().visuals.widgets.noninteractive.bg_stroke.color,
            ));

        egui::TopBottomPanel::top("top_panel")
            .frame(panel_frame)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let logo_text = egui::RichText::new("Kanji Master")
                        .family(egui::FontFamily::Proportional)
                        .size(22.0)
                        .strong();

                    if ui
                        .add(egui::Label::new(logo_text).sense(egui::Sense::click()))
                        .clicked()
                    {
                        self.tab_manager.add_tab(TabType::Home(String::new()), true);
                    }

                    ui.add_space(30.0);
                    ui.add(egui::Separator::default().vertical().spacing(20.0));
                    ui.add_space(10.0);

                    
                    self.nav_button(ui, &local.kanji.title, is_kanji_active, || {
                        TabType::KanjiList(KanjiListState::default())
                    });

                    
                    self.nav_button(ui, &local.kana.title, is_kana_active, || {
                        TabType::Kana(false) 
                    });

                    
                    self.nav_button(ui, &local.cards.title, is_cards_active, || {
                        TabType::CardsSetup(CardsSetupState::default())
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let settings_btn = ui.add(
                            egui::Button::new(egui::RichText::new("⚙").size(18.0)).frame(false),
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
                                self.tab_manager.add_tab(TabType::RomajiToKana(RomajiKanaState::default()), true);
                                ui.close();
                            }

                            if ui.button(&local.tools.translate_kanji).clicked() {
                                self.tab_manager.add_tab(TabType::Translate(TranslateTabState::default()), true);
                                ui.close();
                            }
                        });
                    });
                });
            });
    }

    // Auxiliary function for navigation buttons
    fn nav_button(&mut self, ui: &mut egui::Ui, text: &str, is_active: bool, create_tab_fn: impl FnOnce() -> TabType) {
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
            let new_tab_content = create_tab_fn();
            self.tab_manager.add_tab(new_tab_content, true);
        }

        if response.hovered() {
            ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
        }

        ui.add_space(20.0);
    }

    // Render Kanji List
    fn render_kanji_list(ui: &mut egui::Ui, state: &mut KanjiListState, ctx: &AppContext) -> Option<(TabType, bool)> {
        let mut tab_action = None;
        
        let kanji_set = state.selected_list;
        let translations = &ctx.translate_state.data.entries;
        let search_query = state.search_query.trim().to_lowercase();
        let is_search_empty = search_query.trim().is_empty();

        let filtered_kanji: Vec<&Kanji> = ctx.kanji
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

                let matches_basic = item.kanji.contains(&search_query)
                    || item.onyomi.contains(&search_query)
                    || item.kunyomi.contains(&search_query)
                    || item.onyomi_romaji.contains(&search_query)
                    || item.kunyomi_romaji.contains(&search_query);

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
                        ui.label(
                            egui::RichText::new("🔍")
                                .size(16.0)
                                .color(ui.visuals().text_color().gamma_multiply(0.5)),
                        );
                        ui.add_space(5.0);

                        let text_edit = egui::TextEdit::singleline(&mut state.search_query)
                            .id_source("list_search_field")
                            .hint_text(&ctx.localization.local.screens.search)
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

                    let combo_label = match state.selected_list {
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
                            .selected_text(
                                egui::RichText::new(format!("⚡ {}", combo_label)).strong(),
                            )
                            .width(filter_width - 20.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut state.selected_list,
                                    KanjiList::All,
                                    &ctx.localization.local.top_bar.kanji.all,
                                );
                                ui.separator();
                                ui.selectable_value(
                                    &mut state.selected_list,
                                    KanjiList::Jlpt5,
                                    "JLPT N5",
                                );
                                ui.selectable_value(
                                    &mut state.selected_list,
                                    KanjiList::Jlpt4,
                                    "JLPT N4",
                                );
                                ui.selectable_value(
                                    &mut state.selected_list,
                                    KanjiList::Jlpt3,
                                    "JLPT N3",
                                );
                                ui.selectable_value(
                                    &mut state.selected_list,
                                    KanjiList::Jlpt2,
                                    "JLPT N2",
                                );
                                ui.selectable_value(
                                    &mut state.selected_list,
                                    KanjiList::Jlpt1,
                                    "JLPT N1",
                                );
                            });
                    });
                });
            });
        });

        ui.add_space(20.0);

        if filtered_kanji.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label(
                    egui::RichText::new(&ctx.localization.local.home.kanji_not_found)
                        .weak()
                        .size(18.0),
                );
            });
            return None;
        }

        let item_spacing = 15.0;
        let content_width = if available_width > max_width {
            max_width
        } else {
            available_width - 20.0
        };
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
                                        let kanji_size = ctx.settings.kanji_font_size;
                                        let content_height = kanji_size + 20.0;
                                        ui.add_space((card_height - content_height) / 2.0 - 5.0);

                                        ui.label(
                                            egui::RichText::new(&item.kanji)
                                                .size(kanji_size)
                                                .strong()
                                                .color(ui.visuals().strong_text_color()),
                                        );

                                        if ctx.settings.show_kanji_meaning {
                                            if let Some(entry) = translations.get(&item.kanji) {
                                                if !entry.meaning.is_empty() {
                                                    let meaning_text = if entry.meaning.len() > 18 {
                                                        format!("{}...", &entry.meaning[0..16])
                                                    } else {
                                                        entry.meaning.clone()
                                                    };

                                                    ui.label(
                                                        egui::RichText::new(meaning_text)
                                                            .size(
                                                                ctx.settings.interface_font_size
                                                                    * 0.8,
                                                            )
                                                            .color(
                                                                ui.visuals()
                                                                    .text_color()
                                                                    .gamma_multiply(0.7),
                                                            ),
                                                    );
                                                }
                                            }
                                        }
                                    });
                                });

                                if response.clicked() {
                                    let new_content = Self::create_kanji_tab((*item).clone(), ctx.paths);
                                    tab_action = Some((new_content, false));
                                }

                                if response.secondary_clicked() {
                                    let new_content = Self::create_kanji_tab((*item).clone(), ctx.paths);
                                    tab_action = Some((new_content, true));
                                }

                                if response.hovered() {
                                    ui.output_mut(|o| {
                                        o.cursor_icon = egui::CursorIcon::PointingHand
                                    });
                                }
                            }
                        }
                    });
                }
            });

        tab_action
    }

    // Render Kanji Detail
    fn render_kanji_detail(ui: &mut egui::Ui, state: &mut KanjiDetailState, ctx: &AppContext) -> Option<(TabType, bool)> {
        let kanji = &state.kanji;
        let local = &ctx.localization.local.screens.current_kanji;
        let translation_entry = ctx.translate_state.data.entries.get(&kanji.kanji);

        let panel_rounding = egui::CornerRadius::same(16);
        let panel_fill = ui.visuals().faint_bg_color;
        let panel_stroke = ui.visuals().widgets.noninteractive.bg_stroke;

        let content_frame = egui::Frame::NONE
            .fill(panel_fill)
            .corner_radius(panel_rounding)
            .stroke(panel_stroke)
            .inner_margin(15.0);

        ui.add_space(25.0);

        ui.columns(2, |columns| {
            let available_w = columns[0].available_width().min(340.0);
            let card_w = available_w * 0.94;
            let card_h = card_w * (4.0 / 3.0);

            columns[0].vertical_centered(|ui| {
                ui.add_space(5.0);

                egui::Frame::canvas(ui.style())
                    .fill(ui.visuals().window_fill)
                    .stroke(egui::Stroke::new(
                        1.5,
                        ui.visuals().widgets.noninteractive.bg_stroke.color,
                    ))
                    .corner_radius(20)
                    .shadow(egui::Shadow::NONE)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        let (rect, response) = ui
                            .allocate_exact_size(egui::vec2(card_w, card_h), egui::Sense::click());

                        if response.clicked() {
                            state.animator.replay();
                        }

                        if response.hovered() {
                            ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                        }

                        let anim_side = rect.width().min(rect.height()) * 0.88;
                        let center = rect.center();
                        let draw_rect =
                            egui::Rect::from_center_size(center, egui::vec2(anim_side, anim_side));

                        state.animator.ui(ui, draw_rect, &kanji.kanji, ctx.settings.animation_speed);
                    });

                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new(&local.click_to_replay)
                        .size(10.0)
                        .weak(),
                );

                ui.add_space(15.0);

                let current_time = ui.input(|i| i.time);
                let mut show_checkmark = false;

                if let Some(last_time) = state.last_copy_time {
                    if current_time - last_time < 2.0 {
                        show_checkmark = true;
                        ui.ctx().request_repaint();
                    }
                }

                let btn_bg = ui.visuals().widgets.inactive.bg_fill;
                let btn_stroke = ui.visuals().widgets.noninteractive.bg_stroke;

                let (icon, label_text) = if show_checkmark {
                    ("✅", "Copied")
                } else {
                    ("📋", "Copy")
                };
                
                let btn_text = egui::RichText::new(format!("{} {}", icon, label_text))
                    .size(12.0)
                    .strong();

                let copy_btn = egui::Button::new(btn_text)
                    .fill(btn_bg)
                    .stroke(btn_stroke)
                    .corner_radius(12.0)
                    .min_size(egui::vec2(100.0, 24.0));

                if ui.add(copy_btn)
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .on_hover_text("Copy to clipboard") 
                    .clicked() 
                {
                    ui.ctx().copy_text(kanji.kanji.clone());
                    state.last_copy_time = Some(current_time);
                }
            }); 

            columns[1].vertical(|ui| {
                ui.add_space(5.0);

                let meaning = if let Some(entry) = translation_entry {
                    if !entry.meaning.is_empty() {
                        &entry.meaning
                    } else {
                        ""
                    }
                } else {
                    ""
                };

                ui.vertical_centered_justified(|ui| {
                    ui.label(
                        egui::RichText::new(meaning)
                            .size(28.0)
                            .strong()
                            .color(ui.visuals().strong_text_color()),
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
                            ui.label(
                                egui::RichText::new(&local.onyomi)
                                    .size(12.0)
                                    .weak()
                                    .italics(),
                            );
                            ui.add_space(2.0);
                            ui.label(egui::RichText::new(&kanji.onyomi).size(16.0).strong());
                            ui.label(
                                egui::RichText::new(&kanji.onyomi_romaji)
                                    .size(12.0)
                                    .color(ui.visuals().text_color().gamma_multiply(0.6)),
                            );
                        });

                        ui.add_space(20.0);
                        ui.separator();
                        ui.add_space(20.0);

                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new(&local.kunyomi)
                                    .size(12.0)
                                    .weak()
                                    .italics(),
                            );
                            ui.add_space(2.0);
                            ui.label(egui::RichText::new(&kanji.kunyomi).size(16.0).strong());
                            ui.label(
                                egui::RichText::new(&kanji.kunyomi_romaji)
                                    .size(12.0)
                                    .color(ui.visuals().text_color().gamma_multiply(0.6)),
                            );
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
                                entry
                                    .translate_examples
                                    .get(index)
                                    .cloned()
                                    .unwrap_or_default()
                            } else {
                                String::new()
                            };

                            egui::Frame::NONE
                                .fill(ui.visuals().widgets.open.weak_bg_fill)
                                .corner_radius(8)
                                .stroke(egui::Stroke::new(
                                    1.0,
                                    ui.visuals()
                                        .widgets
                                        .noninteractive
                                        .bg_stroke
                                        .color
                                        .gamma_multiply(0.5),
                                ))
                                .inner_margin(10.0)
                                .show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            egui::RichText::new("•")
                                                .color(ui.visuals().warn_fg_color),
                                        );

                                        ui.vertical(|ui| {
                                            ui.label(egui::RichText::new(example_jp).size(15.0));

                                            if !translation_text.is_empty() {
                                                ui.add_space(2.0);
                                                ui.label(
                                                    egui::RichText::new(&translation_text)
                                                        .italics()
                                                        .size(13.0)
                                                        .color(
                                                            ui.visuals()
                                                                .text_color()
                                                                .gamma_multiply(0.7),
                                                        ),
                                                );
                                            }
                                        });
                                    });
                                });

                            ui.add_space(8.0);
                        }

                        if kanji.example.is_empty() {
                            ui.label(
                                egui::RichText::new(&local.no_examples_available)
                                    .weak()
                                    .italics(),
                            );
                        }
                    });
            });
        });
        None
    }

    // Render Kana
    fn render_kana(ui: &mut egui::Ui, is_katakana: &mut bool, ctx: &AppContext) -> Option<(TabType, bool)> {
        let hiragana_list: Vec<KanaItem> = vec![
            KanaItem {
                kana: "あ",
                romaji: "a",
            },
            KanaItem {
                kana: "い",
                romaji: "i",
            },
            KanaItem {
                kana: "う",
                romaji: "u",
            },
            KanaItem {
                kana: "え",
                romaji: "e",
            },
            KanaItem {
                kana: "お",
                romaji: "o",
            },
            KanaItem {
                kana: "か",
                romaji: "ka",
            },
            KanaItem {
                kana: "き",
                romaji: "ki",
            },
            KanaItem {
                kana: "く",
                romaji: "ku",
            },
            KanaItem {
                kana: "け",
                romaji: "ke",
            },
            KanaItem {
                kana: "こ",
                romaji: "ko",
            },
            KanaItem {
                kana: "さ",
                romaji: "sa",
            },
            KanaItem {
                kana: "し",
                romaji: "shi",
            },
            KanaItem {
                kana: "す",
                romaji: "su",
            },
            KanaItem {
                kana: "せ",
                romaji: "se",
            },
            KanaItem {
                kana: "そ",
                romaji: "so",
            },
            KanaItem {
                kana: "た",
                romaji: "ta",
            },
            KanaItem {
                kana: "ち",
                romaji: "chi",
            },
            KanaItem {
                kana: "つ",
                romaji: "tsu",
            },
            KanaItem {
                kana: "て",
                romaji: "te",
            },
            KanaItem {
                kana: "と",
                romaji: "to",
            },
            KanaItem {
                kana: "な",
                romaji: "na",
            },
            KanaItem {
                kana: "に",
                romaji: "ni",
            },
            KanaItem {
                kana: "ぬ",
                romaji: "nu",
            },
            KanaItem {
                kana: "ね",
                romaji: "ne",
            },
            KanaItem {
                kana: "の",
                romaji: "no",
            },
            KanaItem {
                kana: "は",
                romaji: "ha",
            },
            KanaItem {
                kana: "ひ",
                romaji: "hi",
            },
            KanaItem {
                kana: "ふ",
                romaji: "fu",
            },
            KanaItem {
                kana: "へ",
                romaji: "he",
            },
            KanaItem {
                kana: "ほ",
                romaji: "ho",
            },
            KanaItem {
                kana: "ま",
                romaji: "ma",
            },
            KanaItem {
                kana: "み",
                romaji: "mi",
            },
            KanaItem {
                kana: "む",
                romaji: "mu",
            },
            KanaItem {
                kana: "め",
                romaji: "me",
            },
            KanaItem {
                kana: "も",
                romaji: "mo",
            },
            KanaItem {
                kana: "や",
                romaji: "ya",
            },
            KanaItem {
                kana: "ゆ",
                romaji: "yu",
            },
            KanaItem {
                kana: "よ",
                romaji: "yo",
            },
            KanaItem {
                kana: "ら",
                romaji: "ra",
            },
            KanaItem {
                kana: "り",
                romaji: "ri",
            },
            KanaItem {
                kana: "る",
                romaji: "ru",
            },
            KanaItem {
                kana: "れ",
                romaji: "re",
            },
            KanaItem {
                kana: "ろ",
                romaji: "ro",
            },
            KanaItem {
                kana: "わ",
                romaji: "wa",
            },
            KanaItem {
                kana: "を",
                romaji: "wo",
            },
            KanaItem {
                kana: "ん",
                romaji: "n",
            },
        ];

        let katakana_list: Vec<KanaItem> = vec![
            KanaItem {
                kana: "ア",
                romaji: "a",
            },
            KanaItem {
                kana: "イ",
                romaji: "i",
            },
            KanaItem {
                kana: "ウ",
                romaji: "u",
            },
            KanaItem {
                kana: "エ",
                romaji: "e",
            },
            KanaItem {
                kana: "オ",
                romaji: "o",
            },
            KanaItem {
                kana: "カ",
                romaji: "ka",
            },
            KanaItem {
                kana: "キ",
                romaji: "ki",
            },
            KanaItem {
                kana: "ク",
                romaji: "ku",
            },
            KanaItem {
                kana: "ケ",
                romaji: "ke",
            },
            KanaItem {
                kana: "コ",
                romaji: "ko",
            },
            KanaItem {
                kana: "サ",
                romaji: "sa",
            },
            KanaItem {
                kana: "シ",
                romaji: "shi",
            },
            KanaItem {
                kana: "ス",
                romaji: "su",
            },
            KanaItem {
                kana: "セ",
                romaji: "se",
            },
            KanaItem {
                kana: "ソ",
                romaji: "so",
            },
            KanaItem {
                kana: "タ",
                romaji: "ta",
            },
            KanaItem {
                kana: "チ",
                romaji: "chi",
            },
            KanaItem {
                kana: "ツ",
                romaji: "tsu",
            },
            KanaItem {
                kana: "テ",
                romaji: "te",
            },
            KanaItem {
                kana: "ト",
                romaji: "to",
            },
            KanaItem {
                kana: "ナ",
                romaji: "na",
            },
            KanaItem {
                kana: "ニ",
                romaji: "ni",
            },
            KanaItem {
                kana: "ヌ",
                romaji: "nu",
            },
            KanaItem {
                kana: "ネ",
                romaji: "ne",
            },
            KanaItem {
                kana: "ノ",
                romaji: "no",
            },
            KanaItem {
                kana: "ハ",
                romaji: "ha",
            },
            KanaItem {
                kana: "ヒ",
                romaji: "hi",
            },
            KanaItem {
                kana: "フ",
                romaji: "fu",
            },
            KanaItem {
                kana: "ヘ",
                romaji: "he",
            },
            KanaItem {
                kana: "ホ",
                romaji: "ho",
            },
            KanaItem {
                kana: "マ",
                romaji: "ma",
            },
            KanaItem {
                kana: "ミ",
                romaji: "mi",
            },
            KanaItem {
                kana: "ム",
                romaji: "mu",
            },
            KanaItem {
                kana: "メ",
                romaji: "me",
            },
            KanaItem {
                kana: "モ",
                romaji: "mo",
            },
            KanaItem {
                kana: "ヤ",
                romaji: "ya",
            },
            KanaItem {
                kana: "ユ",
                romaji: "yu",
            },
            KanaItem {
                kana: "ヨ",
                romaji: "yo",
            },
            KanaItem {
                kana: "ラ",
                romaji: "ra",
            },
            KanaItem {
                kana: "リ",
                romaji: "ri",
            },
            KanaItem {
                kana: "ル",
                romaji: "ru",
            },
            KanaItem {
                kana: "レ",
                romaji: "re",
            },
            KanaItem {
                kana: "ロ",
                romaji: "ro",
            },
            KanaItem {
                kana: "ワ",
                romaji: "wa",
            },
            KanaItem {
                kana: "ヲ",
                romaji: "wo",
            },
            KanaItem {
                kana: "ン",
                romaji: "n",
            },
        ];

        let current_list = if *is_katakana {
            &katakana_list
        } else {
            &hiragana_list
        };
        let local = &ctx.localization.local.kana;

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new(&local.title)
                    .size(24.0)
                    .strong()
                    .color(ui.visuals().strong_text_color()),
            );
            ui.add_space(15.0);

            let pill_bg = ui.visuals().widgets.inactive.bg_fill;
            let pill_stroke = ui.visuals().widgets.noninteractive.bg_stroke;

            ui.horizontal(|ui| {
                let pill_width = 213.0;
                ui.add_space((ui.available_width() - pill_width) / 2.0);

                egui::Frame::NONE
                    .fill(pill_bg)
                    .corner_radius(20)
                    .stroke(pill_stroke)
                    .inner_margin(egui::Margin::symmetric(6, 4))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;

                            let toggle_btn = |ui: &mut egui::Ui, text: &str, selected: bool| {
                                let text_color = if selected {
                                    ui.visuals().strong_text_color()
                                } else {
                                    ui.visuals().text_color().gamma_multiply(0.6)
                                };
                                let btn = egui::Button::new(
                                    egui::RichText::new(text).color(text_color).strong(),
                                )
                                .frame(false)
                                .min_size(egui::vec2(100.0, 30.0));
                                let response = ui.add(btn);
                                if selected {
                                    ui.painter().text(
                                        response.rect.center(),
                                        egui::Align2::CENTER_CENTER,
                                        text,
                                        egui::FontId::proportional(14.0),
                                        text_color,
                                    );
                                }
                                response
                            };

                            if toggle_btn(ui, &local.hiragana, !*is_katakana).clicked() {
                                *is_katakana = false;
                            }

                            ui.allocate_ui(egui::vec2(1.0, 20.0), |ui| {
                                ui.painter().line_segment(
                                    [ui.min_rect().center_top(), ui.min_rect().center_bottom()],
                                    ui.visuals().widgets.noninteractive.bg_stroke,
                                );
                            });

                            if toggle_btn(ui, &local.katakana, *is_katakana).clicked() {
                                *is_katakana = true;
                            }
                        });
                    });
            });
        });
        ui.add_space(20.0);

        let max_width = 1200.0;
        let available_width = ui.available_width();

        let content_width = if available_width > max_width {
            max_width
        } else {
            available_width - 20.0
        };
        let side_padding = (available_width - content_width) / 2.0;

        let columns = 5;
        let spacing = 15.0;

        let card_width = (content_width - (spacing * (columns as f32 - 1.0))) / columns as f32;
        let card_height = card_width * 1.2;

        let total_rows = (current_list.len() + columns - 1) / columns;
        let row_height = card_height + spacing;

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show_rows(ui, row_height, total_rows, |ui, row_range| {
                ui.spacing_mut().item_spacing.y = spacing;

                for row_index in row_range {
                    ui.horizontal(|ui| {
                        ui.add_space(side_padding);
                        ui.spacing_mut().item_spacing.x = spacing;

                        let start_index = row_index * columns;
                        let end_index = (start_index + columns).min(current_list.len());

                        for i in start_index..end_index {
                            if let Some(item) = current_list.get(i) {
                                let (rect, response) = ui.allocate_at_least(
                                    egui::vec2(card_width, card_height),
                                    egui::Sense::hover(),
                                );

                                let is_hovered = response.hovered();

                                let bg_color = if is_hovered {
                                    ui.visuals().widgets.hovered.bg_fill
                                } else {
                                    ui.visuals().faint_bg_color
                                };

                                let border_stroke = if is_hovered {
                                    ui.visuals().widgets.hovered.fg_stroke
                                } else {
                                    egui::Stroke::new(
                                        1.0,
                                        ui.visuals()
                                            .widgets
                                            .noninteractive
                                            .bg_stroke
                                            .color
                                            .gamma_multiply(0.5),
                                    )
                                };

                                ui.painter().rect(
                                    rect,
                                    egui::CornerRadius::same(12),
                                    bg_color,
                                    border_stroke,
                                    egui::StrokeKind::Outside,
                                );

                                ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                                    ui.vertical_centered(|ui| {
                                        let kana_size = (card_width * 0.45).clamp(24.0, 64.0);
                                        let romaji_size = (card_width * 0.15).clamp(14.0, 24.0);

                                        let total_h = kana_size + romaji_size + 5.0;
                                        ui.add_space((card_height - total_h) / 2.0);

                                        ui.label(
                                            egui::RichText::new(item.kana)
                                                .size(kana_size)
                                                .strong()
                                                .color(ui.visuals().strong_text_color()),
                                        );

                                        ui.add_space(2.0);

                                        ui.label(
                                            egui::RichText::new(item.romaji)
                                                .size(romaji_size)
                                                .color(
                                                    ui.visuals().text_color().gamma_multiply(0.6),
                                                ),
                                        );
                                    });
                                });

                                if is_hovered {
                                    ui.output_mut(|o| {
                                        o.cursor_icon = egui::CursorIcon::PointingHand
                                    });
                                }
                            }
                        }
                    });
                }
            });
        None
    }

    // Render Romaji to Kana
    fn render_romaji_to_kana(ui: &mut egui::Ui, state: &mut RomajiKanaState, ctx: &AppContext) -> Option<(TabType, bool)> {
        let local = &ctx.localization.local.top_bar.tools.romaji_to_kana_locale;

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            ui.label(egui::RichText::new(&local.title).size(24.0).strong());
        });

        ui.add_space(15.0);

        let pill_bg = ui.visuals().widgets.inactive.bg_fill;
        let pill_stroke = ui.visuals().widgets.noninteractive.bg_stroke;
        let pill_width = 213.0;
        ui.horizontal(|ui| {
            ui.add_space((ui.available_width() - pill_width) / 2.0);
            egui::Frame::NONE
                .fill(pill_bg)
                .corner_radius(20)
                .stroke(pill_stroke)
                .inner_margin(egui::Margin::symmetric(6, 4))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;

                        let toggle_btn =
                            |ui: &mut egui::Ui, text: &str, selected: bool| -> egui::Response {
                                let text_color = if selected {
                                    ui.visuals().strong_text_color()
                                } else {
                                    ui.visuals().text_color().gamma_multiply(0.6)
                                };

                                let btn = egui::Button::new(
                                    egui::RichText::new(text).color(text_color).strong(),
                                )
                                .frame(false)
                                .min_size(egui::vec2(100.0, 30.0));

                                ui.add(btn)
                            };

                        if toggle_btn(ui, &local.output_hiragana, !state.is_katakana).clicked() {
                            state.is_katakana = false;
                        }

                        ui.allocate_ui(egui::vec2(1.0, 20.0), |ui| {
                            ui.painter().line_segment(
                                [ui.min_rect().center_top(), ui.min_rect().center_bottom()],
                                ui.visuals().widgets.noninteractive.bg_stroke,
                            );
                        });

                        if toggle_btn(ui, &local.output_katakana, state.is_katakana).clicked() {
                            state.is_katakana = true;
                        }
                    });
                });
        });

        ui.add_space(20.0);

        let panel_rounding = egui::CornerRadius::same(12);
        let panel_bg = ui.visuals().faint_bg_color;
        let panel_stroke = ui.visuals().widgets.noninteractive.bg_stroke;

        ui.columns(2, |columns| {
            columns[0].vertical(|ui| {
                ui.label(egui::RichText::new(&local.input).strong().size(16.0));
                ui.add_space(5.0);

                egui::Frame::NONE
                    .fill(ui.visuals().window_fill)
                    .stroke(panel_stroke)
                    .corner_radius(panel_rounding)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut state.input)
                                .hint_text(&local.hint_input)
                                .desired_width(f32::INFINITY)
                                .min_size(egui::vec2(0.0, 300.0))
                                .frame(false),
                        );
                    });
            });

            columns[1].vertical(|ui| {
                ui.label(egui::RichText::new(&local.output).strong().size(16.0));
                ui.add_space(5.0);

                let mut output_text = to_kana(&state.input, state.is_katakana);

                egui::Frame::NONE
                    .fill(panel_bg)
                    .stroke(panel_stroke)
                    .corner_radius(panel_rounding)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt("output_scroll")
                            .min_scrolled_height(300.0)
                            .show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut output_text)
                                        .desired_width(f32::INFINITY)
                                        .min_size(egui::vec2(0.0, 300.0))
                                        .font(egui::FontId::proportional(28.0)) // Larger font for Kana
                                        .frame(false),
                                );
                            });
                    });

                ui.add_space(10.0);

                ui.vertical_centered_justified(|ui| {
                    let btn = egui::Button::new(
                        egui::RichText::new(format!("📋 {}", &local.copy_button)).size(16.0),
                    )
                    .min_size(egui::vec2(0.0, 40.0));

                    if ui.add(btn).clicked() {
                        ui.ctx().copy_text(output_text);
                    }
                });
            });
        });

        None
    }

    // Render Translate Kanji
    fn render_translate_kanji(ui: &mut egui::Ui, state: &mut TranslateTabState, ctx: &mut AppContext) -> Option<(TabType, bool)>{
        let local = &ctx.localization.local.top_bar.tools.translate_kanji_locale;
        let input_rounding = egui::CornerRadius::same(12);
        let card_rounding = egui::CornerRadius::same(16);
        let panel_bg = ui.visuals().faint_bg_color;
        let border_stroke = ui.visuals().widgets.noninteractive.bg_stroke;

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            let pill_bg = ui.visuals().widgets.inactive.bg_fill;
            let pill_frame = egui::Frame::NONE
                .fill(pill_bg)
                .corner_radius(20)
                .stroke(border_stroke)
                .inner_margin(egui::Margin::symmetric(10, 6));

            pill_frame.show(ui, |ui| {
                ui.set_min_width(300.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("🔍").size(14.0).color(ui.visuals().text_color().gamma_multiply(0.5)));

                    let text_edit = egui::TextEdit::singleline(&mut state.jump_search_buffer)
                        .desired_width(180.0)
                        .hint_text(&local.hint_kanji_input)
                        .frame(false);

                    let response = ui.add(text_edit);
                    let popup_id = response.id.with("search_popup");

                    if response.has_focus() && !state.jump_search_buffer.is_empty() {
                        egui::Popup::open_id(ui.ctx(), popup_id);
                    }

                    if egui::Popup::is_id_open(ui.ctx(), popup_id) && !state.jump_search_buffer.is_empty() {
                        let search = state.jump_search_buffer.trim().to_lowercase();

                        let matches: Vec<(usize, &Kanji)> = ctx.kanji.iter().enumerate()
                            .filter(|(_, k)| {
                                k.kanji.contains(&search) ||
                                k.id.to_string() == search ||
                                k.onyomi_romaji.to_lowercase().contains(&search) ||
                                k.kunyomi_romaji.to_lowercase().contains(&search) ||
                                k.onyomi.contains(&search) ||
                                k.kunyomi.contains(&search)
                            })
                            .take(15)
                            .collect();

                        if !matches.is_empty() {
                            egui::Area::new(popup_id)
                                .order(egui::Order::Foreground)
                                .fixed_pos(response.rect.left_bottom() + egui::vec2(0.0, 5.0))
                                .constrain_to(ui.ctx().content_rect())
                                .show(ui.ctx(), |ui| {
                                    egui::Frame::popup(ui.style()).show(ui, |ui| {
                                        ui.set_min_width(260.0);
                                        
                                        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                                            for (idx, k) in matches {
                                                let label_text = format!(
                                                    "{} ({} / {}) [ID: {}]",
                                                    k.kanji, k.onyomi_romaji, k.kunyomi_romaji, k.id
                                                );

                                                if ui.selectable_label(false, label_text).clicked() {
                                                    ctx.translate_state.current_index = idx;

                                                    if let Some(kanji) = ctx.kanji.get(idx) {
                                                        ctx.translate_state.sync_translation_buffers(kanji);
                                                    }
                                                    ctx.translate_state.status_message = local.found.clone();
                                                    state.jump_search_buffer.clear();
                                                    egui::Popup::close_id(ui.ctx(), popup_id);
                                                }
                                            }
                                        });
                                    });
                                });
                        }
                    }
                });
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let save_btn = egui::Button::new(format!("💾 {}", &local.save_progress_button))
                    .min_size(egui::vec2(100.0, 30.0));
                
                if ui.add(save_btn).clicked() {
                    ctx.translate_state.save_translations(&ctx.kanji);
                }

                ui.add_space(10.0);
                
                if !ctx.translate_state.status_message.is_empty() {
                    let color = if ctx.translate_state.status_message == local.not_found {
                        ui.visuals().warn_fg_color
                    } else {
                        egui::Color32::GREEN
                    };
                    
                    ui.label(
                        egui::RichText::new(&ctx.translate_state.status_message)
                            .color(color)
                            .italics()
                    );
                }
            });
        });

        ui.add_space(10.0);
        ui.separator();

        if ctx.translate_state.current_index >= ctx.kanji.len() {
            ui.centered_and_justified(|ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("🎉");
                    ui.heading(&local.all_kanji_processed);
                });
            });
            return None;
        }

        let current_kanji = ctx.kanji[ctx.translate_state.current_index].clone();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_space(20.0);

            ui.vertical(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new(&current_kanji.kanji)
                            .size(80.0)
                            .strong()
                            .color(ui.visuals().strong_text_color())
                    );
                    
                    ui.add_space(10.0);

                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Min).with_main_align(egui::Align::Center), |ui| {
                        let chip = |ui: &mut egui::Ui, text: String| {
                            let bg = ui.visuals().widgets.inactive.bg_fill;
                            egui::Frame::NONE
                                .fill(bg)
                                .corner_radius(12)
                                .inner_margin(egui::Margin::symmetric(12, 6))
                                .show(ui, |ui| {
                                    ui.label(egui::RichText::new(text).size(12.0).weak());
                                });
                        };

                        chip(ui, format!("ID: {}", current_kanji.id));
                        ui.add_space(10.0);
                        chip(ui, format!("Index: {} / {}", ctx.translate_state.current_index + 1, ctx.kanji.len()));
                    });
                });
            });

            ui.add_space(30.0);

            ui.label(egui::RichText::new(&local.meaning).strong().size(16.0));
            ui.add_space(5.0);
            
            egui::Frame::NONE
                .fill(ui.visuals().window_fill)
                .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                .corner_radius(input_rounding)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut ctx.translate_state.meaning_buffer)
                            .hint_text(&local.hint_meaning_input)
                            .desired_width(f32::INFINITY)
                            .font(egui::FontId::proportional(18.0))
                            .frame(false)
                    );
                });

            ui.add_space(30.0);

            ui.horizontal(|ui| {
                ui.heading(&local.examples);
                ui.label(egui::RichText::new(format!("({})", current_kanji.example.len())).weak());
            });
            ui.add_space(10.0);
            
            for (idx, original_ex) in current_kanji.example.iter().enumerate() {
                egui::Frame::NONE
                    .fill(panel_bg)
                    .stroke(border_stroke)
                    .corner_radius(card_rounding)
                    .inner_margin(15.0)
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(&local.original).size(14.0));
                            ui.add_space(5.0);
                            ui.label(egui::RichText::new(original_ex).size(16.0).strong());
                        });
                        
                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);

                        if idx < ctx.translate_state.examples_buffer.len() {
                             ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(&local.translation).size(14.0));
                                ui.add_space(5.0);
                                
                                ui.add(
                                    egui::TextEdit::multiline(&mut ctx.translate_state.examples_buffer[idx])
                                        .hint_text(&local.hint_examples_input)
                                        .desired_width(f32::INFINITY)
                                        .desired_rows(2)
                                );
                             });
                        } else {
                            ui.label(egui::RichText::new(&local.error_buffer_mismatch).color(egui::Color32::RED));
                        }
                    });
                
                ui.add_space(15.0);
            }

            ui.add_space(20.0);

            ui.columns(2, |columns| {
                columns[0].vertical_centered_justified(|ui| {
                    let btn = egui::Button::new(egui::RichText::new(format!("⬅ {}", &local.previous_button)).size(16.0))
                        .min_size(egui::vec2(0.0, 45.0));

                    if ui.add_enabled(ctx.translate_state.current_index > 0, btn).clicked() {
                        let entry = KanjiTranslation {
                            meaning: ctx.translate_state.meaning_buffer.clone(),
                            translate_examples: ctx.translate_state.examples_buffer.clone(),
                        };
                        ctx.translate_state.data.entries.insert(current_kanji.kanji.clone(), entry);

                        ctx.translate_state.current_index -= 1;
                        if let Some(k) = ctx.kanji.get(ctx.translate_state.current_index) {
                            ctx.translate_state.sync_translation_buffers(k);
                        }
                        ctx.translate_state.status_message.clear();
                    }
                });

                columns[1].vertical_centered_justified(|ui| {
                    let btn = egui::Button::new(egui::RichText::new(format!("{} ➡", &local.next_button)).strong().size(16.0))
                        .min_size(egui::vec2(0.0, 45.0))
                        .fill(ui.visuals().widgets.active.bg_fill); 

                    if ui.add(btn).clicked() {
                        let entry = KanjiTranslation {
                            meaning: ctx.translate_state.meaning_buffer.clone(),
                            translate_examples: ctx.translate_state.examples_buffer.clone(),
                        };
                        ctx.translate_state.data.entries.insert(current_kanji.kanji.clone(), entry);
                        ctx.translate_state.data.last_id = current_kanji.id;

                        ctx.translate_state.current_index += 1;
                        if let Some(next_k) = ctx.kanji.get(ctx.translate_state.current_index) {
                            ctx.translate_state.sync_translation_buffers(next_k);
                        }
                        ctx.translate_state.status_message.clear();
                    }
                });
            });
            
            ui.add_space(30.0);
        });

        None
    }
    
    // Render Card Setup
    fn render_card_setup(ui: &mut egui::Ui, state: &mut CardsSetupState, ctx: &mut AppContext) -> Option<(TabType, bool)> {
        let local = &ctx.localization.local.top_bar.cards;
        let panel_rounding = egui::CornerRadius::same(16);
        let item_rounding = egui::CornerRadius::same(12);
        let panel_bg = ui.visuals().faint_bg_color;
        let border_stroke = ui.visuals().widgets.noninteractive.bg_stroke;

        let mut next_tab: Option<TabType> = None;

        ui.add_space(10.0);
        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new(&local.card_setup_title).size(24.0).strong());
        });
        ui.add_space(20.0);

        ui.columns(2, |cols| {
            cols[0].vertical(|ui| {
                egui::Frame::NONE
                    .fill(panel_bg)
                    .corner_radius(panel_rounding)
                    .stroke(border_stroke)
                    .inner_margin(15.0)
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        
                        ui.vertical_centered(|ui| {
                            ui.label(egui::RichText::new(&local.jlpt_level).strong().size(18.0));
                        });
                        ui.add_space(15.0);

                        egui::Frame::NONE
                            .fill(ui.visuals().window_fill)
                            .corner_radius(12)
                            .inner_margin(egui::Margin::symmetric(10, 8))
                            .stroke(border_stroke)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(format!("{}:", &local.card_count));
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.add(egui::DragValue::new(&mut state.card_limit).range(5..=1000).speed(1.0));
                                    });
                                });
                            });
                        
                        ui.add_space(20.0);
                        ui.separator();
                        ui.add_space(20.0);

                        let levels = vec!["N5", "N4", "N3", "N2", "N1"];
                        
                        for level in levels {
                            let btn_text = format!("{} {}", &local.card_setup_start, level);
                            
                            let btn = egui::Button::new(egui::RichText::new(&btn_text).size(16.0))
                                .min_size(egui::vec2(ui.available_width(), 45.0))
                                .corner_radius(item_rounding);

                            if ui.add(btn).clicked() {
                                let deck_name = format!("JLPT {}", level);
                                let filtered: Vec<Kanji> = ctx.kanji.iter()
                                    .filter(|k| k.jlpt == level)
                                    .cloned()
                                    .collect();
                                
                                if !filtered.is_empty() {
                                    let session = CardsSession::new(filtered, state.card_limit);
                                    next_tab = Some(TabType::CardsActive(session, deck_name));
                                }
                            }
                            ui.add_space(10.0);
                        }
                    });
            });

            cols[1].vertical(|ui| {
                egui::Frame::NONE
                    .fill(panel_bg)
                    .corner_radius(panel_rounding)
                    .stroke(border_stroke)
                    .inner_margin(15.0)
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());

                        ui.vertical_centered(|ui| {
                            ui.label(egui::RichText::new(&local.custom_deck_title).strong().size(18.0));
                        });
                        ui.add_space(15.0);

                        let create_btn = egui::Button::new(
                            egui::RichText::new(format!("➕ {}", &local.manage_decks)).size(16.0)
                        )
                        .min_size(egui::vec2(ui.available_width(), 40.0))
                        .corner_radius(item_rounding);

                        if ui.add(create_btn).clicked() {
                            let mut builder = DeckBuilderState::default();
                            builder.is_editing = false;
                            next_tab = Some(TabType::DeckManager(builder));
                        }

                        ui.add_space(15.0);
                        ui.separator();
                        ui.add_space(15.0);
                        
                        ui.label(egui::RichText::new(format!("{}:", &local.my_decks)).strong());
                        ui.add_space(5.0);

                        let deck_names: Vec<String> = ctx.config.custom_decks.keys().cloned().collect();

                        if deck_names.is_empty() {
                            ui.centered_and_justified(|ui| {
                                ui.label(egui::RichText::new(&local.no_custom_decks_created).weak().italics());
                            });
                        } else {
                            egui::ScrollArea::vertical()
                                .max_height(400.0)
                                .show(ui, |ui| {
                                    let mut deck_to_delete = None;

                                    for name in deck_names {
                                        egui::Frame::NONE
                                            .fill(ui.visuals().window_fill)
                                            .stroke(border_stroke)
                                            .corner_radius(item_rounding)
                                            .inner_margin(10.0)
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    ui.label(egui::RichText::new(&name).strong().size(15.0));

                                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                        let play_btn = egui::Button::new(format!("▶ {}", &local.play_deck))
                                                            .fill(ui.visuals().widgets.active.bg_fill)
                                                            .corner_radius(8);
                                                        
                                                        if ui.add(play_btn).clicked() {
                                                            if let Some(ids) = ctx.config.custom_decks.get(&name) {
                                                                let deck_kanji: Vec<Kanji> = ctx.kanji.iter()
                                                                    .filter(|k| ids.contains(&k.id))
                                                                    .cloned()
                                                                    .collect();
                                                                if !deck_kanji.is_empty() {
                                                                    let session = CardsSession::new(deck_kanji, 0);
                                                                    next_tab = Some(TabType::CardsActive(session, name.clone()));
                                                                }
                                                            }
                                                        }

                                                        ui.add_space(5.0);

                                                        if ui.button("✏").on_hover_text(&local.edit_deck).clicked() {
                                                            if let Some(ids) = ctx.config.custom_decks.get(&name) {
                                                                let mut builder = DeckBuilderState::default();
                                                                builder.deck_name_buffer = name.clone();
                                                                builder.selected_kanji_idx = ids.clone();
                                                                builder.is_editing = true;
                                                                next_tab = Some(TabType::DeckManager(builder));
                                                            }
                                                        }

                                                        if ui.button(egui::RichText::new("🗑").color(egui::Color32::RED))
                                                            .on_hover_text(&local.delete_deck)
                                                            .clicked() 
                                                        {
                                                            deck_to_delete = Some(name.clone());
                                                        }
                                                    });
                                                });
                                            });
                                        ui.add_space(8.0);
                                    }

                                    if let Some(name) = deck_to_delete {
                                        ctx.config.custom_decks.remove(&name);
                                    }
                                });
                        }
                    });
            });
        });

        next_tab.map(|tab| (tab, false))
    }

    // Render Card Session
    fn render_card_session(ui: &mut egui::Ui, session: &mut CardsSession, deck_name: &String, ctx: &AppContext) -> Option<(TabType, bool)> {
        let local = ctx.localization.local.top_bar.cards.clone();
        let local_common = &ctx.localization.local.screens.current_kanji;

        if session.finished {
            ui.centered_and_justified(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("🏆").size(64.0));
                    ui.add_space(10.0);
                    ui.heading(egui::RichText::new(&local.session_complete).size(24.0).strong());
                    ui.add_space(20.0);
                    
                    let btn = egui::Button::new(egui::RichText::new(&local.return_to_menu).size(18.0))
                        .min_size(egui::vec2(200.0, 50.0))
                        .fill(ui.visuals().widgets.active.bg_fill)
                        .corner_radius(12);

                    if ui.add(btn).clicked() {
                        return Some((TabType::CardsSetup(CardsSetupState::default()), false));
                    }

                    return None;
                });
            });
            return None;
        }

        let current_idx = session.current_index;
        let total = session.total_count;
        let current_kanji = match session.queue.get(current_idx) {
            Some(k) => k.clone(),
            None => return None,
        };

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading(egui::RichText::new(deck_name).strong());
                ui.label(egui::RichText::new(format!("{} {} / {}", &local.card, current_idx + 1, total)).weak());
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(egui::RichText::new(format!("❌ {}", &local.exit_training))).clicked() {
                    return Some((TabType::CardsSetup(CardsSetupState::default()), false));
                }
                None
            });
        });

        ui.add_space(10.0);

        let progress = if total > 0 {
            (current_idx as f32) / (total as f32)
        } else {
            0.0
        };

        ui.add(egui::ProgressBar::new(progress).show_percentage().animate(false));

        ui.add_space(20.0);

        let card_w = 320.0;
        let card_h = 500.0;
        
        ui.vertical_centered(|ui| {
            let (rect, response) = ui.allocate_exact_size(egui::vec2(card_w, card_h), egui::Sense::click());
            let is_hovered = response.hovered();
            let bg_color = ui.visuals().window_fill;
            let stroke_color = if is_hovered {
                ui.visuals().widgets.active.bg_stroke.color
            } else {
                ui.visuals().widgets.noninteractive.bg_stroke.color
            };
        
            ui.painter().rect(
                rect,
                16.0,
                bg_color,
                egui::Stroke::new(if is_hovered { 1.5 } else { 1.0 }, stroke_color),
                egui::StrokeKind::Outside,
            );

            if response.clicked() {
                session.is_card_flipped = !session.is_card_flipped;
            }

            ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    ui.label(
                        egui::RichText::new(&current_kanji.kanji)
                            .size(90.0)
                            .strong()
                            .color(ui.visuals().strong_text_color())
                    );

                    ui.add_space(20.0);

                    ui.scope(|ui| {
                        ui.set_min_height(340.0);

                        if session.is_card_flipped {
                            ui.separator();
                            ui.add_space(15.0);

                            let meaning_text = if let Some(entry) = ctx.translate_state.data.entries.get(&current_kanji.kanji) {
                                &entry.meaning
                            } else {
                                ""
                            };

                            if !meaning_text.is_empty() {
                                ui.label(egui::RichText::new(meaning_text).size(20.0).strong().color(ui.visuals().text_color()));
                                ui.add_space(5.0);
                                ui.label(egui::RichText::new(&local_common.meaning).size(12.0).weak());
                                ui.add_space(15.0);
                                ui.separator();
                                ui.add_space(15.0);
                            }

                            ui.columns(2, |cols| {
                                cols[0].vertical_centered(|ui| {
                                    ui.label(egui::RichText::new(&local.onyomi).size(12.0).weak());
                                    ui.add_space(2.0);
                                    ui.label(egui::RichText::new(&current_kanji.onyomi).strong().size(16.0));
                                    ui.label(egui::RichText::new(&current_kanji.onyomi_romaji).italics().size(14.0));
                                });

                                cols[1].vertical_centered(|ui| {
                                    ui.label(egui::RichText::new(&local.kunyomi).size(12.0).weak());
                                    ui.add_space(2.0);
                                    ui.label(egui::RichText::new(&current_kanji.kunyomi).strong().size(16.0));
                                    ui.label(egui::RichText::new(&current_kanji.kunyomi_romaji).italics().size(14.0));
                                });
                            });
                        

                        } else {
                            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                            ui.add_space(20.0);
                            ui.label(egui::RichText::new(&local.click_to_flip).weak().size(12.0));
                        });
                    }

                    });
                });
            });

            ui.add_space(25.0);

            let next_btn = egui::Button::new(
                egui::RichText::new(format!("{} ➡", &local.next_card))
                    .size(18.0)
                    .strong()
            )
            .min_size(egui::vec2(200.0, 45.0))
            .fill(ui.visuals().widgets.open.bg_fill) 
            .corner_radius(20);

            if ui.add(next_btn).clicked() {
                session.next();
            }
            });

        None
    }

    // Render Deck Manager
    fn render_deck_manager(ui: &mut egui::Ui, state: &mut DeckBuilderState, ctx: &mut AppContext) -> Option<(TabType, bool)> {
        let local = ctx.localization.local.top_bar.cards.clone();
        
        let panel_rounding = egui::CornerRadius::same(12);
        let item_rounding = egui::CornerRadius::same(8);
        let panel_bg = ui.visuals().faint_bg_color;
        let border_stroke = ui.visuals().widgets.noninteractive.bg_stroke;

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            if ui.add(egui::Button::new(egui::RichText::new(format!("⬅ {}", &local.back)).size(16.0)).frame(false)).clicked() {
                return Some((TabType::CardsSetup(CardsSetupState::default()), false));
            }

            let title = if state.is_editing {
                &local.edit_deck
            } else {
                &local.deck_manager
            };
            ui.heading(egui::RichText::new(title).strong().size(20.0));

            None
        });
        ui.add_space(15.0);

        egui::Frame::NONE
            .fill(ui.visuals().window_fill)
            .stroke(border_stroke)
            .corner_radius(panel_rounding)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(format!("{}:", &local.deck_name)).strong());
                    
                    let pill_bg = ui.visuals().widgets.inactive.bg_fill;
                    egui::Frame::NONE
                        .fill(pill_bg)
                        .corner_radius(16)
                        .stroke(border_stroke)
                        .inner_margin(egui::Margin::symmetric(10, 5))
                        .show(ui, |ui| {
                            ui.set_width(300.0);
                            ui.add_enabled(
                                !state.is_editing,
                                egui::TextEdit::singleline(&mut state.deck_name_buffer)
                                    .frame(false)
                                    .hint_text(&ctx.localization.local.top_bar.cards.my_new_decks)
                            );
                        });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let btn_text = if state.is_editing {
                            &local.update_deck
                        } else {
                            &local.save_deck
                        };
                        
                        let save_btn = egui::Button::new(egui::RichText::new(btn_text).strong())
                            .min_size(egui::vec2(100.0, 35.0))
                            .fill(ui.visuals().widgets.active.bg_fill)
                            .corner_radius(8);

                        if ui.add(save_btn).clicked() {
                            if !state.deck_name_buffer.is_empty()
                                && !state.selected_kanji_idx.is_empty()
                            {
                                ctx.config.custom_decks.insert(
                                    state.deck_name_buffer.clone(),
                                    state.selected_kanji_idx.clone(),
                                );
                                *state = DeckBuilderState::default();
                            }
                        }
                    });
                });
            });

        ui.add_space(15.0);

        ui.columns(2, |cols| {
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new(&local.avaliable_kanji).strong().size(16.0));
                ui.add_space(5.0);
                
                let pill_bg = ui.visuals().widgets.inactive.bg_fill;
                egui::Frame::NONE
                    .fill(pill_bg)
                    .corner_radius(16)
                    .stroke(border_stroke)
                    .inner_margin(egui::Margin::symmetric(10, 6))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal(|ui| {
                            ui.label("🔍");
                            ui.add(
                                egui::TextEdit::singleline(&mut state.search_buffer)
                                    .frame(false)
                                    .hint_text(&local.search_kanji_hint)
                            );
                        });
                    });

                ui.add_space(10.0);

                egui::Frame::NONE
                    .fill(panel_bg)
                    .stroke(border_stroke)
                    .corner_radius(panel_rounding)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        let search = state.search_buffer.to_lowercase();
                        
                        egui::ScrollArea::vertical()
                            .id_salt("source_list")
                            .max_height(450.0)
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                
                                let filtered: Vec<&Kanji> = ctx.kanji.iter()
                                    .filter(|k| {
                                        if search.is_empty() { return false; }
                                        k.kanji.contains(&search)
                                            || k.onyomi_romaji.contains(&search)
                                            || k.kunyomi_romaji.contains(&search)
                                    })
                                    .take(50)
                                    .collect();

                                if search.is_empty() {
                                    ui.centered_and_justified(|ui| {
                                        ui.label(egui::RichText::new(&local.type_search_hint).weak());
                                    });
                                } else if filtered.is_empty() {
                                     ui.centered_and_justified(|ui| {
                                        ui.label(egui::RichText::new(&ctx.localization.local.top_bar.cards.no_matches).weak());
                                    });
                                }

                                for k in filtered {
                                    let is_added = state.selected_kanji_idx.contains(&k.id);
                                    
                                    ui.group(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new(&k.kanji).size(20.0).strong());
                                            ui.vertical(|ui| {
                                                ui.label(egui::RichText::new(&k.onyomi_romaji).size(10.0).weak());
                                                ui.label(egui::RichText::new(&k.kunyomi_romaji).size(10.0).weak());
                                            });

                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                if is_added {
                                                    ui.label(egui::RichText::new(format!("✔ {}", &ctx.localization.local.top_bar.cards.added)).color(egui::Color32::GREEN).size(12.0));
                                                } else {
                                                    let btn = egui::Button::new(format!("{} ➡", &local.add_kanji))
                                                        .corner_radius(item_rounding);
                                                    if ui.add(btn).clicked() {
                                                        state.selected_kanji_idx.push(k.id);
                                                    }
                                                }
                                            });
                                        });
                                    });
                                    ui.add_space(4.0);
                                }
                            });
                    });
            });

            cols[1].vertical(|ui| {
                ui.label(
                    egui::RichText::new(format!("{} ({})", &local.deck_content, state.selected_kanji_idx.len()))
                        .strong()
                        .size(16.0)
                );
                ui.add_space(5.0);

                egui::Frame::NONE
                    .fill(panel_bg)
                    .stroke(border_stroke)
                    .corner_radius(panel_rounding)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        
                        egui::ScrollArea::vertical()
                            .id_salt("deck_list")
                            .max_height(500.0) 
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                
                                if state.selected_kanji_idx.is_empty() {
                                    ui.centered_and_justified(|ui| {
                                        ui.label(egui::RichText::new(&ctx.localization.local.top_bar.cards.decks_is_empty).weak());
                                    });
                                }

                                let mut ids_to_remove = Vec::new();

                                for (idx, id) in state.selected_kanji_idx.iter().enumerate() {
                                    if let Some(k) = ctx.kanji.iter().find(|k| k.id == *id) {
                                        ui.group(|ui| {
                                            ui.horizontal(|ui| {
                                                // Delete Button
                                                let btn = egui::Button::new(egui::RichText::new("❌").color(egui::Color32::RED))
                                                    .frame(false);
                                                if ui.add(btn).clicked() {
                                                    ids_to_remove.push(idx);
                                                }

                                                ui.label(egui::RichText::new(&k.kanji).size(20.0).strong());
                                                
                                                // Extra info
                                                ui.label(egui::RichText::new(format!("[{}]", k.id)).size(10.0).weak());
                                                ui.label(egui::RichText::new(&k.onyomi).size(12.0));
                                            });
                                        });
                                        ui.add_space(4.0);
                                    }
                                }

                                for idx in ids_to_remove.iter().rev() {
                                    state.selected_kanji_idx.remove(*idx);
                                }
                            });
                    });
            });
        });

        None
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
            self.tab_manager.ui(ui, ctx);
            ui.separator();

            let mut action: Option<(TabType, bool)> = None;

            let active_index = self.tab_manager.active_tab_index;

            if let Some(content) = self.tab_manager.active_content_mut() {
                let mut context = AppContext {
                    kanji: &self.kanji,
                    config: &mut self.config,
                    paths: &self.paths,
                    settings: &self.settings,
                    localization: &self.localization,
                    translate_state: &mut self.translate_state,
                };

                match content {
                    TabType::Home(search_query) => {
                        action = Self::render_home(ui, search_query, &context);
                    },
                    TabType::KanjiList(state) => {
                        action = Self::render_kanji_list(ui, state, &context);
                    },
                    TabType::KanjiDetail(state) => {
                        action = Self::render_kanji_detail(ui, state, &context);
                    },
                    TabType::Kana(is_katakana) => {
                        action = Self::render_kana(ui, is_katakana, &context);
                    },
                    TabType::RomajiToKana(state) => {
                        action = Self::render_romaji_to_kana(ui, state, &context);
                    },
                    TabType::Translate(state) => {
                        action = Self::render_translate_kanji(ui, state, &mut context);
                    },
                    TabType::CardsSetup(state) => {
                        action = Self::render_card_setup(ui, state, &mut context);
                    },
                    TabType::CardsActive(session, deck_name) => {
                        action = Self::render_card_session(ui, session, deck_name, &context);
                    },
                    TabType::DeckManager(state) => {
                        action = Self::render_deck_manager(ui, state, &mut context);
                    },
                }
            }

            if let Some((new_content, open_in_new)) = action {
                if open_in_new {
                    self.tab_manager.add_tab(new_content, true);
                } else {
                    if let Some(tab) = self.tab_manager.tabs.get_mut(active_index) {
                        tab.content = new_content;
                    }
                }
            }
        });

        if self
            .settings
            .setting(&mut self.settings_window, &mut self.paths, ctx)
        {
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
