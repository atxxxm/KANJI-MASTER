use eframe::egui;
use crate::ui::context::{AppContext, TabOpenMode};
use crate::ui::tabs::{TabType, HomeState};
use crate::ui::views::kanji_detail;

pub fn render(ui: &mut egui::Ui, state: &mut HomeState, ctx: &AppContext) -> Option<(TabType, TabOpenMode)> {
    let mut tab_action = None;
    let translations = &ctx.translate_state.data.entries;
    let current_search_trim = state.search_query.trim().to_lowercase();
    let is_search_empty = current_search_trim.is_empty();

    let query_changed = state.last_search_query.as_deref() != Some(current_search_trim.as_str());

    if query_changed {
        state.cached_results.clear();
        
        if !is_search_empty {
            for (idx, item) in ctx.kanji.iter().enumerate() {
                let matches_basic = item.kanji.contains(&current_search_trim)
                    || item.onyomi.contains(&current_search_trim)
                    || item.kunyomi.contains(&current_search_trim)
                    || item.onyomi_romaji.contains(&current_search_trim)
                    || item.kunyomi_romaji.contains(&current_search_trim);

                let matches_meaning = if let Some(entry) = translations.get(&item.kanji) {
                    entry.meaning.to_lowercase().contains(&current_search_trim)
                } else {
                    false
                };

                if matches_basic || matches_meaning {
                    state.cached_results.push(idx);
                }
            }
        }
        state.last_search_query = Some(current_search_trim);
    }

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

                let text_edit = egui::TextEdit::singleline(&mut state.search_query)
                    .id_source("home_search_field")
                    .hint_text(&ctx.localization.local.home.kanji_search_hint)
                    .frame(false)
                    .desired_width(f32::INFINITY)
                    .font(egui::FontId::proportional(20.0));

                let output = ui.add(text_edit);

                if ctx.settings.focus_on_search {
                    if state.search_query.is_empty() && !ui.memory(|m| m.has_focus(output.id)) {
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

    if state.cached_results.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.label(egui::RichText::new(&ctx.localization.local.home.kanji_not_found).weak().size(18.0));
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
    let total_rows = (state.cached_results.len() + columns - 1) / columns;

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show_rows(ui, row_height, total_rows, |ui, row_range| {
            ui.spacing_mut().item_spacing.x = item_spacing;
            ui.spacing_mut().item_spacing.y = item_spacing;

            for row_index in row_range {
                ui.horizontal(|ui| {
                    ui.add_space(side_padding);

                    let start_index = row_index * columns;
                    let end_index = (start_index + columns).min(state.cached_results.len());

                    for i in start_index..end_index {
                        let kanji_idx = state.cached_results[i];
                        if let Some(item) = ctx.kanji.get(kanji_idx) {
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
                                let new_content = kanji_detail::create_tab((*item).clone(), ctx.svg_cache);
                                tab_action = Some((new_content, TabOpenMode::NewTabActive));
                            }

                            if response.secondary_clicked() || response.middle_clicked() {
                                let new_content = kanji_detail::create_tab((*item).clone(), ctx.svg_cache);
                                tab_action = Some((new_content, TabOpenMode::NewTabBackground));
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