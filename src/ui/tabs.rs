use crate::back::core::Kanji;
use crate::ui::animator::KanjiAnimator;
use crate::back::localization::Localization;
use eframe::egui;
use rand;
use std::sync::Arc;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KanjiList {
    All,
    Jlpt5,
    Jlpt4,
    Jlpt3,
    Jlpt2,
    Jlpt1,
}

#[derive(Clone, Default)]
pub struct HomeState {
    pub search_query: String,
    pub last_search_query: Option<String>,
    pub cached_results: Vec<usize>,
}

#[derive(Clone)]
pub struct KanjiListState {
    pub search_query: String,
    pub selected_list: KanjiList,
    pub last_search_query: Option<String>,
    pub last_selected_list: Option<KanjiList>,
    pub cached_results: Vec<usize>,
}


// Kanji List State Default
impl Default for KanjiListState {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            selected_list: KanjiList::All,
            last_search_query: None,
            last_selected_list: None,
            cached_results: Vec::new(),
        }
    }
}

// State for tab Current Kanji
pub struct KanjiDetailState {
    pub kanji: Arc<Kanji>,
    pub animator: KanjiAnimator,
    pub last_copy_time: Option<f64>,
}

// State for tab Translate
#[derive(Clone)]
pub struct TranslateTabState {
    pub jump_search_buffer: String
}

// Translate Tab State Default
impl Default for TranslateTabState {
    fn default() -> Self {
        Self {
            jump_search_buffer: String::new(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ExportStep {
    Selection,
    Review,
    Options,
}

#[derive(Clone)]
pub struct ExportOptions {
    pub kanji: bool,
    pub meaning: bool,
    pub onyomi: bool,
    pub kunyomi: bool,
    pub jlpt: bool,
    pub examples: bool,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            kanji: true,
            meaning: true,
            onyomi: true,
            kunyomi: true,
            jlpt: true,
            examples: true,
        }
    }
}

#[derive(Clone)]
pub struct AnkiExportState {
    pub step: ExportStep,
    pub search_query: String,
    pub selected_list: KanjiList,
    pub last_search_query: Option<String>,
    pub last_selected_list: Option<KanjiList>,
    pub cached_results: Vec<usize>,
    pub selected_kanji_ids: HashSet<i32>,
    pub options: ExportOptions,
    pub status_message: Option<String>,
}

impl Default for AnkiExportState {
    fn default() -> Self {
        Self {
            step: ExportStep::Selection,
            search_query: String::new(),
            selected_list: KanjiList::All,
            last_search_query: None,
            last_selected_list: None,
            cached_results: Vec::new(),
            selected_kanji_ids: HashSet::new(),
            options: ExportOptions::default(),
            status_message: None,
        }
    }
}

// State for tab Romaji to Kana
#[derive(Clone)]
pub struct RomajiKanaState {
    pub input: String,
    pub is_katakana: bool
}

// Romaji Kana State Default
impl Default for RomajiKanaState {
    fn default() -> Self {
        Self {
            input: String::new(),
            is_katakana: false,
        }
    }
}

#[derive(Clone)]
pub struct DrawSearchState {
    pub strokes: Vec<Vec<egui::Pos2>>, 
    pub current_stroke: Vec<egui::Pos2>,
    pub results: Vec<Arc<Kanji>>,
}

impl Default for DrawSearchState {
    fn default() -> Self {
        Self {
            strokes: Vec::new(),
            current_stroke: Vec::new(),
            results: Vec::new(),
        }
    }
}

// Main Tab Struct
pub enum TabType {
    Home(HomeState),
    KanjiList(KanjiListState),
    KanjiDetail(KanjiDetailState),
    Kana(bool), 
    RomajiToKana(RomajiKanaState),
    Translate(TranslateTabState),
    AnkiExport(AnkiExportState),
    DrawSearch(DrawSearchState),
}

// Tab Type Partial Eq
impl PartialEq for TabType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl TabType {
    pub fn title(&self, local: &Localization) -> String {
        match self {
            TabType::Home(_) => format!("🏠 {}", local.local.top_bar.tabs.home),
            TabType::KanjiList(_) => format!("📃 {}", local.local.top_bar.tabs.kanji_list),
            TabType::KanjiDetail(state) => format!("字 {}", state.kanji.kanji),
            TabType::Kana(_) => format!("あ {}", local.local.top_bar.tabs.kana),
            TabType::RomajiToKana(_) => format!("{}", local.local.top_bar.tabs.romaji_to_kana),
            TabType::Translate(_) => format!("🌐 {}", local.local.top_bar.tabs.translate_kanji),
            TabType::AnkiExport(_) => format!("📦 {}", local.local.top_bar.tabs.anki_export),
            TabType::DrawSearch(_) => format!("🎨 {}", local.local.top_bar.tabs.draw_search),
        }
    }
}

// Tabs Struct
pub struct Tab {
    pub id: String,
    pub content: TabType,
}

impl Tab {
    pub fn new(content: TabType) -> Self {
        let id = rand::random::<u64>().to_string();
        Self { id, content }
    }
}

// Tab Action Enum
enum TabAction {
    Switch(usize),
    Close(usize),
    CloseOthers(usize),
}

// Manager TabManager Struct
pub struct TabManager {
    pub tabs: Vec<Tab>,
    pub active_tab_index: usize,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: vec![Tab::new(TabType::Home(HomeState::default()))],
            active_tab_index: 0,
        }
    }

