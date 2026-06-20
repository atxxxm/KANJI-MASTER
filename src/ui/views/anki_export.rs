use eframe::egui;
use crate::ui::context::{AppContext, TabOpenMode};
use crate::ui::tabs::{TabType, AnkiExportState, ExportStep, KanjiList};
use crate::ui::theme;
use rfd::FileDialog;
use std::fs::File;
use std::io::Write;

pub fn render(ui: &mut egui::Ui, state: &mut AnkiExportState, ctx: &AppContext) -> Option<(TabType, TabOpenMode)> {
    let local = &ctx.localization.local.top_bar.tools.anki_export_locale;

    ui.add_space(10.0);
    
    // Header
    ui.vertical_centered(|ui| {
        let step_title = match state.step {
            ExportStep::Selection => &local.step_selection,
            ExportStep::Review => &local.step_review,
            ExportStep::Options => &local.step_options,
        };
        ui.heading(egui::RichText::new(format!("{} - {}", &local.title, step_title)).size(24.0).strong());
    });
    
    ui.add_space(20.0);

    match state.step {
        ExportStep::Selection => render_selection(ui, state, ctx, local),
        ExportStep::Review => render_review(ui, state, ctx, local),
        ExportStep::Options => render_options(ui, state, ctx, local),
    }

    None
}

fn render_selection(ui: &mut egui::Ui, state: &mut AnkiExportState, ctx: &AppContext, local: &crate::back::localization::AnkiExportLocale) {
    let kanji_set = state.selected_list;
    let translations = &ctx.translate_state.data.entries;
    let current_search_trim = state.search_query.trim().to_lowercase();
    let is_search_empty = current_search_trim.is_empty();
    
    let query_changed = state.last_search_query.as_deref() != Some(current_search_trim.as_str());
    let filter_changed = state.last_selected_list != Some(kanji_set);

    if query_changed || filter_changed {
        state.cached_results.clear();

        for (idx, item) in ctx.kanji.iter().enumerate() {
            let matches_category = match kanji_set {
                KanjiList::All => true,
                KanjiList::Jlpt5 => item.jlpt == "N5",
                KanjiList::Jlpt4 => item.jlpt == "N4",
                KanjiList::Jlpt3 => item.jlpt == "N3",
                KanjiList::Jlpt2 => item.jlpt == "N2",
                KanjiList::Jlpt1 => item.jlpt == "N1",
            };

            if !matches_category { continue; }

            if is_search_empty {
                state.cached_results.push(idx);
                continue;
            }

            let matches_basic = item.kanji.contains(&current_search_trim)
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
    }

    // Top Bar (Search + Filter)
    ui.horizontal(|ui| {
        ui.add(egui::TextEdit::singleline(&mut state.search_query).hint_text("🔍 Search").desired_width(200.0));
        
        egui::ComboBox::from_id_salt("anki_filter_combo")
            .selected_text(match state.selected_list {
                KanjiList::All => "All JLPT",
                KanjiList::Jlpt5 => "N5",
                KanjiList::Jlpt4 => "N4",
                KanjiList::Jlpt3 => "N3",
                KanjiList::Jlpt2 => "N2",
                KanjiList::Jlpt1 => "N1",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut state.selected_list, KanjiList::All, "All");
                ui.selectable_value(&mut state.selected_list, KanjiList::Jlpt5, "N5");
                ui.selectable_value(&mut state.selected_list, KanjiList::Jlpt4, "N4");
                ui.selectable_value(&mut state.selected_list, KanjiList::Jlpt3, "N3");
                ui.selectable_value(&mut state.selected_list, KanjiList::Jlpt2, "N2");
                ui.selectable_value(&mut state.selected_list, KanjiList::Jlpt1, "N1");
            });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let next_btn = egui::Button::new(egui::RichText::new(&local.next_button).strong().color(theme::ON_ACCENT))
                .fill(ui.visuals().widgets.active.bg_fill);
            if ui.add_enabled(!state.selected_kanji_ids.is_empty(), next_btn).clicked() {
                state.step = ExportStep::Review;
            }
            ui.label(format!("{}: {}", &local.selected_count, state.selected_kanji_ids.len()));
        });
    });

    ui.add_space(10.0);

    // Grid of Kanji
    let item_spacing = 10.0;
    let available_width = ui.available_width();
    let columns = (available_width / 80.0).floor().max(3.0) as usize;
    let card_width = (available_width - (item_spacing * (columns as f32 - 1.0))) / columns as f32;
    let card_height = card_width * 1.2;
    let row_height = card_height + item_spacing;
    let total_rows = state.cached_results.len().div_ceil(columns);

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show_rows(ui, row_height, total_rows, |ui, row_range| {
            ui.spacing_mut().item_spacing = egui::vec2(item_spacing, item_spacing);

            for row_index in row_range {
                ui.horizontal(|ui| {
                    let start = row_index * columns;
                    let end = (start + columns).min(state.cached_results.len());

                    for i in start..end {
                        let kanji_idx = state.cached_results[i];
                        if let Some(item) = ctx.kanji.get(kanji_idx) {
                            let is_selected = state.selected_kanji_ids.contains(&item.id);

                            let (rect, response) = ui.allocate_at_least(
                                egui::vec2(card_width, card_height),
                                egui::Sense::click(),
                            );

                            let bg_color = if is_selected {
                                ui.visuals().widgets.active.bg_fill.gamma_multiply(0.3)
                            } else if response.hovered() {
                                ui.visuals().widgets.hovered.bg_fill
                            } else {
                                ui.visuals().faint_bg_color
                            };

                            let stroke = if is_selected {
                                egui::Stroke::new(2.0, ui.visuals().widgets.active.bg_fill)
                            } else {
                                egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color)
                            };

                            ui.painter().rect(
                                rect,
                                8.0,
                                bg_color,
                                stroke,
                                egui::StrokeKind::Inside,
                            );

                            ui.painter().text(
                                rect.center(),
                                egui::Align2::CENTER_CENTER,
                                &item.kanji,
                                egui::FontId::proportional(card_width * 0.5),
                                ui.visuals().strong_text_color(),
                            );

                            if response.clicked() {
                                if is_selected {
                                    state.selected_kanji_ids.remove(&item.id);
                                } else {
                                    state.selected_kanji_ids.insert(item.id);
                                }
                            }
                            
                            if response.hovered() {
                                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                            }
                        }
                    }
                });
            }
        });
}

