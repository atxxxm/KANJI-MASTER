use eframe::egui;
use crate::ui::context::{AppContext, TabOpenMode};
use crate::ui::tabs::TabType;

struct KanaItem {
    kana: &'static str,
    romaji: &'static str,
}

macro_rules! kana_item {
    ($kana:expr, $romaji:expr) => {
        KanaItem { kana: $kana, romaji: $romaji }
    };
}

static HIRAGANA_LIST: &[KanaItem] = &[
    kana_item!("あ", "a"), kana_item!("い", "i"), kana_item!("う", "u"), kana_item!("え", "e"), kana_item!("お", "o"),
    kana_item!("か", "ka"), kana_item!("き", "ki"), kana_item!("く", "ku"), kana_item!("け", "ke"), kana_item!("こ", "ko"),
    kana_item!("さ", "sa"), kana_item!("し", "shi"), kana_item!("す", "su"), kana_item!("せ", "se"), kana_item!("そ", "so"),
    kana_item!("た", "ta"), kana_item!("ち", "chi"), kana_item!("つ", "tsu"), kana_item!("て", "te"), kana_item!("と", "to"),
    kana_item!("な", "na"), kana_item!("に", "ni"), kana_item!("ぬ", "nu"), kana_item!("ね", "ne"), kana_item!("の", "no"),
    kana_item!("は", "ha"), kana_item!("ひ", "hi"), kana_item!("ふ", "fu"), kana_item!("へ", "he"), kana_item!("ほ", "ho"),
    kana_item!("ま", "ma"), kana_item!("み", "mi"), kana_item!("む", "mu"), kana_item!("め", "me"), kana_item!("も", "mo"),
    kana_item!("や", "ya"), kana_item!("ゆ", "yu"), kana_item!("よ", "yo"),
    kana_item!("ら", "ra"), kana_item!("り", "ri"), kana_item!("る", "ru"), kana_item!("れ", "re"), kana_item!("ろ", "ro"),
    kana_item!("わ", "wa"), kana_item!("を", "wo"), kana_item!("ん", "n"),
];

static KATAKANA_LIST: &[KanaItem] = &[
    kana_item!("ア", "a"), kana_item!("イ", "i"), kana_item!("ウ", "u"), kana_item!("エ", "e"), kana_item!("オ", "o"),
    kana_item!("カ", "ka"), kana_item!("キ", "ki"), kana_item!("ク", "ku"), kana_item!("ケ", "ke"), kana_item!("コ", "ko"),
    kana_item!("サ", "sa"), kana_item!("シ", "shi"), kana_item!("ス", "su"), kana_item!("セ", "se"), kana_item!("ソ", "so"),
    kana_item!("タ", "ta"), kana_item!("チ", "chi"), kana_item!("ツ", "tsu"), kana_item!("テ", "te"), kana_item!("ト", "to"),
    kana_item!("ナ", "na"), kana_item!("ニ", "ni"), kana_item!("ヌ", "nu"), kana_item!("ネ", "ne"), kana_item!("ノ", "no"),
    kana_item!("ハ", "ha"), kana_item!("ヒ", "hi"), kana_item!("フ", "fu"), kana_item!("ヘ", "he"), kana_item!("ホ", "ho"),
    kana_item!("マ", "ma"), kana_item!("ミ", "mi"), kana_item!("ム", "mu"), kana_item!("メ", "me"), kana_item!("モ", "mo"),
    kana_item!("ヤ", "ya"), kana_item!("ユ", "yu"), kana_item!("ヨ", "yo"),
    kana_item!("ラ", "ra"), kana_item!("リ", "ri"), kana_item!("ル", "ru"), kana_item!("レ", "re"), kana_item!("ロ", "ro"),
    kana_item!("ワ", "wa"), kana_item!("ヲ", "wo"), kana_item!("ン", "n"),
];

pub fn render(ui: &mut egui::Ui, is_katakana: &mut bool, ctx: &AppContext) -> Option<(TabType, TabOpenMode)> {
        let current_list: &[KanaItem] = if *is_katakana {
            KATAKANA_LIST
        } else {
            HIRAGANA_LIST
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
        let total_rows = current_list.len().div_ceil(columns);
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