use eframe::egui;
use crate::ui::context::{AppContext, TabOpenMode};
use crate::ui::tabs::{TabType, CardsSetupState};
use crate::back::cards::{CardsSession, DeckBuilderState};
use crate::back::core::Kanji;

pub fn render_setup(ui: &mut egui::Ui, state: &mut CardsSetupState, ctx: &mut AppContext) -> Option<(TabType, TabOpenMode)> {
    let local = &ctx.localization.local.top_bar.cards;
    let panel_rounding = egui::CornerRadius::same(16);
    let item_rounding = egui::CornerRadius::same(12);
    let panel_bg = ui.visuals().faint_bg_color;
    let border_stroke = ui.visuals().widgets.noninteractive.bg_stroke;

    let mut next_tab: Option<TabType> = None;

    ui.add_space(10.0);
    ui.vertical_centered(|ui| {
        ui.heading(egui::RichText::new(&local.card_setup_title).size(24.0).strong());
    });
    ui.add_space(20.0);

    ui.columns(2, |cols| {
        cols[0].vertical(|ui| {
            egui::Frame::NONE
                .fill(panel_bg)
                .corner_radius(panel_rounding)
                .stroke(border_stroke)
                .inner_margin(15.0)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new(&local.jlpt_level).strong().size(18.0));
                    });
                    ui.add_space(15.0);

                    egui::Frame::NONE
                        .fill(ui.visuals().window_fill)
                        .corner_radius(12)
                        .inner_margin(egui::Margin::symmetric(10, 8))
                        .stroke(border_stroke)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}:", &local.card_count));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.add(egui::DragValue::new(&mut state.card_limit).range(5..=1000).speed(1.0));
                                });
                            });
                        });
                    
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);

                    let levels = vec!["N5", "N4", "N3", "N2", "N1"];
                    
                    for level in levels {
                        let btn_text = format!("{} {}", &local.card_setup_start, level);
                        
                        let btn = egui::Button::new(egui::RichText::new(&btn_text).size(16.0))
                            .min_size(egui::vec2(ui.available_width(), 45.0))
                            .corner_radius(item_rounding);

                        if ui.add(btn).clicked() {
                            let deck_name = format!("JLPT {}", level);
                            let filtered: Vec<Kanji> = ctx.kanji.iter()
                                .filter(|k| k.jlpt == level)
                                .cloned()
                                .collect();
                            
                            if !filtered.is_empty() {
                                let session = CardsSession::new(filtered, state.card_limit);
                                next_tab = Some(TabType::CardsActive(session, deck_name));
                            }
                        }
                        ui.add_space(10.0);
                    }
                });
        });

        cols[1].vertical(|ui| {
            egui::Frame::NONE
                .fill(panel_bg)
                .corner_radius(panel_rounding)
                .stroke(border_stroke)
                .inner_margin(15.0)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());

                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new(&local.custom_deck_title).strong().size(18.0));
                    });
                    ui.add_space(15.0);

                    let create_btn = egui::Button::new(
                        egui::RichText::new(format!("➕ {}", &local.manage_decks)).size(16.0)
                    )
                    .min_size(egui::vec2(ui.available_width(), 40.0))
                    .corner_radius(item_rounding);

                    if ui.add(create_btn).clicked() {
                        let mut builder = DeckBuilderState::default();
                        builder.is_editing = false;
                        next_tab = Some(TabType::DeckManager(builder));
                    }

                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(15.0);
                    
                    ui.label(egui::RichText::new(format!("{}:", &local.my_decks)).strong());
                    ui.add_space(5.0);

                    let deck_names: Vec<String> = ctx.config.custom_decks.keys().cloned().collect();

                    if deck_names.is_empty() {
                        ui.centered_and_justified(|ui| {
                            ui.label(egui::RichText::new(&local.no_custom_decks_created).weak().italics());
                        });
                    } else {
                        egui::ScrollArea::vertical()
                            .max_height(400.0)
                            .show(ui, |ui| {
                                let mut deck_to_delete = None;

                                for name in deck_names {
                                    egui::Frame::NONE
                                        .fill(ui.visuals().window_fill)
                                        .stroke(border_stroke)
                                        .corner_radius(item_rounding)
                                        .inner_margin(10.0)
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(egui::RichText::new(&name).strong().size(15.0));

                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    let play_btn = egui::Button::new(format!("▶ {}", &local.play_deck))
                                                        .fill(ui.visuals().widgets.active.bg_fill)
                                                        .corner_radius(8);
                                                    
                                                    if ui.add(play_btn).clicked() {
                                                        if let Some(ids) = ctx.config.custom_decks.get(&name) {
                                                            let deck_kanji: Vec<Kanji> = ctx.kanji.iter()
                                                                .filter(|k| ids.contains(&k.id))
                                                                .cloned()
                                                                .collect();
                                                            if !deck_kanji.is_empty() {
                                                                let session = CardsSession::new(deck_kanji, 0);
                                                                next_tab = Some(TabType::CardsActive(session, name.clone()));
                                                            }
                                                        }
                                                    }

                                                    ui.add_space(5.0);

                                                    if ui.button("✏").on_hover_text(&local.edit_deck).clicked() {
                                                        if let Some(ids) = ctx.config.custom_decks.get(&name) {
                                                            let mut builder = DeckBuilderState::default();
                                                            builder.deck_name_buffer = name.clone();
                                                            builder.selected_kanji_idx = ids.clone();
                                                            builder.is_editing = true;
                                                            next_tab = Some(TabType::DeckManager(builder));
                                                        }
                                                    }

                                                    if ui.button(egui::RichText::new("🗑").color(egui::Color32::RED))
                                                        .on_hover_text(&local.delete_deck)
                                                        .clicked() 
                                                    {
                                                        deck_to_delete = Some(name.clone());
                                                    }
                                                });
                                            });
                                        });
                                    ui.add_space(8.0);
                                }

                                if let Some(name) = deck_to_delete {
                                    ctx.config.custom_decks.remove(&name);
                                }
                            });
                    }
                });
        });
    });

    next_tab.map(|tab| (tab, TabOpenMode::ReplaceCurrent))
}