    // Add tab
    pub fn add_tab(&mut self, content: TabType, switch_to_it: bool) {
        let new_tab = Tab::new(content);
        self.tabs.push(new_tab);
        if switch_to_it {
            self.active_tab_index = self.tabs.len() - 1;
        }
    }

    // Close tab
    pub fn close_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.tabs.remove(index);

            // Correct active tab index
            if self.active_tab_index >= index && self.active_tab_index > 0 {
                self.active_tab_index -= 1;
            }

            // If close tab is last, and tab is one, create Home tab
            if self.tabs.is_empty() {
                self.tabs.push(Tab::new(TabType::Home(HomeState::default())));
                self.active_tab_index = 0;
            }
        }
    }

    // Active tab
    pub fn active_content_mut(&mut self) -> Option<&mut TabType> {
        self.tabs.get_mut(self.active_tab_index).map(|t| &mut t.content)
    }

    // Ui for tabs line
    pub fn ui(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context, local: &Localization) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            
            let mut action: Option<TabAction> = None;
            
            let mut dragged_index: Option<usize> = None;
            let mut target_index: Option<usize> = None;
            
            let pointer_pos = ui.input(|i| i.pointer.hover_pos());

            egui::ScrollArea::horizontal().show(ui, |ui| {
                for (index, tab) in self.tabs.iter().enumerate() {
                    let is_active = index == self.active_tab_index;
                    let title = tab.content.title(local);
                    
                    let bg_color = if is_active {
                        ui.visuals().widgets.active.bg_fill
                    } else {
                        ui.visuals().widgets.inactive.bg_fill
                    };
                    
                    let fg_color = if is_active {
                        ui.visuals().strong_text_color()
                    } else {
                        ui.visuals().text_color()
                    };

                    let item_id = ui.id().with("tab").with(&tab.id);

                    let frame = egui::Frame::NONE
                        .fill(bg_color)
                        .corner_radius(egui::CornerRadius { nw: 8, ne: 8, sw: 0, se: 0 })
                        .inner_margin(egui::Margin::symmetric(10, 6))
                        .stroke(if is_active { egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color) } else { egui::Stroke::NONE });

                    let frame_response = frame.show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let title_btn = egui::Button::new(egui::RichText::new(&title).color(fg_color))
                                .frame(false)
                                .sense(egui::Sense::click_and_drag()); 

                            let title_response = ui.add(title_btn);

                            let interact = ui.interact(title_response.rect, item_id, egui::Sense::click_and_drag());

                            if interact.dragged() {
                                dragged_index = Some(index);
                                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Grabbing);
                            }

                            if interact.clicked() && !interact.dragged() {
                                action = Some(TabAction::Switch(index));
                            }
                            if interact.clicked_by(egui::PointerButton::Middle) {
                                action = Some(TabAction::Close(index));
                            }

                            ui.add_space(4.0);

                            let close_btn = ui.add(
                                egui::Button::new(egui::RichText::new("×").size(14.0).color(fg_color))
                                    .frame(false)
                                    .min_size(egui::vec2(16.0, 16.0))
                            );

                            if close_btn.clicked() {
                                action = Some(TabAction::Close(index));
                            }
                            
                            if close_btn.hovered() {
                                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                            }
                        });
                    });

                    frame_response.response.context_menu(|ui| {
                        if ui.button("Close Tab").clicked() {
                            action = Some(TabAction::Close(index));
                            ui.close();
                        }
                        if ui.button("Close Others").clicked() {
                            action = Some(TabAction::CloseOthers(index));
                            ui.close();
                        }
                    });

                    if let Some(pos) = pointer_pos {
                        if frame_response.response.rect.contains(pos) {
                            target_index = Some(index);
                        }
                    }
                }
            });

            if let (Some(from), Some(to)) = (dragged_index, target_index) {
                if from != to {
                    self.tabs.swap(from, to);
                    
                    if self.active_tab_index == from {
                        self.active_tab_index = to;
                    } else if self.active_tab_index == to {
                        self.active_tab_index = from;
                    }
                    
                    ui.ctx().request_repaint();
                }
            }

            match action {
                Some(TabAction::Switch(index)) => {
                    self.active_tab_index = index;
                }
                Some(TabAction::Close(index)) => {
                    self.close_tab(index);
                }
                Some(TabAction::CloseOthers(index)) => {
                    if index < self.tabs.len() {
                        let keep_tab = self.tabs.remove(index);
                        self.tabs.clear();
                        self.tabs.push(keep_tab);
                        self.active_tab_index = 0;
                    }
                }
                None => {}
            }

            let new_tab_btn = ui.add(egui::Button::new("+").frame(false));
            if new_tab_btn.clicked() {
                self.add_tab(TabType::Home(HomeState::default()), true);
            }
            
            new_tab_btn.context_menu(|ui| {
                if ui.button("New Kanji List").clicked() {
                    self.add_tab(TabType::KanjiList(KanjiListState::default()), true);
                    ui.close();
                }
                if ui.button("New Translator").clicked() {
                     self.add_tab(TabType::Translate(TranslateTabState::default()), true);
                    ui.close();
                }
            });
        });
        
        ui.separator();
    }
}