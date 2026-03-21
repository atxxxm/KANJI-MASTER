use crate::back::config::Config;
use crate::back::core::Kanji;
use crate::back::localization::Localization;
use crate::back::translation::TranslateState;
use crate::ui::settings::Settings;
use crate::back::svg_cache::SvgCache;

pub struct AppContext<'a> {
    pub kanji: &'a Vec<Kanji>,
    pub config: &'a mut Config,
    pub settings: &'a Settings,
    pub localization: &'a Localization,
    pub translate_state: &'a mut TranslateState,
    pub svg_cache: &'a SvgCache,
}

pub enum TabOpenMode {
    ReplaceCurrent,
    NewTabActive,
    NewTabBackground,
}