pub fn render_session(ui: &mut egui::Ui, session: &mut CardsSession, deck_name: &String, ctx: &AppContext) -> Option<(TabType, TabOpenMode)> {
    let local = ctx.localization.local.top_bar.cards.clone();
    let local_common = &ctx.localization.local.screens.current_kanji;

    if session.finished {
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("🏆").size(64.0));
                ui.add_space(10.0);
                ui.heading(egui::RichText::new(&local.session_complete).size(24.0).strong());
                ui.add_space(20.0);
                
                let btn = egui::Button::new(egui::RichText::new(&local.return_to_menu).size(18.0))
                    .min_size(egui::vec2(200.0, 50.0))
                    .fill(ui.visuals().widgets.active.bg_fill)
                    .corner_radius(12);

                if ui.add(btn).clicked() {
                    return Some((TabType::CardsSetup(CardsSetupState::default()), false));
                }

                return None;
            });
        });
        return None;
    }

    let current_idx = session.current_index;
    let total = session.total_count;
    let current_kanji = match session.queue.get(current_idx) {
        Some(k) => k.clone(),
        None => return None,
    };

    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.heading(egui::RichText::new(deck_name).strong());
            ui.label(egui::RichText::new(format!("{} {} / {}", &local.card, current_idx + 1, total)).weak());
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(egui::RichText::new(format!("❌ {}", &local.exit_training))).clicked() {
                return Some((TabType::CardsSetup(CardsSetupState::default()), false));
            }
            None
        });
    });

    ui.add_space(10.0);

    let progress = if total > 0 {
        (current_idx as f32) / (total as f32)
    } else {
        0.0
    };

    ui.add(egui::ProgressBar::new(progress).show_percentage().animate(false));

    ui.add_space(20.0);

    let card_w = 320.0;
    let card_h = 500.0;
    
    ui.vertical_centered(|ui| {
        let (rect, response) = ui.allocate_exact_size(egui::vec2(card_w, card_h), egui::Sense::click());
        let is_hovered = response.hovered();
        let bg_color = ui.visuals().window_fill;
        let stroke_color = if is_hovered {
            ui.visuals().widgets.active.bg_stroke.color
        } else {
            ui.visuals().widgets.noninteractive.bg_stroke.color
        };
    
        ui.painter().rect(
            rect,
            16.0,
            bg_color,
            egui::Stroke::new(if is_hovered { 1.5 } else { 1.0 }, stroke_color),
            egui::StrokeKind::Outside,
        );

        if response.clicked() {
            session.is_card_flipped = !session.is_card_flipped;
        }

        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.label(
                    egui::RichText::new(&current_kanji.kanji)
                        .size(90.0)
                        .strong()
                        .color(ui.visuals().strong_text_color())
                );

                ui.add_space(20.0);

                ui.scope(|ui| {
                    ui.set_min_height(340.0);

                    if session.is_card_flipped {
                        ui.separator();
                        ui.add_space(15.0);

                        let meaning_text = if let Some(entry) = ctx.translate_state.data.entries.get(&current_kanji.kanji) {
                            &entry.meaning
                        } else {
                            ""
                        };

                        if !meaning_text.is_empty() {
                            ui.label(egui::RichText::new(meaning_text).size(20.0).strong().color(ui.visuals().text_color()));
                            ui.add_space(5.0);
                            ui.label(egui::RichText::new(&local_common.meaning).size(12.0).weak());
                            ui.add_space(15.0);
                            ui.separator();
                            ui.add_space(15.0);
                        }

                        ui.columns(2, |cols| {
                            cols[0].vertical_centered(|ui| {
                                ui.label(egui::RichText::new(&local.onyomi).size(12.0).weak());
                                ui.add_space(2.0);
                                ui.label(egui::RichText::new(&current_kanji.onyomi).strong().size(16.0));
                                ui.label(egui::RichText::new(&current_kanji.onyomi_romaji).italics().size(14.0));
                            });

                            cols[1].vertical_centered(|ui| {
                                ui.label(egui::RichText::new(&local.kunyomi).size(12.0).weak());
                                ui.add_space(2.0);
                                ui.label(egui::RichText::new(&current_kanji.kunyomi).strong().size(16.0));
                                ui.label(egui::RichText::new(&current_kanji.kunyomi_romaji).italics().size(14.0));
                            });
                        });
                    

                    } else {
                        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        ui.add_space(20.0);
                        ui.label(egui::RichText::new(&local.click_to_flip).weak().size(12.0));
                    });
                }

                });
            });
        });

        ui.add_space(25.0);

        let next_btn = egui::Button::new(
            egui::RichText::new(format!("{} ➡", &local.next_card))
                .size(18.0)
                .strong()
        )
        .min_size(egui::vec2(200.0, 45.0))
        .fill(ui.visuals().widgets.open.bg_fill) 
        .corner_radius(20);

        if ui.add(next_btn).clicked() {
            session.next();
        }
        });

    None
}