fn render_review(ui: &mut egui::Ui, state: &mut AnkiExportState, ctx: &AppContext, local: &crate::back::localization::AnkiExportLocale) {
    ui.horizontal(|ui| {
        if ui.button(&local.back_button).clicked() {
            state.step = ExportStep::Selection;
        }
        if ui.button(&local.clear_all).clicked() {
            state.selected_kanji_ids.clear();
            state.step = ExportStep::Selection;
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let next_btn = egui::Button::new(egui::RichText::new(&local.next_button).strong().color(theme::ON_ACCENT))
                .fill(ui.visuals().widgets.active.bg_fill);
            if ui.add(next_btn).clicked() {
                state.step = ExportStep::Options;
            }
            ui.label(format!("{}: {}", &local.selected_count, state.selected_kanji_ids.len()));
        });
    });

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    let mut to_remove = None;

    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        for id in &state.selected_kanji_ids {
            if let Some(item) = ctx.kanji_by_id.get(id) {
                egui::Frame::NONE
                    .fill(ui.visuals().faint_bg_color)
                    .corner_radius(8)
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(&item.kanji).size(24.0).strong());
                            ui.vertical(|ui| {
                                ui.label(&item.onyomi_romaji);
                                ui.label(&item.kunyomi_romaji);
                            });
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button(egui::RichText::new("❌").color(egui::Color32::RED)).clicked() {
                                    to_remove = Some(*id);
                                }
                            });
                        });
                    });
                ui.add_space(5.0);
            }
        }
    });

    if let Some(id) = to_remove {
        state.selected_kanji_ids.remove(&id);
        if state.selected_kanji_ids.is_empty() {
            state.step = ExportStep::Selection; 
        }
    }
}

