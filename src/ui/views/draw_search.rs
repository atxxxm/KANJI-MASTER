use eframe::egui;
use crate::ui::context::{AppContext, TabOpenMode};
use crate::ui::tabs::TabType;
use crate::ui::theme;
use std::sync::Arc;
use crate::ui::views::kanji_detail;

pub fn render(ui: &mut egui::Ui, state: &mut crate::ui::tabs::DrawSearchState, ctx: &AppContext, recognition_system: &crate::back::recognition::RecognitionSystem) -> Option<(TabType, TabOpenMode)> {
    let mut tab_action = None;
    let local = &ctx.localization.local.top_bar.tools.draw_and_search;

    // Header Section
    ui.vertical_centered(|ui| {
        ui.add_space(10.0);
        ui.heading(egui::RichText::new(&local.draw_kanji).size(24.0).strong());
        ui.label(
            egui::RichText::new(&local.hint_text)
                .size(12.0)
                .weak(),
        );
        ui.add_space(15.0);
    });

    // Determine layout based on available width
    let available_width = ui.available_width();
    let canvas_size_val = 400.0;
    let spacing = 20.0;
    let min_sidebar_width = 250.0;
    
    // Breakpoint: if not enough space for canvas + spacing + sidebar, stack vertically
    let is_horizontal_layout = available_width > (canvas_size_val + spacing + min_sidebar_width);

    macro_rules! render_canvas_panel {
        ($ui:expr) => {
            $ui.vertical(|ui| {
                // Canvas Container
                let canvas_rect_size = egui::vec2(canvas_size_val, canvas_size_val);
                
                // Frame for the canvas to give it a "pad" look
                egui::Frame::NONE
                    .fill(ui.visuals().faint_bg_color)
                    .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                    .corner_radius(16.0)
                    .inner_margin(0.0)
                    .show(ui, |ui| {
                        let (response, painter) = ui.allocate_painter(canvas_rect_size, egui::Sense::drag());
                        let rect = response.rect;

                        // Background helper lines
                        painter.rect_filled(rect, 16.0, ui.visuals().faint_bg_color);
                        
                        // Input Processing
                        if response.dragged() {
                            if let Some(pointer_pos) = response.interact_pointer_pos() {
                                state.current_stroke.push(pointer_pos);
                            }
                        }

                        if response.drag_stopped() {
                            if !state.current_stroke.is_empty() {
                                state.strokes.push(state.current_stroke.clone());
                                state.current_stroke.clear();
                            }
                        }

                        // Drawing
                        let stroke_color = ui.visuals().strong_text_color();
                        let stroke_width = 5.0;
                        let stroke_opts = egui::Stroke::new(stroke_width, stroke_color);

                        // Draw finished strokes
                        for stroke in &state.strokes {
                            if stroke.len() > 1 {
                                painter.add(egui::Shape::line(stroke.clone(), stroke_opts));
                            } else if stroke.len() == 1 {
                                painter.circle_filled(stroke[0], stroke_width / 2.0, stroke_color);
                            }
                        }

                        // Draw current stroke
                        if state.current_stroke.len() > 1 {
                            painter.add(egui::Shape::line(state.current_stroke.clone(), stroke_opts));
                        }
                    });

                ui.add_space(15.0);

                // Control Panel (Buttons)
                ui.horizontal(|ui| {
                    ui.set_width(canvas_size_val);
                    
                    // Clear Button
                    if ui.add(
                        egui::Button::new(egui::RichText::new("🗑").size(16.0))
                            .min_size(egui::vec2(40.0, 40.0))
                    ).on_hover_text(&local.clear_button).clicked() {
                        state.strokes.clear();
                        state.results.clear();
                    }

                    // Undo Button
                    if ui.add(
                        egui::Button::new(egui::RichText::new("⬅").size(16.0))
                            .min_size(egui::vec2(40.0, 40.0))
                    ).on_hover_text(&local.undo_button).clicked() {
                        state.strokes.pop();
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let search_btn = egui::Button::new(
                            egui::RichText::new(&format!("🔍 {}", &local.search_button))
                                .strong()
                                .size(16.0)
                                .color(theme::ON_ACCENT)
                        )
                        .min_size(egui::vec2(120.0, 40.0))
                        .fill(ui.visuals().widgets.active.bg_fill)
                        .corner_radius(8);

                        if ui.add(search_btn).clicked() {
                            // Convert inputs
                            let input_points: Vec<Vec<(f32, f32)>> = state.strokes.iter()
                                .map(|stroke| stroke.iter().map(|p| (p.x, p.y)).collect())
                                .collect();

                            // Logic
                            let matches = recognition_system.search(&input_points, 20);
                            
                            state.results = matches.iter()
                            .filter_map(|(id, _score)| {
                                ctx.kanji_by_id.get(id).map(Arc::clone)
                            })
                            .collect();
                        }
                    });
                });
            });
        };
    }

    macro_rules! render_results_panel {
        ($ui:expr, $is_sidebar:expr) => {
            $ui.vertical(|ui| {
                if $is_sidebar {
                    ui.set_min_width(min_sidebar_width);
                } else {
                    ui.set_width(ui.available_width());
                }

                ui.label(egui::RichText::new(&local.best_matches).strong().size(18.0));
                ui.add_space(10.0);

                // Results Container
                let frame = egui::Frame::NONE
                    .fill(ui.visuals().window_fill)
                    .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                    .corner_radius(12.0)
                    .inner_margin(10.0);

                frame.show(ui, |ui| {
                    if state.results.is_empty() {
                        ui.centered_and_justified(|ui| {
                            let text = if state.strokes.is_empty() {
                                &local.draw_something
                            } else {
                                &local.not_match_yet
                            };
                            ui.label(egui::RichText::new(text).weak().italics());
                        });
                    } else {
                        let scroll_area = egui::ScrollArea::vertical()
                            .auto_shrink([false, false]); // fill available space

                        scroll_area.show(ui, |ui| {
                            // Use wrapping layout for cards
                            ui.horizontal_wrapped(|ui| {
                                ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);
                                
                                for kanji in &state.results {
                                    let card_size = egui::vec2(60.0, 70.0);
                                    let (rect, response) = ui.allocate_at_least(card_size, egui::Sense::click());

                                    let is_hovered = response.hovered();
                                    let bg_color = if is_hovered {
                                        ui.visuals().widgets.hovered.bg_fill
                                    } else {
                                        ui.visuals().faint_bg_color
                                    };
                                    let stroke = if is_hovered {
                                        ui.visuals().widgets.hovered.fg_stroke
                                    } else {
                                        ui.visuals().widgets.noninteractive.bg_stroke
                                    };

                                    ui.painter().rect(
                                        rect,
                                        8.0,
                                        bg_color,
                                        stroke,
                                        egui::StrokeKind::Outside
                                    );

                                    // Kanji Text
                                    ui.painter().text(
                                        rect.center() - egui::vec2(0.0, 5.0),
                                        egui::Align2::CENTER_CENTER,
                                        &kanji.kanji,
                                        egui::FontId::proportional(32.0),
                                        ui.visuals().strong_text_color(),
                                    );
                                    
                                    ui.painter().text(
                                        rect.center() + egui::vec2(0.0, 20.0),
                                        egui::Align2::CENTER_CENTER,
                                        &kanji.onyomi_romaji.split_whitespace().next().unwrap_or(""), // Show first reading
                                        egui::FontId::proportional(9.0),
                                        ui.visuals().text_color().gamma_multiply(0.6),
                                    );

                                    if response.clicked() {
                                        let new_content = kanji_detail::create_tab(Arc::clone(kanji), ctx.svg_cache);
                                        tab_action = Some((new_content, TabOpenMode::NewTabActive));
                                    }

                                    if response.secondary_clicked() || response.middle_clicked() {
                                        let new_content = kanji_detail::create_tab(Arc::clone(kanji), ctx.svg_cache);
                                        tab_action = Some((new_content, TabOpenMode::NewTabBackground));
                                    }

                                    
                                    if is_hovered {
                                        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                                    }
                                }
                            });
                        });
                    }
                });
            });
        };
    }

    // Layout Switch
    if is_horizontal_layout {
        // Horizontal Split: Canvas Left | Results Right
        ui.horizontal(|ui| {
            // Centering the canvas block relative to available height
            ui.add_space((available_width - (canvas_size_val + spacing + min_sidebar_width)).max(0.0) / 2.0);

            render_canvas_panel!(ui);
            
            ui.add_space(spacing);
            
            render_results_panel!(ui, true);
        });
    } else {
        // Vertical Stack: Canvas Top | Results Bottom
        ui.vertical_centered(|ui| {
            render_canvas_panel!(ui);
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);
            
            // For vertical layout, give results a fixed height
            ui.allocate_ui(egui::vec2(ui.available_width(), 300.0), |ui| {
                render_results_panel!(ui, false);
            });
        });
    }

    tab_action
}