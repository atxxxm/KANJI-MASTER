export interface KanjiDto {
  id: number;
  kanji: string;
  strokes: number;
  jlpt: string;
  grade: string;
  frequency: string;
  unicode: string;
  onyomi: string;
  onyomi_romaji: string;
  kunyomi: string;
  kunyomi_romaji: string;
  examples: string[];
  meaning: string | null;
}

export interface KanjiTranslation {
  meaning: string;
  translate_examples: string[];
}

export interface TranslationFile {
  last_id: number;
  entries: Record<string, KanjiTranslation>;
}

export interface SrsSettings {
  levels: string[];
  new_per_day: number;
}

export interface SrsSummary {
  due_count: number;
  new_remaining_today: number;
  new_available: number;
  total_cards: number;
  mature_count: number;
  settings: SrsSettings;
}

export interface ReviewCard {
  kanji: KanjiDto;
  is_new: boolean;
}

export interface WordDto {
  id: number;
  kanji: string | null;
  reading: string;
  gloss_en: string | null;
  gloss_ru: string | null;
  rank: number;
}

export interface Config {
  interface_font_size: number;
  kanji_font_size: number;
  animation_speed: number;
  path_to_db_core: string;
  path_to_kanji_localization: string;
  path_to_svg_images: string;
  show_kanji_meaning: boolean;
  focus_on_search: boolean;
  dark_mode: boolean;
  interface_language: string;
  onboarding_seen: boolean;
}
