use eframe::egui;
use crate::ui::context::{AppContext, TabOpenMode};
use crate::ui::tabs::TabType;
use crate::back::romaji_kana::to_kana;
use crate::ui::tabs::RomajiKanaState;

pub fn render(ui: &mut egui::Ui, state: &mut RomajiKanaState, ctx: &AppContext) -> Option<(TabType, TabOpenMode)> {
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
        let panel_stroke = ui.visuals().widgets.noninteractive.bg_stroke;

        ui.horizontal(|ui| {
            let width = 600.0; 
            ui.add_space((ui.available_width() - width) / 2.0);
            
            ui.vertical(|ui| {
                ui.set_width(width); 
                ui.add_space(5.0);

                egui::Frame::NONE
                    .fill(ui.visuals().window_fill)
                    .stroke(panel_stroke)
                    .corner_radius(panel_rounding)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        let output = egui::TextEdit::multiline(&mut state.input)
                            .hint_text(&local.hint_input)
                            .desired_width(width)
                            .min_size(egui::vec2(0.0, 400.0))
                            .font(egui::FontId::proportional(22.0))
                            .frame(false)
                            .show(ui);

                        if output.response.changed() {
                            let converted = to_kana(&state.input, state.is_katakana, true);
                            state.input = converted;
                        }
                    });

                ui.add_space(10.0);

                ui.vertical_centered(|ui| {
                    let btn = egui::Button::new(
                        egui::RichText::new(format!("📋 {}", &local.copy_button)).size(16.0),
                    )
                    .min_size(egui::vec2(250.0, 45.0));

                    if ui.add(btn).clicked() {
                        let final_text = to_kana(&state.input, state.is_katakana, false);
                        ui.ctx().copy_text(final_text);
                    }
                });
            });
        });

        None
    }