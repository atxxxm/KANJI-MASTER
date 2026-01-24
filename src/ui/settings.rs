use crate::back::config::Config;
use crate::back::localization::{Localization, Paths, save};
use rfd::FileDialog;
use eframe::egui;

pub struct Settings {
    localization: Localization,   // Localization Settings
    pub interface_font_size: f32, // Interface Font Size
    pub kanji_font_size: f32,     // Kanji Font Size
    pub animation_speed: f32,     // Animation Speed
    pub show_kanji_meaning: bool, // Show Kanji Meaning
    pub focus_on_search: bool,    // Focus on search when opening the app
}

impl Settings {
    pub fn new(localization: Localization, config: &Config) -> Self {
        Self {
            localization,
            interface_font_size: config.interface_font_size,
            kanji_font_size: config.kanji_font_size,
            animation_speed: config.animation_speed,
            show_kanji_meaning: config.show_kanji_meaning,
            focus_on_search: config.focus_on_search,
        }
    }

    pub fn setting(&mut self, is_open: &mut bool, paths: &mut Paths, ctx: &egui::Context) -> bool {
        let mut path_changed = false;

        if !*is_open {
            return false;
        }

        let local = &self.localization.local.settings;
        let mut open_file_localization = false;
        let mut open_file_kanji_localization = false;
        let mut create_default_localization_file = false;

        let panel_rounding = egui::CornerRadius::same(12);
        let border_stroke = ctx.style().visuals.widgets.noninteractive.bg_stroke;
        let panel_bg = ctx.style().visuals.faint_bg_color;

        egui::Window::new(egui::RichText::new(&local.title).strong())
            .open(is_open)
            .resizable(true)
            .vscroll(true)
            .default_width(450.0)
            .show(ctx, |ui| {
                ui.add_space(5.0);

                ui.label(egui::RichText::new(format!("📂 {}", &local.files_and_data)).strong());
                ui.add_space(5.0);

                egui::Frame::NONE
                    .fill(panel_bg)
                    .stroke(border_stroke)
                    .corner_radius(panel_rounding)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new(&local.lang).size(12.0).weak());
                        ui.horizontal(|ui| {
                            let path_bg = ui.visuals().widgets.inactive.bg_fill;
                            egui::Frame::NONE
                                .fill(path_bg)
                                .corner_radius(8)
                                .inner_margin(egui::Margin::symmetric(8, 4))
                                .show(ui, |ui| {
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(&paths.path_to_localization).monospace().size(11.0)
                                        ).truncate()
                                    );
                                });
                            
                            if ui.button("📂").on_hover_text(&local.lang_button).clicked() {
                                open_file_localization = true;
                            }
                        });

                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);

                        ui.label(egui::RichText::new(&local.kanji_localization).size(12.0).weak());
                        ui.horizontal(|ui| {
                            let path_bg = ui.visuals().widgets.inactive.bg_fill;
                            egui::Frame::NONE
                                .fill(path_bg)
                                .corner_radius(8)
                                .inner_margin(egui::Margin::symmetric(8, 4))
                                .show(ui, |ui| {
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(&paths.path_to_kanji_localization).monospace().size(11.0)
                                        ).truncate()
                                    );
                                });

                            if ui.button("📂").on_hover_text(&local.lang_button).clicked() {
                                open_file_kanji_localization = true;
                            }
                        });
                    });

                ui.add_space(20.0);

                ui.label(egui::RichText::new(format!("🎨 {}", &local.appearance)).strong());
                ui.add_space(5.0);

                egui::Frame::NONE
                    .fill(panel_bg)
                    .stroke(border_stroke)
                    .corner_radius(panel_rounding)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        egui::Grid::new("settings_grid")
                            .num_columns(2)
                            .spacing([20.0, 15.0])
                            .show(ui, |ui| {
                                ui.label(&local.interface_font_size);
                                ui.add(egui::Slider::new(&mut self.interface_font_size, 12.0..=32.0).suffix(" px"));
                                ui.end_row();

                                ui.label(&local.kanji_font_size);
                                ui.add(egui::Slider::new(&mut self.kanji_font_size, 20.0..=120.0).suffix(" px"));
                                ui.end_row();

                                ui.label(&local.kanji_animation_speed);
                                ui.add(egui::Slider::new(&mut self.animation_speed, 0.1..=5.0).logarithmic(true));
                                ui.end_row();
                            });
                    });

                ui.add_space(20.0);

                ui.label(egui::RichText::new(format!("⚙ {}", &local.behavior)).strong());
                ui.add_space(5.0);

                egui::Frame::NONE
                    .fill(panel_bg)
                    .stroke(border_stroke)
                    .corner_radius(panel_rounding)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.focus_on_search, "");
                            ui.label(&local.focus_on_search);
                        });

                        ui.add_space(5.0);
                        ui.separator();
                        ui.add_space(5.0);

                        // Show Meaning
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.show_kanji_meaning, "");
                            ui.label(&local.show_kanji_meaning);
                        });
                    });

                ui.add_space(20.0);

                ui.label(egui::RichText::new(&local.tools).strong());
                ui.add_space(5.0);
                
                let btn = egui::Button::new(format!("📝 {}", &local.create_default_localization_file))
                    .min_size(egui::vec2(ui.available_width(), 35.0));
                
                if ui.add(btn).clicked() {
                    create_default_localization_file = true;
                }
                
                ui.add_space(10.0);
            });

        if open_file_localization {
            if self.open_localization_file(paths) {
                path_changed = true;
            }
        }

        if open_file_kanji_localization {
            if self.open_kanji_localization_file(paths) {
                path_changed = true;
            }
        }

        if create_default_localization_file {
            self.create_default_localization_file();
        }

        path_changed
    }

    // Open Localization File
    fn open_localization_file(&self, paths: &mut Paths) -> bool {
        let file_path = FileDialog::new()
            .add_filter("Localization File (*json)", &["json"])
            .pick_file();

        if let Some(path) = file_path {
            paths.path_to_localization = path.display().to_string();
            return true;
        }

        false
    }

    // Open Kanji Localization File
    fn open_kanji_localization_file(&self, paths: &mut Paths) -> bool {
        let file_path = FileDialog::new()
            .add_filter("Kanji Localization File (*json)", &["json"])
            .pick_file();

        if let Some(path) = file_path {
            paths.path_to_kanji_localization = path.display().to_string();
            return true;
        }

        false
    }

    // Update Localization
    pub fn update_localization(&mut self, new_local: Localization) {
        self.localization = new_local;
    }

    // Create Default Localization File
    fn create_default_localization_file(&self) {
        // Open save dialog
        let file_path = FileDialog::new()
            .set_file_name("default_localization.json")
            .add_filter("JSON", &["json"])
            .save_file();

        if let Some(path) = file_path {
            if let Some(path_str) = path.to_str() {
                // Create default structure
                let default_loc = Localization::default();

                // Try to save
                match save(path_str, &default_loc) {
                    Ok(_) => {}
                    Err(e) => eprintln!("Error saving localization: {}", e),
                }
            }
        }
    }
}