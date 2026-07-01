mod back;

use back::config::{get_config_path, load_config, save_config, Config};
use back::core::{Database, Kanji};
use back::recognition::RecognitionSystem;
use back::romaji_kana::to_kana;
use back::svg_cache::SvgCache;
use back::translation::TranslationFile;

use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager, State};

// ── App state ────────────────────────────────────────────────────────────────

struct AppState {
    kanji: Vec<Arc<Kanji>>,
    kanji_by_char: HashMap<String, Arc<Kanji>>,
    // Lazily parses each kanji's SVG (KanjiVG 0-109 coords) on first request
    // instead of parsing every file at startup.
    svg_cache: SvgCache,
    recognition: RecognitionSystem,
    config: Config,
    // kanji char → meaning string (loaded from user's localization JSON)
    kanji_meanings: HashMap<String, String>,
}

type AppStateHandle = Mutex<AppState>;

// ── Serializable DTO ─────────────────────────────────────────────────────────

#[derive(Serialize, Clone)]
struct KanjiDto {
    id: i32,
    kanji: String,
    strokes: i8,
    jlpt: String,
    grade: String,
    frequency: String,
    unicode: String,
    onyomi: String,
    onyomi_romaji: String,
    kunyomi: String,
    kunyomi_romaji: String,
    examples: Vec<String>,
    meaning: Option<String>,
}

fn to_dto(k: &Kanji, meaning: Option<String>) -> KanjiDto {
    KanjiDto {
        id: k.id,
        kanji: k.kanji.clone(),
        strokes: k.strokes,
        jlpt: k.jlpt.clone(),
        grade: k.grade.clone(),
        frequency: k.frequency.clone(),
        unicode: k.unicode.clone(),
        onyomi: k.onyomi.clone(),
        onyomi_romaji: k.onyomi_romaji.clone(),
        kunyomi: k.kunyomi.clone(),
        kunyomi_romaji: k.kunyomi_romaji.clone(),
        examples: k.example.clone(),
        meaning,
    }
}

// ── Tauri commands ───────────────────────────────────────────────────────────

#[tauri::command]
fn get_kanji_list(state: State<AppStateHandle>) -> Vec<KanjiDto> {
    let st = state.lock().unwrap();
    st.kanji
        .iter()
        .map(|k| {
            let meaning = st.kanji_meanings.get(&k.kanji).cloned();
            to_dto(k, meaning)
        })
        .collect()
}

#[tauri::command]
fn get_kanji_by_char(state: State<AppStateHandle>, ch: String) -> Option<KanjiDto> {
    let st = state.lock().unwrap();
    st.kanji_by_char.get(&ch).map(|k| {
        let meaning = st.kanji_meanings.get(&k.kanji).cloned();
        to_dto(k, meaning)
    })
}

#[tauri::command]
fn convert_romaji(input: String, is_katakana: bool, live_input: bool) -> String {
    to_kana(&input, is_katakana, live_input)
}

/// Returns stroke point arrays for canvas animation, parsing the kanji's SVG
/// file on first request and caching the result for subsequent calls.
/// Each stroke is a flat sequence of [x, y] in KanjiVG 0-109 coordinates.
#[tauri::command]
fn get_svg_strokes(state: State<AppStateHandle>, kanji_id: i32) -> Option<Vec<Vec<[f32; 2]>>> {
    let mut st = state.lock().unwrap();
    st.svg_cache.get_or_load(kanji_id).map(|strokes| {
        strokes
            .iter()
            .map(|stroke| {
                stroke
                    .points
                    .iter()
                    .map(|sp| [sp.pos.x as f32, sp.pos.y as f32])
                    .collect()
            })
            .collect()
    })
}

/// Receives user-drawn strokes from the canvas and returns the top 8 matching kanji.
/// Input strokes can use any coordinate scale — RecognitionSystem normalises them.
#[tauri::command]
fn search_by_strokes(
    state: State<AppStateHandle>,
    strokes: Vec<Vec<[f32; 2]>>,
) -> Vec<KanjiDto> {
    let st = state.lock().unwrap();
    let user_strokes: Vec<Vec<(f32, f32)>> = strokes
        .iter()
        .map(|s| s.iter().map(|p| (p[0], p[1])).collect())
        .collect();

    let results = st.recognition.search(&user_strokes, 8);
    let by_id: HashMap<i32, &Arc<Kanji>> = st.kanji.iter().map(|k| (k.id, k)).collect();

    results
        .iter()
        .filter_map(|(id, _score)| {
            by_id.get(id).map(|k| {
                let meaning = st.kanji_meanings.get(&k.kanji).cloned();
                to_dto(k, meaning)
            })
        })
        .collect()
}

/// Whether the stroke-recognition templates have finished loading in the
/// background. The draw-search UI uses this to show a loading state instead of
/// reporting "no matches" while templates are still being parsed at startup.
#[tauri::command]
fn is_recognition_ready(state: State<AppStateHandle>) -> bool {
    state.lock().unwrap().recognition.is_loaded()
}

