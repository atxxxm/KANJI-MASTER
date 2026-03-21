use crate::back::config::{Config, save_config, get_config_path};
use crate::back::core::{Database, Kanji};
use crate::back::localization::*;
use crate::back::translation::*;
use crate::back::recognition::RecognitionSystem;
use crate::ui::settings::Settings;
use crate::ui::tabs::{KanjiListState, RomajiKanaState, TabManager,
    TabType, TranslateTabState, CardsSetupState, DrawSearchState};
use crate::ui::context::{AppContext, TabOpenMode};
use crate::ui::views;
use eframe::egui;
use std::path::PathBuf;
use std::sync::OnceLock;

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

    recognition: RecognitionSystem,

    error_notification: Option<String>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>, config: Config) -> anyhow::Result<Self> {
        // Set Font
        set_font(&cc.egui_ctx);

        let config_path = get_config_path()
            .ok_or_else(|| anyhow::anyhow!("Cannot get config path"))?;

        // Load Base Localization
        let localization: Localization = load(&config.path_to_localization)
            .map_err(|e| anyhow::anyhow!("Failed to load localization: {}", e))?;
        // Load Kanji

        let kanji = Database::new(&config.path_to_db_core)
            .get_kanji()
            .map_err(|e| anyhow::anyhow!("Failed to read database: {}", e))?;

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

        // Recognition initialization
        let mut recognition = RecognitionSystem::new();
        recognition.load_cache(&kanji, &config.path_to_svg_images);

        Ok(Self {
            tab_manager: TabManager::new(),
            kanji,
            settings: Settings::new(localization.clone(), &config),
            config,
            paths,
            localization,
            settings_window,
            translate_state,
            config_path,
            recognition,
            error_notification: None,
        })
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

        if let Err(e) = save_config(&self.config_path, &current_config) {
            self.error_notification = Some(format!("Failed to save config: {}", e));
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

                            if ui.button(&local.tools.draw_and_search.title).clicked() {
                                self.tab_manager.add_tab(TabType::DrawSearch(DrawSearchState::default()), true);
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
            self.tab_manager.ui(ui, ctx, &self.localization);
            ui.separator();

            let mut action: Option<(TabType, TabOpenMode)> = None;

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
                        action = views::home::render(ui, search_query, &context);
                    },
                    TabType::KanjiList(state) => {
                        action = views::kanji_list::render(ui, state, &context);
                    },
                    TabType::KanjiDetail(state) => {
                        action = views::kanji_detail::render(ui, state, &context);
                    },
                    TabType::Kana(is_katakana) => {
                        action = views::kana::render(ui, is_katakana, &context);
                    },
                    TabType::RomajiToKana(state) => {
                        action = views::romaji_kana::render(ui, state, &context);
                    },
                    TabType::Translate(state) => {
                        action = views::translate::render(ui, state, &mut context);
                    },
                    TabType::CardsSetup(state) => {
                        action = views::cards::render_setup(ui, state, &mut context);
                    },
                    TabType::CardsActive(session, deck_name) => {
                        action = views::cards::render_session(ui, session, deck_name, &context);
                    },
                    TabType::DeckManager(state) => {
                        action = views::deck_builder::render(ui, state, &mut context);
                    },
                    TabType::DrawSearch(state) => {
                        action = views::draw_search::render(ui, state, &context, &self.recognition);
                    },
                }
            }

            if let Some((new_content, mode)) = action {
                match mode {
                    TabOpenMode::NewTabActive => {
                        self.tab_manager.add_tab(new_content, true); 
                    }
                    TabOpenMode::NewTabBackground => {
                        self.tab_manager.add_tab(new_content, false);
                    }
                    TabOpenMode::ReplaceCurrent => {
                        if let Some(tab) = self.tab_manager.tabs.get_mut(active_index) {
                            tab.content = new_content;
                        }
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

        if let Some(err_msg) = self.error_notification.take() {
        let mut is_open = true;
        let mut dismiss_clicked = false;

        egui::Window::new("Error")
            .open(&mut is_open) 
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-10.0, -10.0))
            .show(ctx, |ui| {
                ui.label(egui::RichText::new(&err_msg).color(ui.visuals().error_fg_color));
                if ui.button("Dismiss").clicked() {
                    dismiss_clicked = true; 
                }
            });

        if is_open && !dismiss_clicked {
            self.error_notification = Some(err_msg);
        }
    }
    }

    // On Exit and Save Config
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save();
    }
}


pub struct AppWrapper {
    state: Result<App, String>,
}

impl AppWrapper {
    pub fn new(cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        match App::new(cc, config) {
            Ok(app) => Self { state: Ok(app) },
            Err(e) => Self { state: Err(e.to_string()) },
        }
    }
}

impl eframe::App for AppWrapper {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match &mut self.state {
            Ok(app) => {
                app.update(ctx, frame);
            }
            Err(err_msg) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(egui::RichText::new("⚠️").size(60.0));
                            ui.add_space(20.0);
                            ui.heading(egui::RichText::new("Application Error").color(egui::Color32::RED));
                            ui.add_space(10.0);
                            ui.label(err_msg.clone());
                            ui.add_space(30.0);
                            if ui.button("Quit App").clicked() {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        });
                    });
                });
            }
        }
    }

    fn on_exit(&mut self, gl: Option<&eframe::glow::Context>) {
        if let Ok(app) = &mut self.state {
            app.on_exit(gl);
        }
    }
}

// Run App
pub fn run(config: Config) -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((800.0, 600.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        "Kanji Master",
        native_options,
        Box::new(|cc| Ok(Box::new(AppWrapper::new(cc, config)))), 
    )
}

// Set Font
fn set_font(ctx: &egui::Context) {
    static FONT_LOADED: OnceLock<()> = OnceLock::new();

    FONT_LOADED.get_or_init(|| {
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
    });
}
