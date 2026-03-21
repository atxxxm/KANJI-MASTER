use eframe::egui;
use crate::ui::context::{AppContext, TabOpenMode};
use crate::ui::tabs::TabType;

struct KanaItem {
    kana: &'static str,
    romaji: &'static str,
}


pub fn render(ui: &mut egui::Ui, is_katakana: &mut bool, ctx: &AppContext) -> Option<(TabType, TabOpenMode)> {
        let hiragana_list: Vec<KanaItem> = vec![
            KanaItem {
                kana: "あ",
                romaji: "a",
            },
            KanaItem {
                kana: "い",
                romaji: "i",
            },
            KanaItem {
                kana: "う",
                romaji: "u",
            },
            KanaItem {
                kana: "え",
                romaji: "e",
            },
            KanaItem {
                kana: "お",
                romaji: "o",
            },
            KanaItem {
                kana: "か",
                romaji: "ka",
            },
            KanaItem {
                kana: "き",
                romaji: "ki",
            },
            KanaItem {
                kana: "く",
                romaji: "ku",
            },
            KanaItem {
                kana: "け",
                romaji: "ke",
            },
            KanaItem {
                kana: "こ",
                romaji: "ko",
            },
            KanaItem {
                kana: "さ",
                romaji: "sa",
            },
            KanaItem {
                kana: "し",
                romaji: "shi",
            },
            KanaItem {
                kana: "す",
                romaji: "su",
            },
            KanaItem {
                kana: "せ",
                romaji: "se",
            },
            KanaItem {
                kana: "そ",
                romaji: "so",
            },
            KanaItem {
                kana: "た",
                romaji: "ta",
            },
            KanaItem {
                kana: "ち",
                romaji: "chi",
            },
            KanaItem {
                kana: "つ",
                romaji: "tsu",
            },
            KanaItem {
                kana: "て",
                romaji: "te",
            },
            KanaItem {
                kana: "と",
                romaji: "to",
            },
            KanaItem {
                kana: "な",
                romaji: "na",
            },
            KanaItem {
                kana: "に",
                romaji: "ni",
            },
            KanaItem {
                kana: "ぬ",
                romaji: "nu",
            },
            KanaItem {
                kana: "ね",
                romaji: "ne",
            },
            KanaItem {
                kana: "の",
                romaji: "no",
            },
            KanaItem {
                kana: "は",
                romaji: "ha",
            },
            KanaItem {
                kana: "ひ",
                romaji: "hi",
            },
            KanaItem {
                kana: "ふ",
                romaji: "fu",
            },
            KanaItem {
                kana: "へ",
                romaji: "he",
            },
            KanaItem {
                kana: "ほ",
                romaji: "ho",
            },
            KanaItem {
                kana: "ま",
                romaji: "ma",
            },
            KanaItem {
                kana: "み",
                romaji: "mi",
            },
            KanaItem {
                kana: "む",
                romaji: "mu",
            },
            KanaItem {
                kana: "め",
                romaji: "me",
            },
            KanaItem {
                kana: "も",
                romaji: "mo",
            },
            KanaItem {
                kana: "や",
                romaji: "ya",
            },
            KanaItem {
                kana: "ゆ",
                romaji: "yu",
            },
            KanaItem {
                kana: "よ",
                romaji: "yo",
            },
            KanaItem {
                kana: "ら",
                romaji: "ra",
            },
            KanaItem {
                kana: "り",
                romaji: "ri",
            },
            KanaItem {
                kana: "る",
                romaji: "ru",
            },
            KanaItem {
                kana: "れ",
                romaji: "re",
            },
            KanaItem {
                kana: "ろ",
                romaji: "ro",
            },
            KanaItem {
                kana: "わ",
                romaji: "wa",
            },
            KanaItem {
                kana: "を",
                romaji: "wo",
            },
            KanaItem {
                kana: "ん",
                romaji: "n",
            },
        ];

        let katakana_list: Vec<KanaItem> = vec![
            KanaItem {
                kana: "ア",
                romaji: "a",
            },
            KanaItem {
                kana: "イ",
                romaji: "i",
            },
            KanaItem {
                kana: "ウ",
                romaji: "u",
            },
            KanaItem {
                kana: "エ",
                romaji: "e",
            },
            KanaItem {
                kana: "オ",
                romaji: "o",
            },
            KanaItem {
                kana: "カ",
                romaji: "ka",
            },
            KanaItem {
                kana: "キ",
                romaji: "ki",
            },
            KanaItem {
                kana: "ク",
                romaji: "ku",
            },
            KanaItem {
                kana: "ケ",
                romaji: "ke",
            },
            KanaItem {
                kana: "コ",
                romaji: "ko",
            },
            KanaItem {
                kana: "サ",
                romaji: "sa",
            },
            KanaItem {
                kana: "シ",
                romaji: "shi",
            },
            KanaItem {
                kana: "ス",
                romaji: "su",
            },
            KanaItem {
                kana: "セ",
                romaji: "se",
            },
            KanaItem {
                kana: "ソ",
                romaji: "so",
            },
            KanaItem {
                kana: "タ",
                romaji: "ta",
            },
            KanaItem {
                kana: "チ",
                romaji: "chi",
            },
            KanaItem {
                kana: "ツ",
                romaji: "tsu",
            },
            KanaItem {
                kana: "テ",
                romaji: "te",
            },
            KanaItem {
                kana: "ト",
                romaji: "to",
            },
            KanaItem {
                kana: "ナ",
                romaji: "na",
            },
            KanaItem {
                kana: "ニ",
                romaji: "ni",
            },
            KanaItem {
                kana: "ヌ",
                romaji: "nu",
            },
            KanaItem {
                kana: "ネ",
                romaji: "ne",
            },
            KanaItem {
                kana: "ノ",
                romaji: "no",
            },
            KanaItem {
                kana: "ハ",
                romaji: "ha",
            },
            KanaItem {
                kana: "ヒ",
                romaji: "hi",
            },
            KanaItem {
                kana: "フ",
                romaji: "fu",
            },
            KanaItem {
                kana: "ヘ",
                romaji: "he",
            },
            KanaItem {
                kana: "ホ",
                romaji: "ho",
            },
            KanaItem {
                kana: "マ",
                romaji: "ma",
            },
            KanaItem {
                kana: "ミ",
                romaji: "mi",
            },
            KanaItem {
                kana: "ム",
                romaji: "mu",
            },
            KanaItem {
                kana: "メ",
                romaji: "me",
            },
            KanaItem {
                kana: "モ",
                romaji: "mo",
            },
            KanaItem {
                kana: "ヤ",
                romaji: "ya",
            },
            KanaItem {
                kana: "ユ",
                romaji: "yu",
            },
            KanaItem {
                kana: "ヨ",
                romaji: "yo",
            },
            KanaItem {
                kana: "ラ",
                romaji: "ra",
            },
            KanaItem {
                kana: "リ",
                romaji: "ri",
            },
            KanaItem {
                kana: "ル",
                romaji: "ru",
            },
            KanaItem {
                kana: "レ",
                romaji: "re",
            },
            KanaItem {
                kana: "ロ",
                romaji: "ro",
            },
            KanaItem {
                kana: "ワ",
                romaji: "wa",
            },
            KanaItem {
                kana: "ヲ",
                romaji: "wo",
            },
            KanaItem {
                kana: "ン",
                romaji: "n",
            },
        ];

        let current_list = if *is_katakana {
            &katakana_list
        } else {
            &hiragana_list
        };
        let local = &ctx.localization.local.kana;

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new(&local.title)
                    .size(24.0)
                    .strong()
                    .color(ui.visuals().strong_text_color()),
            );
            ui.add_space(15.0);

            let pill_bg = ui.visuals().widgets.inactive.bg_fill;
            let pill_stroke = ui.visuals().widgets.noninteractive.bg_stroke;

            ui.horizontal(|ui| {
                let pill_width = 213.0;
                ui.add_space((ui.available_width() - pill_width) / 2.0);

                egui::Frame::NONE
                    .fill(pill_bg)
                    .corner_radius(20)
                    .stroke(pill_stroke)
                    .inner_margin(egui::Margin::symmetric(6, 4))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;

                            let toggle_btn = |ui: &mut egui::Ui, text: &str, selected: bool| {
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
                                let response = ui.add(btn);
                                if selected {
                                    ui.painter().text(
                                        response.rect.center(),
                                        egui::Align2::CENTER_CENTER,
                                        text,
                                        egui::FontId::proportional(14.0),
                                        text_color,
                                    );
                                }
                                response
                            };

                            if toggle_btn(ui, &local.hiragana, !*is_katakana).clicked() {
                                *is_katakana = false;
                            }

                            ui.allocate_ui(egui::vec2(1.0, 20.0), |ui| {
                                ui.painter().line_segment(
                                    [ui.min_rect().center_top(), ui.min_rect().center_bottom()],
                                    ui.visuals().widgets.noninteractive.bg_stroke,
                                );
                            });

                            if toggle_btn(ui, &local.katakana, *is_katakana).clicked() {
                                *is_katakana = true;
                            }
                        });
                    });
            });
        });
        ui.add_space(20.0);

        let available_width = ui.available_width();
        
        let target_card_width = 85.0; 
        let spacing = 12.0;
        
        let max_content_width = 900.0;
        let content_width = (available_width - 20.0).min(max_content_width);
        
        let mut columns = ((content_width + spacing) / (target_card_width + spacing)).floor() as usize;
        columns = columns.clamp(4, 10); 

        let card_width = (content_width - (spacing * (columns as f32 - 1.0))) / columns as f32;
        let card_height = card_width * 1.15; 
        
        let side_padding = (available_width - content_width) / 2.0;
        let total_rows = (current_list.len() + columns - 1) / columns;
        let row_height = card_height + spacing;

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show_rows(ui, row_height, total_rows, |ui, row_range| {
                ui.spacing_mut().item_spacing.y = spacing;

                for row_index in row_range {
                    ui.horizontal(|ui| {
                        ui.add_space(side_padding);
                        ui.spacing_mut().item_spacing.x = spacing;

                        let start_index = row_index * columns;
                        let end_index = (start_index + columns).min(current_list.len());

                        for i in start_index..end_index {
                            if let Some(item) = current_list.get(i) {
                                let (rect, response) = ui.allocate_at_least(
                                    egui::vec2(card_width, card_height),
                                    egui::Sense::hover(),
                                );

                                let is_hovered = response.hovered();

                                let bg_color = if is_hovered {
                                    ui.visuals().widgets.hovered.bg_fill
                                } else {
                                    ui.visuals().faint_bg_color
                                };

                                let border_stroke = if is_hovered {
                                    ui.visuals().widgets.hovered.fg_stroke
                                } else {
                                    egui::Stroke::new(
                                        1.0,
                                        ui.visuals()
                                            .widgets
                                            .noninteractive
                                            .bg_stroke
                                            .color
                                            .gamma_multiply(0.5),
                                    )
                                };

                                ui.painter().rect(
                                    rect,
                                    egui::CornerRadius::same(12),
                                    bg_color,
                                    border_stroke,
                                    egui::StrokeKind::Outside,
                                );

                                ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                                    ui.vertical_centered(|ui| {
                                        let kana_size = (card_width * 0.45).clamp(20.0, 42.0);
                                        let romaji_size = (card_width * 0.22).clamp(12.0, 18.0); 

                                        let total_h = kana_size + romaji_size + 5.0;
                                        ui.add_space((card_height - total_h) / 2.0);

                                        ui.label(
                                            egui::RichText::new(item.kana)
                                                .size(kana_size)
                                                .strong()
                                                .color(ui.visuals().strong_text_color()),
                                        );

                                        ui.add_space(2.0);

                                        ui.label(
                                            egui::RichText::new(item.romaji)
                                                .size(romaji_size)
                                                .color(
                                                    ui.visuals().text_color().gamma_multiply(0.6),
                                                ),
                                        );
                                    });
                                });

                                if is_hovered {
                                    ui.output_mut(|o| {
                                        o.cursor_icon = egui::CursorIcon::PointingHand
                                    });
                                }
                            }
                        }
                    });
                }
            });
        None
    }