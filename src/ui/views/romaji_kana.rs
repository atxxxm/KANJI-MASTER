use eframe::egui;
use crate::ui::context::{AppContext, TabOpenMode};
use crate::ui::tabs::TabType;
use crate::back::romaji_kana::to_kana;
use crate::ui::tabs::RomajiKanaState;

fn extract_trailing_kana(input: &str) -> Option<(usize, &str)> {
    if input.is_empty() { return None; }

    let mut start_byte = 0;
    for (i, c) in input.char_indices().rev() {
        if c.is_whitespace()
            || c.is_ascii_punctuation()
            || c == '。' || c == '、' || c == '？' || c == '！' || c == '・'
            || (c >= '\u{4E00}' && c <= '\u{9FAF}') 
        {
            start_byte = i + c.len_utf8();
            break;
        }
    }

    if start_byte < input.len() {
        Some((start_byte, &input[start_byte..]))
    } else {
        None
    }
}

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
        let available_w = ui.available_width();
        let assistant_w = 320.0;
        let spacing = 20.0;
        let editor_w = (available_w - assistant_w - spacing).max(300.0);

        let offset = (available_w - (editor_w + assistant_w + spacing)).max(0.0) / 2.0;
        ui.add_space(offset);
        
        ui.vertical(|ui| {
            ui.set_width(editor_w); 
            ui.add_space(5.0);

            egui::Frame::NONE
                .fill(ui.visuals().window_fill)
                .stroke(panel_stroke)
                .corner_radius(panel_rounding)
                .inner_margin(15.0)
                .show(ui, |ui| {
                    let output = egui::TextEdit::multiline(&mut state.input)
                        .hint_text(&local.hint_input)
                        .desired_width(editor_w)
                        .min_size(egui::vec2(0.0, 400.0))
                        .font(egui::FontId::proportional(22.0))
                        .frame(false)
                        .show(ui);

                    if output.response.changed() {
                        let converted = to_kana(&state.input, state.is_katakana, true);
                        state.input = converted;
                    }
                });

            ui.add_space(15.0);

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

        ui.add_space(spacing);

        ui.vertical(|ui| {
            ui.set_width(assistant_w);
            ui.add_space(5.0);

            egui::Frame::NONE
                .fill(ui.visuals().window_fill)
                .stroke(panel_stroke)
                .corner_radius(panel_rounding)
                .inner_margin(15.0)
                .show(ui, |ui| {
                    ui.heading(egui::RichText::new(&local.assistant_title).strong());
                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(10.0);

                    if let Some((start_idx, chunk)) = extract_trailing_kana(&state.input) {
                        let chunk_trim = chunk.trim();

                        if chunk_trim.is_empty() {
                            ui.label(egui::RichText::new(&local.assistant_hint).weak().italics());
                        } else {
                            ui.label(egui::RichText::new(format!("{} {}", &local.assistant_suggestions_for, chunk_trim)).strong());
                            ui.add_space(10.0);

                            let hiragana_chunk: String = chunk_trim.chars().map(|c| {
                                if c as u32 >= 0x30A1 && c as u32 <= 0x30F6 { char::from_u32(c as u32 - 0x0060).unwrap_or(c) } else { c }
                            }).collect();

                            let katakana_chunk: String = chunk_trim.chars().map(|c| {
                                if c as u32 >= 0x3041 && c as u32 <= 0x3096 { char::from_u32(c as u32 + 0x0060).unwrap_or(c) } else { c }
                            }).collect();

                            // Ищем подходящие кандзи
                            let mut matches = Vec::new();
                            for kanji in ctx.kanji.iter() {
                                let clean_kun = kanji.kunyomi.replace(['.', '-'], "");
                                let clean_on = kanji.onyomi.replace(['.', '-'], "");

                                if clean_kun.contains(&hiragana_chunk) || clean_on.contains(&katakana_chunk) {
                                    let mut is_exact = false;
                                    for r in clean_kun.split('、') { if r.trim() == hiragana_chunk { is_exact = true; break; } }
                                    for r in clean_on.split('、') { if r.trim() == katakana_chunk { is_exact = true; break; } }

                                    matches.push((kanji, is_exact));
                                }
                            }

                            matches.sort_by(|(a, a_exact), (b, b_exact)| {
                                match (a_exact, b_exact) {
                                    (true, false) => std::cmp::Ordering::Less,
                                    (false, true) => std::cmp::Ordering::Greater,
                                    _ => b.jlpt.cmp(&a.jlpt) 
                                }
                            });

                            if matches.is_empty() {
                                ui.label(egui::RichText::new(&local.assistant_no_kanji_found).weak());
                            } else {
                                egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                                    for (kanji, _is_exact) in matches.into_iter().take(30) {
                                        let card_height = 60.0;
                                        let (rect, response) = ui.allocate_exact_size(
                                            egui::vec2(ui.available_width(), card_height), 
                                            egui::Sense::click()
                                        );

                                        let bg = if response.hovered() { ui.visuals().widgets.hovered.bg_fill } else { ui.visuals().faint_bg_color };
                                        let stroke = if response.hovered() { ui.visuals().widgets.hovered.fg_stroke } else { egui::Stroke::NONE };
                                        
                                        ui.painter().rect(rect, 8.0, bg, stroke, egui::StrokeKind::Inside);

                                        ui.scope_builder(egui::UiBuilder::new().max_rect(rect.shrink(8.0)), |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(egui::RichText::new(&kanji.kanji).size(32.0).strong());
                                                ui.add_space(10.0);
                                                ui.vertical(|ui| {
                                                    ui.label(egui::RichText::new(format!("{} / {}", kanji.onyomi_romaji, kanji.kunyomi_romaji)).size(11.0));
                                                    
                                                    let meaning = ctx.translate_state.data.entries.get(&kanji.kanji).map(|t| t.meaning.clone()).unwrap_or_default();
                                                    let display_meaning = if meaning.is_empty() { "—".to_string() } else { meaning };
                                                    
                                                    ui.add(egui::Label::new(
                                                        egui::RichText::new(display_meaning).size(12.0).weak()
                                                    ).truncate());
                                                });
                                            });
                                        });

                                        if response.clicked() {
                                            state.input.replace_range(start_idx.., &kanji.kanji);
                                        }

                                        if response.hovered() {
                                            ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                                        }

                                        ui.add_space(8.0);
                                    }
                                });
                            }
                        }
                    } else {
                        ui.label(egui::RichText::new(&local.assistant_hint).weak().italics());
                    }
                });
        });
    });

    None
}