fn render_options(ui: &mut egui::Ui, state: &mut AnkiExportState, ctx: &AppContext, local: &crate::back::localization::AnkiExportLocale) {
    ui.horizontal(|ui| {
        if ui.button(&local.back_button).clicked() {
            state.step = ExportStep::Review;
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let export_btn = egui::Button::new(egui::RichText::new(&local.export_button).strong().color(theme::ON_ACCENT))
                .fill(theme::SUCCESS); // Green confirms the final export action
            
            if ui.add(export_btn).clicked() {
                export_to_tsv(state, ctx, local);
            }
        });
    });

    ui.add_space(20.0);

    egui::Frame::NONE
        .fill(ui.visuals().faint_bg_color)
        .corner_radius(12)
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .inner_margin(20.0)
        .show(ui, |ui| {
            ui.heading("Fields to Export");
            ui.add_space(10.0);
            
            ui.checkbox(&mut state.options.kanji, &local.field_kanji);
            ui.checkbox(&mut state.options.meaning, &local.field_meaning);
            ui.checkbox(&mut state.options.onyomi, &local.field_onyomi);
            ui.checkbox(&mut state.options.kunyomi, &local.field_kunyomi);
            ui.checkbox(&mut state.options.jlpt, &local.field_jlpt);
            ui.checkbox(&mut state.options.examples, &local.field_examples);
        });

    ui.add_space(20.0);
    
    if let Some(msg) = &state.status_message {
        ui.label(egui::RichText::new(msg).color(theme::SUCCESS).size(16.0).strong());
    }
}

fn export_to_tsv(state: &mut AnkiExportState, ctx: &AppContext, local: &crate::back::localization::AnkiExportLocale) {
    if state.selected_kanji_ids.is_empty() {
        state.status_message = Some(local.no_kanji_selected.clone());
        return;
    }

    let file_path = FileDialog::new()
        .set_file_name("kanji_anki_export.txt")
        .add_filter("Text File", &["txt"])
        .save_file();

    if let Some(path) = file_path {
        let mut lines = Vec::new();

        // Generate Headers
        let mut headers = Vec::new();
        if state.options.kanji { headers.push(local.field_kanji.as_str()); }
        if state.options.meaning { headers.push(local.field_meaning.as_str()); }
        if state.options.onyomi { headers.push(local.field_onyomi.as_str()); }
        if state.options.kunyomi { headers.push(local.field_kunyomi.as_str()); }
        if state.options.jlpt { headers.push(local.field_jlpt.as_str()); }
        if state.options.examples { headers.push(local.field_examples.as_str()); }
        
        lines.push(headers.join("\t")); // TSV Separator

        // Generate Data Rows
        for id in &state.selected_kanji_ids {
            if let Some(kanji) = ctx.kanji_by_id.get(id) {
                let mut row = Vec::new();
                let translation = ctx.translate_state.data.entries.get(&kanji.kanji);

                if state.options.kanji { row.push(kanji.kanji.clone()); }
                if state.options.meaning {
                    let meaning = translation.map(|t| t.meaning.clone()).unwrap_or_default();
                    row.push(meaning);
                }
                if state.options.onyomi { row.push(kanji.onyomi.clone()); }
                if state.options.kunyomi { row.push(kanji.kunyomi.clone()); }
                if state.options.jlpt { row.push(kanji.jlpt.clone()); }
                
                if state.options.examples {
                    let mut ex_html = String::new();
                    for (i, ex) in kanji.example.iter().enumerate() {
                        let tr = translation.and_then(|t| t.translate_examples.get(i)).cloned().unwrap_or_default();
                        if !tr.is_empty() {
                            ex_html.push_str(&format!("{} — {}", ex, tr));
                        } else {
                            ex_html.push_str(ex);
                        }
                        
                        if i < kanji.example.len() - 1 {
                            ex_html.push_str("<br>"); 
                        }
                    }
                    row.push(ex_html);
                }

                // Clean tabs and newlines from strings to prevent breaking TSV format
                let clean_row: Vec<String> = row.into_iter()
                    .map(|s| s.replace(['\t', '\n'], " "))
                    .collect();

                lines.push(clean_row.join("\t"));
            }
        }

        // Write to File
        match File::create(path) {
            Ok(mut file) => {
                if file.write_all(lines.join("\n").as_bytes()).is_ok() {
                    state.status_message = Some(local.success_message.clone());
                } else {
                    state.status_message = Some(local.error_message.clone());
                }
            }
            Err(_) => {
                state.status_message = Some(local.error_message.clone());
            }
        }
    }
}