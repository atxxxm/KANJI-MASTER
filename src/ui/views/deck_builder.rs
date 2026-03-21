use eframe::egui;
use crate::ui::context::{AppContext, TabOpenMode};
use crate::ui::tabs::{TabType, CardsSetupState};
use crate::back::cards::DeckBuilderState;
use crate::back::core::Kanji;
use std::sync::Arc;

pub fn render(ui: &mut egui::Ui, state: &mut DeckBuilderState, ctx: &mut AppContext) -> Option<(TabType, TabOpenMode)> {
    let local = ctx.localization.local.top_bar.cards.clone();
    
    let panel_rounding = egui::CornerRadius::same(12);
    let item_rounding = egui::CornerRadius::same(8);
    let panel_bg = ui.visuals().faint_bg_color;
    let border_stroke = ui.visuals().widgets.noninteractive.bg_stroke;

    ui.add_space(10.0);
    ui.horizontal(|ui| {
        if ui.add(egui::Button::new(egui::RichText::new(format!("⬅ {}", &local.back)).size(16.0)).frame(false)).clicked() {
            return Some((TabType::CardsSetup(CardsSetupState::default()), false));
        }

        let title = if state.is_editing {
            &local.edit_deck
        } else {
            &local.deck_manager
        };
        ui.heading(egui::RichText::new(title).strong().size(20.0));

        None
    });
    ui.add_space(15.0);

    egui::Frame::NONE
        .fill(ui.visuals().window_fill)
        .stroke(border_stroke)
        .corner_radius(panel_rounding)
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("{}:", &local.deck_name)).strong());
                
                let pill_bg = ui.visuals().widgets.inactive.bg_fill;
                egui::Frame::NONE
                    .fill(pill_bg)
                    .corner_radius(16)
                    .stroke(border_stroke)
                    .inner_margin(egui::Margin::symmetric(10, 5))
                    .show(ui, |ui| {
                        ui.set_width(300.0);
                        ui.add_enabled(
                            !state.is_editing,
                            egui::TextEdit::singleline(&mut state.deck_name_buffer)
                                .frame(false)
                                .hint_text(&ctx.localization.local.top_bar.cards.my_new_decks)
                        );
                    });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let btn_text = if state.is_editing {
                        &local.update_deck
                    } else {
                        &local.save_deck
                    };
                    
                    let save_btn = egui::Button::new(egui::RichText::new(btn_text).strong())
                        .min_size(egui::vec2(100.0, 35.0))
                        .fill(ui.visuals().widgets.active.bg_fill)
                        .corner_radius(8);

                    if ui.add(save_btn).clicked() {
                        if !state.deck_name_buffer.is_empty()
                            && !state.selected_kanji_idx.is_empty()
                        {
                            ctx.config.custom_decks.insert(
                                state.deck_name_buffer.clone(),
                                state.selected_kanji_idx.clone(),
                            );
                            *state = DeckBuilderState::default();
                        }
                    }
                });
            });
        });

    ui.add_space(15.0);

    ui.columns(2, |cols| {
        cols[0].vertical(|ui| {
            ui.label(egui::RichText::new(&local.avaliable_kanji).strong().size(16.0));
            ui.add_space(5.0);
            
            let pill_bg = ui.visuals().widgets.inactive.bg_fill;
            egui::Frame::NONE
                .fill(pill_bg)
                .corner_radius(16)
                .stroke(border_stroke)
                .inner_margin(egui::Margin::symmetric(10, 6))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        ui.label("🔍");
                        ui.add(
                            egui::TextEdit::singleline(&mut state.search_buffer)
                                .frame(false)
                                .hint_text(&local.search_kanji_hint)
                        );
                    });
                });

            ui.add_space(10.0);

            egui::Frame::NONE
                .fill(panel_bg)
                .stroke(border_stroke)
                .corner_radius(panel_rounding)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    let search = state.search_buffer.to_lowercase();
                    
                    egui::ScrollArea::vertical()
                        .id_salt("source_list")
                        .max_height(450.0)
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            
                            let filtered: Vec<&Arc<Kanji>> = ctx.kanji.iter()
                                .filter(|k| {
                                    if search.is_empty() { return false; }
                                    k.kanji.contains(&search)
                                        || k.onyomi_romaji.contains(&search)
                                        || k.kunyomi_romaji.contains(&search)
                                })
                                .take(50)
                                .collect();

                            if search.is_empty() {
                                ui.centered_and_justified(|ui| {
                                    ui.label(egui::RichText::new(&local.type_search_hint).weak());
                                });
                            } else if filtered.is_empty() {
                                    ui.centered_and_justified(|ui| {
                                    ui.label(egui::RichText::new(&ctx.localization.local.top_bar.cards.no_matches).weak());
                                });
                            }

                            for k in filtered {
                                let is_added = state.selected_kanji_idx.contains(&k.id);
                                
                                ui.group(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new(&k.kanji).size(20.0).strong());
                                        ui.vertical(|ui| {
                                            ui.label(egui::RichText::new(&k.onyomi_romaji).size(10.0).weak());
                                            ui.label(egui::RichText::new(&k.kunyomi_romaji).size(10.0).weak());
                                        });

                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            if is_added {
                                                ui.label(egui::RichText::new(format!("✔ {}", &ctx.localization.local.top_bar.cards.added)).color(egui::Color32::GREEN).size(12.0));
                                            } else {
                                                let btn = egui::Button::new(format!("{} ➡", &local.add_kanji))
                                                    .corner_radius(item_rounding);
                                                if ui.add(btn).clicked() {
                                                    state.selected_kanji_idx.push(k.id);
                                                }
                                            }
                                        });
                                    });
                                });
                                ui.add_space(4.0);
                            }
                        });
                });
        });

        cols[1].vertical(|ui| {
            ui.label(
                egui::RichText::new(format!("{} ({})", &local.deck_content, state.selected_kanji_idx.len()))
                    .strong()
                    .size(16.0)
            );
            ui.add_space(5.0);

            egui::Frame::NONE
                .fill(panel_bg)
                .stroke(border_stroke)
                .corner_radius(panel_rounding)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    
                    egui::ScrollArea::vertical()
                        .id_salt("deck_list")
                        .max_height(500.0) 
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            
                            if state.selected_kanji_idx.is_empty() {
                                ui.centered_and_justified(|ui| {
                                    ui.label(egui::RichText::new(&ctx.localization.local.top_bar.cards.decks_is_empty).weak());
                                });
                            }

                            let mut ids_to_remove = Vec::new();

                            for (idx, id) in state.selected_kanji_idx.iter().enumerate() {
                                if let Some(k) = ctx.kanji.iter().find(|k| k.id == *id) {
                                    ui.group(|ui| {
                                        ui.horizontal(|ui| {
                                            // Delete Button
                                            let btn = egui::Button::new(egui::RichText::new("❌").color(egui::Color32::RED))
                                                .frame(false);
                                            if ui.add(btn).clicked() {
                                                ids_to_remove.push(idx);
                                            }

                                            ui.label(egui::RichText::new(&k.kanji).size(20.0).strong());
                                            
                                            // Extra info
                                            ui.label(egui::RichText::new(format!("[{}]", k.id)).size(10.0).weak());
                                            ui.label(egui::RichText::new(&k.onyomi).size(12.0));
                                        });
                                    });
                                    ui.add_space(4.0);
                                }
                            }

                            for idx in ids_to_remove.iter().rev() {
                                state.selected_kanji_idx.remove(*idx);
                            }
                        });
                });
        });
    });

    None
}