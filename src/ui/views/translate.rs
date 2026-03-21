use eframe::egui;
use crate::ui::context::{AppContext, TabOpenMode};
use crate::ui::tabs::TabType;
use crate::back::core::Kanji;
use crate::back::translation::KanjiTranslation;
use crate::ui::tabs::TranslateTabState;
use std::sync::Arc;

pub fn render(ui: &mut egui::Ui, state: &mut TranslateTabState, ctx: &mut AppContext) -> Option<(TabType, TabOpenMode)>{
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

                        let matches: Vec<(usize, &Arc<Kanji>)> = ctx.kanji.iter().enumerate()
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

        let current_kanji = Arc::clone(&ctx.kanji[ctx.translate_state.current_index]);

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