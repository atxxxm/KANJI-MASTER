use crate::back::localization::Localization;
use rfd::FileDialog;

pub struct Settings {
    localization: Localization, // Localization Settings
    pub interface_font_size: f32, // Interface Font Size
    pub kanji_font_size: f32, // Kanji Font Size
    pub auto_save_progress: bool, // Auto Save Progress
    pub open_last_session_at_startup: bool, // Open Last Session At Startup
    pub confrim_card_delete: bool, // Confrim Card Delete
    pub confrim_progress_reset: bool, // Confrim Progress Reset
    pub animation_speed: f32, // Animation Speed
}


impl Settings {
    pub fn new(localization: Localization) -> Self {
        Self {
            localization,
            interface_font_size: 14.0,
            kanji_font_size: 48.0,
            auto_save_progress: false,
            open_last_session_at_startup: false,
            confrim_card_delete: false,
            confrim_progress_reset: false,
            animation_speed: 0.75,
        }
    }

    pub fn setting(&mut self, is_open: &mut bool, ctx: &egui::Context) {
        if !*is_open {
            return;
        }

        let local = &self.localization.local.settings;
        let mut open_file_localization = false;
        let mut open_file_kanji_localization = false;

        egui::Window::new(&local.title)
            .open(is_open)
            .resizable(true)
            .show(ctx, |ui| {
                ui.label(&local.lang);
                if ui.button(&local.lang_button).clicked() {
                    open_file_localization = true;
                }

                ui.separator();

                ui.label(&local.kanji_localization);
                if ui.button(&local.lang_button).clicked() {
                    open_file_kanji_localization = true;
                }

                ui.separator();

                ui.label(&local.interface_font_size);
                ui.add(egui::Slider::new(
                    &mut self.interface_font_size,
                    8.0..=32.0,
                ));

                ui.separator();

                ui.label(&local.kanji_font_size);
                ui.add(egui::Slider::new(&mut self.kanji_font_size, 10.0..=90.0));

                ui.separator();

                ui.label(&local.kanji_animation_speed);
                ui.add(egui::Slider::new(
                    &mut self.animation_speed,
                    0.1..=3.0,
                ));

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

        if open_file_localization {
            self.open_localization_file();
        }

        if open_file_kanji_localization {
            self.open_kanji_localization_file();
        }
    }

    // Open Localization File
    fn open_localization_file(&self) {
        let file_path = FileDialog::new()
            .add_filter("Localization File (*json)", &["json"])
            .pick_file();

        match file_path {
            Some(path) => println!("Selected file: {:?}", path),
            None => println!("No file selected"),
        }
    }

    // Open Kanji Localization File
    fn open_kanji_localization_file(&self) {
        let file_path = FileDialog::new()
            .add_filter("Kanji Localization File (*json)", &["json"])
            .pick_file();

        match file_path {
            Some(path) => println!("Selected file: {:?}", path),
            None => println!("No file selected"),
        }
    }

}

