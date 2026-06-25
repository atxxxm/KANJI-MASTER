use crate::back::config::Config;
use crate::back::core::Kanji;
use crate::back::localization::Localization;
use crate::back::translation::TranslateState;
use crate::ui::settings::Settings;
use crate::back::svg_cache::SvgCache;
use std::collections::HashMap;
use std::sync::Arc;

pub struct AppContext<'a> {
    pub kanji: &'a Vec<Arc<Kanji>>,
    pub kanji_by_id: &'a HashMap<i32, Arc<Kanji>>,
    #[allow(dead_code)]
    pub config: &'a mut Config,
    pub settings: &'a Settings,
    pub localization: &'a Localization,
    pub translate_state: &'a mut TranslateState,
    pub svg_cache: &'a mut SvgCache,
}

pub enum TabOpenMode {
    #[allow(dead_code)]
    ReplaceCurrent,
    NewTabActive,
    NewTabBackground,
}