use eframe::egui;
use std::path::Path;
use crate::ui::context::{AppContext, TabOpenMode};
use crate::ui::tabs::{TabType, KanjiDetailState};
use crate::ui::animator::KanjiAnimator;
use crate::back::core::Kanji;
use crate::back::localization::Paths;

pub fn create_tab(kanji: Kanji, paths: &Paths) -> TabType {
    let mut animator = KanjiAnimator::new();
    let svg_path = format!("{}/0{}.svg", paths.path_to_svg_images, kanji.unicode.to_lowercase());

    if Path::new(&svg_path).exists() {
        if let Err(_) = animator.load_svg(&svg_path) {
            animator.clear();
        }
    } else {
        animator.clear();
    }

    TabType::KanjiDetail(KanjiDetailState {
        kanji,
        animator,
        last_copy_time: None,
    })
}

pub fn render(ui: &mut egui::Ui, state: &mut KanjiDetailState, ctx: &AppContext) -> Option<(TabType, TabOpenMode)> {
    let kanji = &state.kanji;
    let local = &ctx.localization.local.screens.current_kanji;
    let translation_entry = ctx.translate_state.data.entries.get(&kanji.kanji);

    let panel_rounding = egui::CornerRadius::same(16);
    let panel_fill = ui.visuals().faint_bg_color;
    let panel_stroke = ui.visuals().widgets.noninteractive.bg_stroke;

    let content_frame = egui::Frame::NONE
        .fill(panel_fill)
        .corner_radius(panel_rounding)
        .stroke(panel_stroke)
        .inner_margin(15.0);

    ui.add_space(25.0);

    ui.columns(2, |columns| {
        let available_w = columns[0].available_width().min(340.0);
        let card_w = available_w * 0.94;
        let card_h = card_w * (4.0 / 3.0);

        columns[0].vertical_centered(|ui| {
            ui.add_space(5.0);

            egui::Frame::canvas(ui.style())
                .fill(ui.visuals().window_fill)
                .stroke(egui::Stroke::new(
                    1.5,
                    ui.visuals().widgets.noninteractive.bg_stroke.color,
                ))
                .corner_radius(20)
                .shadow(egui::Shadow::NONE)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    let (rect, response) = ui
                        .allocate_exact_size(egui::vec2(card_w, card_h), egui::Sense::click());

                    if response.clicked() {
                        state.animator.replay();
                    }

                    if response.hovered() {
                        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                    }

                    let anim_side = rect.width().min(rect.height()) * 0.88;
                    let center = rect.center();
                    let draw_rect =
                        egui::Rect::from_center_size(center, egui::vec2(anim_side, anim_side));

                    state.animator.ui(ui, draw_rect, &kanji.kanji, ctx.settings.animation_speed);
                });

            ui.add_space(10.0);
            ui.label(
                egui::RichText::new(&local.click_to_replay)
                    .size(10.0)
                    .weak(),
            );

            ui.add_space(15.0);

            let current_time = ui.input(|i| i.time);
            let mut show_checkmark = false;

            if let Some(last_time) = state.last_copy_time {
                if current_time - last_time < 2.0 {
                    show_checkmark = true;
                    ui.ctx().request_repaint();
                }
            }

            let btn_bg = ui.visuals().widgets.inactive.bg_fill;
            let btn_stroke = ui.visuals().widgets.noninteractive.bg_stroke;

            let (icon, label_text) = if show_checkmark {
                ("✅", "Copied")
            } else {
                ("📋", "Copy")
            };
            
            let btn_text = egui::RichText::new(format!("{} {}", icon, label_text))
                .size(12.0)
                .strong();

            let copy_btn = egui::Button::new(btn_text)
                .fill(btn_bg)
                .stroke(btn_stroke)
                .corner_radius(12.0)
                .min_size(egui::vec2(100.0, 24.0));

            if ui.add(copy_btn)
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .on_hover_text("Copy to clipboard") 
                .clicked() 
            {
                ui.ctx().copy_text(kanji.kanji.clone());
                state.last_copy_time = Some(current_time);
            }
        }); 

        columns[1].vertical(|ui| {
            ui.add_space(5.0);

            let meaning = if let Some(entry) = translation_entry {
                if !entry.meaning.is_empty() {
                    &entry.meaning
                } else {
                    ""
                }
            } else {
                ""
            };

            ui.vertical_centered_justified(|ui| {
                ui.label(
                    egui::RichText::new(meaning)
                        .size(28.0)
                        .strong()
                        .color(ui.visuals().strong_text_color()),
                );
            });
            ui.add_space(15.0);

            let badge = |ui: &mut egui::Ui, label: &str, value: &str| {
                let bg = ui.visuals().widgets.inactive.bg_fill;
                egui::Frame::NONE
                    .fill(bg)
                    .corner_radius(12)
                    .inner_margin(egui::Margin::symmetric(12, 6))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(label).size(11.0).weak());
                            ui.label(egui::RichText::new(value).size(12.0).strong());
                        });
                    });
            };

            ui.horizontal_wrapped(|ui| {
                badge(ui, &local.jlpt, &kanji.jlpt);
                ui.add_space(5.0);
                badge(ui, &local.grade, &kanji.grade);
                ui.add_space(5.0);
                badge(ui, &local.strokes, &format!("{}", &kanji.strokes));
                ui.add_space(5.0);
                badge(ui, &local.frequency, &kanji.frequency);
            });

            ui.add_space(15.0);

            content_frame.show(ui, |ui| {
                ui.set_width(ui.available_width());

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(&local.onyomi)
                                .size(12.0)
                                .weak()
                                .italics(),
                        );
                        ui.add_space(2.0);
                        ui.label(egui::RichText::new(&kanji.onyomi).size(16.0).strong());
                        ui.label(
                            egui::RichText::new(&kanji.onyomi_romaji)
                                .size(12.0)
                                .color(ui.visuals().text_color().gamma_multiply(0.6)),
                        );
                    });

                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);

                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(&local.kunyomi)
                                .size(12.0)
                                .weak()
                                .italics(),
                        );
                        ui.add_space(2.0);
                        ui.label(egui::RichText::new(&kanji.kunyomi).size(16.0).strong());
                        ui.label(
                            egui::RichText::new(&kanji.kunyomi_romaji)
                                .size(12.0)
                                .color(ui.visuals().text_color().gamma_multiply(0.6)),
                        );
                    });
                });
            });

            ui.add_space(20.0);

            ui.label(egui::RichText::new(&local.examples).strong().size(18.0));
            ui.add_space(8.0);

            let scroll_height = ui.available_height() - 20.0;

            egui::ScrollArea::vertical()
                .max_height(scroll_height)
                .id_salt("examples_scroll")
                .show(ui, |ui| {
                    for (index, example_jp) in kanji.example.iter().enumerate() {
                        let translation_text = if let Some(entry) = translation_entry {
                            entry
                                .translate_examples
                                .get(index)
                                .cloned()
                                .unwrap_or_default()
                        } else {
                            String::new()
                        };

                        egui::Frame::NONE
                            .fill(ui.visuals().widgets.open.weak_bg_fill)
                            .corner_radius(8)
                            .stroke(egui::Stroke::new(
                                1.0,
                                ui.visuals()
                                    .widgets
                                    .noninteractive
                                    .bg_stroke
                                    .color
                                    .gamma_multiply(0.5),
                            ))
                            .inner_margin(10.0)
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new("•")
                                            .color(ui.visuals().warn_fg_color),
                                    );

                                    ui.vertical(|ui| {
                                        ui.label(egui::RichText::new(example_jp).size(15.0));

                                        if !translation_text.is_empty() {
                                            ui.add_space(2.0);
                                            ui.label(
                                                egui::RichText::new(&translation_text)
                                                    .italics()
                                                    .size(13.0)
                                                    .color(
                                                        ui.visuals()
                                                            .text_color()
                                                            .gamma_multiply(0.7),
                                                    ),
                                            );
                                        }
                                    });
                                });
                            });

                        ui.add_space(8.0);
                    }

                    if kanji.example.is_empty() {
                        ui.label(
                            egui::RichText::new(&local.no_examples_available)
                                .weak()
                                .italics(),
                        );
                    }
                });
        });
    });
    None
}