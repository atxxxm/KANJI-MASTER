use crate::back::config::Config;
use crate::back::core::Kanji;
use crate::back::localization::{Localization, Paths};
use crate::back::translation::TranslateState;
use crate::ui::settings::Settings;

pub struct AppContext<'a> {
    pub kanji: &'a Vec<Kanji>,
    pub config: &'a mut Config,
    pub paths: &'a Paths,
    pub settings: &'a Settings,
    pub localization: &'a Localization,
    pub translate_state: &'a mut TranslateState,
}

pub enum TabOpenMode {
    ReplaceCurrent,
    NewTabActive,
    NewTabBackground,
}