#[tauri::command]
fn get_settings(state: State<AppStateHandle>) -> Config {
    state.lock().unwrap().config.clone()
}

#[tauri::command]
fn save_settings(state: State<AppStateHandle>, config: Config) -> Result<(), String> {
    let path = get_config_path().ok_or_else(|| "Cannot determine config path".to_string())?;
    save_config(&path, &config).map_err(|e| e.to_string())?;
    state.lock().unwrap().config = config;
    Ok(())
}

/// Reload meanings from a user-specified JSON localization file.
#[tauri::command]
fn reload_meanings(state: State<AppStateHandle>, path: String) -> Result<usize, String> {
    let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let tf: TranslationFile = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    let meanings: HashMap<String, String> = tf
        .entries
        .into_iter()
        .map(|(k, v)| (k, v.meaning))
        .collect();
    let count = meanings.len();
    state.lock().unwrap().kanji_meanings = meanings;
    Ok(count)
}

/// Loads the full translation file (meaning + examples per kanji) for the
/// Translate Kanji editor. Returns an empty file if it doesn't exist yet.
#[tauri::command]
fn get_translations(path: String) -> TranslationFile {
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|c| serde_json::from_str::<TranslationFile>(&c).ok())
        .unwrap_or_default()
}

/// Writes the full translation file to disk and refreshes the in-memory
/// meanings cache so get_kanji_list/get_kanji_by_char reflect the edit.
#[tauri::command]
fn save_translations(
    state: State<AppStateHandle>,
    path: String,
    data: TranslationFile,
) -> Result<(), String> {
    if let Some(parent) = std::path::Path::new(&path).parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;

    let meanings: HashMap<String, String> = data
        .entries
        .into_iter()
        .map(|(k, v)| (k, v.meaning))
        .collect();
    state.lock().unwrap().kanji_meanings = meanings;
    Ok(())
}

/// Lists the interface languages bundled with the app (one per
/// `<Name>.toml` file in the localization resources).
#[tauri::command]
fn list_languages(app: AppHandle) -> Vec<String> {
    back::localization::list_available_languages(&app).unwrap_or_default()
}

/// Loads `<lang>.toml`'s strings as a flat "section.key" -> text map.
#[tauri::command]
fn get_localization(app: AppHandle, lang: String) -> HashMap<String, String> {
    back::localization::load_language(&app, &lang).unwrap_or_default()
}

// ── Startup ──────────────────────────────────────────────────────────────────

fn build_app_state() -> AppState {
    let config = load_config().unwrap_or_default();

    let db = Database::new(&config.path_to_db_core);
    let kanji = db.get_kanji().unwrap_or_default();

    let kanji_by_char: HashMap<String, Arc<Kanji>> = kanji
        .iter()
        .map(|k| (k.kanji.clone(), Arc::clone(k)))
        .collect();

    let mut svg_cache = SvgCache::new();
    svg_cache.prepare(&kanji, &config.path_to_svg_images);

    // Recognition templates are parsed from 6700+ SVGs, which is too slow to do
    // on the startup critical path. Start empty and fill it from a background
    // thread in `setup` so the window can appear immediately.
    let recognition = RecognitionSystem::new();

    let kanji_meanings: HashMap<String, String> =
        if std::path::Path::new(&config.path_to_kanji_localization).exists() {
            std::fs::read_to_string(&config.path_to_kanji_localization)
                .ok()
                .and_then(|c| serde_json::from_str::<TranslationFile>(&c).ok())
                .map(|tf| tf.entries.into_iter().map(|(k, v)| (k, v.meaning)).collect())
                .unwrap_or_default()
        } else {
            HashMap::new()
        };

    AppState {
        kanji,
        kanji_by_char,
        svg_cache,
        recognition,
        config,
        kanji_meanings,
    }
}

// ── Entry point ──────────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = build_app_state();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(Mutex::new(state))
        .setup(|app| {
            let handle = app.handle().clone();

            // Snapshot the inputs the recognition cache needs (Arc clones + a
            // path string — cheap) so the background thread doesn't hold the
            // state lock while parsing SVGs.
            let (kanji, svg_path) = {
                let st = handle.state::<AppStateHandle>();
                let st = st.lock().unwrap();
                (st.kanji.clone(), st.config.path_to_svg_images.clone())
            };

            std::thread::spawn(move || {
                let cache_path =
                    back::config::get_app_config_dir().join("recognition_cache.bin");
                let mut recognition = RecognitionSystem::new();
                recognition.load_or_build(&kanji, &svg_path, &cache_path);

                {
                    let st = handle.state::<AppStateHandle>();
                    let mut st = st.lock().unwrap();
                    st.recognition = recognition;
                }

                // Let the UI enable draw-search once templates are ready.
                let _ = handle.emit("recognition-ready", ());
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_kanji_list,
            get_kanji_by_char,
            convert_romaji,
            get_svg_strokes,
            search_by_strokes,
            is_recognition_ready,
            get_settings,
            save_settings,
            reload_meanings,
            get_translations,
            save_translations,
            list_languages,
            get_localization,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
