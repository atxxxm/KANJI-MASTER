use crate::back::localization::{Localization, Paths, save};
use rfd::FileDialog;
use crate::back::config::Config;

pub struct Settings {
    localization: Localization, // Localization Settings
    pub interface_font_size: f32, // Interface Font Size
    pub kanji_font_size: f32, // Kanji Font Size
    pub animation_speed: f32, // Animation Speed
    pub show_kanji_meaning: bool, // Show Kanji Meaning
    pub focus_on_search: bool, // Focus on search when opening the app
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

        egui::Window::new(&local.title)
            .open(is_open)
            .resizable(true)
            .vscroll(true)
            .show(ctx, |ui| {
                // Localization
                ui.label(&local.lang);
                ui.label(egui::RichText::new(&paths.path_to_localization).size(10.0));
                if ui.button(&local.lang_button).clicked() {
                    open_file_localization = true;
                }

                ui.separator();

                // Kanji Localization
                ui.label(&local.kanji_localization);
                ui.label(egui::RichText::new(&paths.path_to_kanji_localization).size(10.0));
                if ui.button(&local.lang_button).clicked() {
                    open_file_kanji_localization = true;
                }

                ui.separator();

                // Interface Font Size
                ui.label(&local.interface_font_size);
                ui.add(egui::Slider::new(
                    &mut self.interface_font_size,
                    8.0..=32.0,
                ));

                ui.separator();

                // Kanji Font Size
                ui.label(&local.kanji_font_size);
                ui.add(egui::Slider::new(&mut self.kanji_font_size, 10.0..=90.0));

                ui.separator();

                // Animation Speed Kanji
                ui.label(&local.kanji_animation_speed);
                ui.add(egui::Slider::new(
                    &mut self.animation_speed,
                    0.1..=3.0,
                ));

                ui.separator();

                // Focus on search when opening the app
                ui.label(&local.focus_on_search);
                ui.checkbox(&mut self.focus_on_search, "");

                ui.separator();

                // Show Kanji Meaning
                ui.label(&local.show_kanji_meaning);
                ui.checkbox(&mut self.show_kanji_meaning, "");

                ui.separator();

                // Tools
                ui.heading(&local.tools);

                // Create Default Localization File
                if ui.button(&local.create_default_localization_file).clicked() {
                    create_default_localization_file = true;
                }
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
                    Ok(_) => {},
                    Err(e) => eprintln!("Error saving localization: {}", e),
                }
            }
        }
    }

}

