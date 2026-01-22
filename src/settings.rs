use crate::localization::Localization;


pub struct Settings {
    localization: Localization,
    pub interface_font_size: f32,
    pub kanji_font_size: f32,
    pub auto_save_progress: bool,
    pub open_last_session_at_startup: bool,
    pub confrim_card_delete: bool,
    pub confrim_progress_reset: bool,
}


impl Settings {
    pub fn new(localization: Localization) -> Self {
        Self {
            localization,
            interface_font_size: 16.0,
            kanji_font_size: 48.0,
            auto_save_progress: false,
            open_last_session_at_startup: false,
            confrim_card_delete: false,
            confrim_progress_reset: false,
        }
    }

    pub fn setting(&mut self, is_open: &mut bool, ctx: &egui::Context) {
        if !*is_open {
            return;
        }

        let local = &self.localization.local.settings;

        egui::Window::new(&local.title)
            .open(is_open)
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
}

