use crate::ui::tabs::{KanjiList, KanjiListState, StrokeFilter};
use crate::ui::context::{AppContext, TabOpenMode};
use crate::ui::tabs::TabType;
use crate::ui::views::kanji_detail;
use std::sync::Arc;

pub fn render(ui: &mut egui::Ui, state: &mut KanjiListState, ctx: &AppContext) -> Option<(TabType, TabOpenMode)> {
    let mut tab_action = None;
    let kanji_set = state.selected_list;
    let translations = &ctx.translate_state.data.entries;
    let current_search_trim = state.search_query.trim().to_lowercase();
    let is_search_empty = current_search_trim.is_empty();
    let query_changed = state.last_search_query.as_deref() != Some(current_search_trim.as_str());
    let stroke_filter = state.stroke_filter;
    let filter_changed = state.last_selected_list != Some(kanji_set)
        || state.last_stroke_filter != Some(stroke_filter);

    if query_changed || filter_changed {
        state.cached_results.clear();

        for (idx, item) in ctx.kanji.iter().enumerate() {
            let matches_jlpt = match kanji_set {
                KanjiList::All => true,
                KanjiList::Jlpt5 => item.jlpt == "N5",
                KanjiList::Jlpt4 => item.jlpt == "N4",
                KanjiList::Jlpt3 => item.jlpt == "N3",
                KanjiList::Jlpt2 => item.jlpt == "N2",
                KanjiList::Jlpt1 => item.jlpt == "N1",
            };

            let matches_strokes = match stroke_filter {
                StrokeFilter::Any => true,
                StrokeFilter::S1to4 => item.strokes >= 1 && item.strokes <= 4,
                StrokeFilter::S5to8 => item.strokes >= 5 && item.strokes <= 8,
                StrokeFilter::S9to12 => item.strokes >= 9 && item.strokes <= 12,
                StrokeFilter::S13plus => item.strokes >= 13,
            };

            if !matches_jlpt || !matches_strokes {
                continue;
            }

            if is_search_empty {
                state.cached_results.push(idx);
                continue;
            }

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

        state.last_search_query = Some(current_search_trim);
        state.last_selected_list = Some(kanji_set);
        state.last_stroke_filter = Some(stroke_filter);
    }

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
            let combo_width = 110.0;
            let spacing = 10.0;
            let search_width = ui.available_width() - (combo_width + spacing) * 2.0;

            // Search bar
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
                        .id(egui::Id::new("kanji_list_search"))
                        .hint_text(&ctx.localization.local.screens.search)
                        .frame(false)
                        .desired_width(f32::INFINITY)
                        .font(egui::FontId::proportional(18.0))
                        .margin(egui::vec2(0.0, 2.0));
                    ui.add(text_edit);
                });
            });

            ui.add_space(spacing);

            // JLPT filter
            pill_frame.show(ui, |ui| {
                ui.set_width(combo_width);
                ui.set_min_height(28.0);
                let jlpt_label = match state.selected_list {
                    KanjiList::All => "JLPT",
                    KanjiList::Jlpt5 => "N5",
                    KanjiList::Jlpt4 => "N4",
                    KanjiList::Jlpt3 => "N3",
                    KanjiList::Jlpt2 => "N2",
                    KanjiList::Jlpt1 => "N1",
                };
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.add_space(2.0);
                    egui::ComboBox::from_id_salt("kanji_filter_combo")
                        .selected_text(egui::RichText::new(format!("⚡ {}", jlpt_label)).strong())
                        .width(combo_width - 20.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut state.selected_list,
                                KanjiList::All,
                                &ctx.localization.local.top_bar.kanji.all,
                            );
                            ui.separator();
                            ui.selectable_value(&mut state.selected_list, KanjiList::Jlpt5, "JLPT N5");
                            ui.selectable_value(&mut state.selected_list, KanjiList::Jlpt4, "JLPT N4");
                            ui.selectable_value(&mut state.selected_list, KanjiList::Jlpt3, "JLPT N3");
                            ui.selectable_value(&mut state.selected_list, KanjiList::Jlpt2, "JLPT N2");
                            ui.selectable_value(&mut state.selected_list, KanjiList::Jlpt1, "JLPT N1");
                        });
                });
            });

            ui.add_space(spacing);

            // Stroke count filter
            pill_frame.show(ui, |ui| {
                ui.set_width(combo_width);
                ui.set_min_height(28.0);
                let stroke_label = match state.stroke_filter {
                    StrokeFilter::Any => "Strokes",
                    StrokeFilter::S1to4 => "1–4",
                    StrokeFilter::S5to8 => "5–8",
                    StrokeFilter::S9to12 => "9–12",
                    StrokeFilter::S13plus => "13+",
                };
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.add_space(2.0);
                    egui::ComboBox::from_id_salt("stroke_filter_combo")
                        .selected_text(egui::RichText::new(format!("✏ {}", stroke_label)).strong())
                        .width(combo_width - 20.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut state.stroke_filter, StrokeFilter::Any, "Any");
                            ui.separator();
                            ui.selectable_value(&mut state.stroke_filter, StrokeFilter::S1to4, "1–4 strokes");
                            ui.selectable_value(&mut state.stroke_filter, StrokeFilter::S5to8, "5–8 strokes");
                            ui.selectable_value(&mut state.stroke_filter, StrokeFilter::S9to12, "9–12 strokes");
                            ui.selectable_value(&mut state.stroke_filter, StrokeFilter::S13plus, "13+ strokes");
                        });
                });
            });
        });
    });

    ui.add_space(20.0);

    if state.cached_results.is_empty() {
        ui.centered_and_justified(|ui| {
            ui.label(egui::RichText::new(&ctx.localization.local.home.kanji_not_found).weak().size(18.0));
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
    let total_rows = state.cached_results.len().div_ceil(columns);

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
                                            .strong()
                                            .color(ui.visuals().strong_text_color()),
                                    );

                                    if ctx.settings.show_kanji_meaning
                                        && let Some(entry) = translations.get(&item.kanji)
                                            && !entry.meaning.is_empty() {
                                                let meaning_text = if entry.meaning.chars().count() > 18 {
                                                    let truncated: String =
                                                        entry.meaning.chars().take(16).collect();
                                                    format!("{}...", truncated)
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
                                });
                            });

                            if response.clicked() {
                                let new_content = kanji_detail::create_tab(Arc::clone(item), ctx.svg_cache);
                                tab_action = Some((new_content, TabOpenMode::NewTabActive));
                            }

                            if response.secondary_clicked() || response.middle_clicked() {
                                let new_content = kanji_detail::create_tab(Arc::clone(item), ctx.svg_cache);
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