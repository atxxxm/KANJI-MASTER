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

export interface Config {
  interface_font_size: number;
  kanji_font_size: number;
  animation_speed: number;
  path_to_db_core: string;
  path_to_localization: string;
  path_to_kanji_localization: string;
  path_to_svg_images: string;
  show_kanji_meaning: boolean;
  focus_on_search: boolean;
  dark_mode: boolean;
}
