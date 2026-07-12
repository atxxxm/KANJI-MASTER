// Convert romaji to kana
pub fn to_kana(input: &str, is_katakana: bool, live_input: bool) -> String {
    let mut results = String::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Attempt to find a match with a length of 4 (e.g. "xtsu")
        if i + 4 <= chars.len() {
            let key: String = chars[i..i + 4].iter().collect();
            if let Some(kana) = get_kana(&key, is_katakana) {
                results.push_str(kana);
                i += 4;
                continue;
            }
        }

        // Attempt to find a match with a length of 3 (e.g "shi", "chi", "tsu")
        if i + 3 <= chars.len() {
            let key: String = chars[i..i + 3].iter().collect();
            if let Some(kana) = get_kana(&key, is_katakana) {
                results.push_str(kana);
                i += 3;
                continue;
            }
        }

        // Attempt to find a match with a length of 2 (e.g "ka", "ta", "sa")
        if i + 2 <= chars.len() {
            let key: String = chars[i..i + 2].iter().collect();
            if let Some(kana) = get_kana(&key, is_katakana) {
                results.push_str(kana);
                i += 2;
                continue;
            }
        }

        // Processing "n" (special case)
        // n' (apostrophe) forces ん regardless of what follows
        // n before a consonant (except y) or at end of word -> ん
        if chars[i] == 'n' {
            let is_end = i + 1 >= chars.len();

            if live_input && is_end {
                results.push('n');
                i += 1;
                continue;
            }

            // n' -> ん, consume the apostrophe
            if !is_end && chars[i + 1] == '\'' {
                results.push_str(if is_katakana { "ン" } else { "ん" });
                i += 2;
                continue;
            }

            let next_is_consonant = !is_end && !is_vowel(chars[i + 1]) && chars[i + 1] != 'y';

            if is_end || next_is_consonant {
                results.push_str(if is_katakana { "ン" } else { "ん" });
                i += 1;
                continue;
            }
        }

        // Processing double consonants (Sokuon: tt, kk, pp -> っ)
        // If the current letter matches the next and is a consonant -> っ
        if i + 1 < chars.len()
            && chars[i] == chars[i + 1]
            && !is_vowel(chars[i])
            && chars[i].is_ascii_alphabetic()
        {
            results.push_str(if is_katakana { "ッ" } else { "っ" });
            i += 1;
            continue;
        }

        // Attempt to find a match of length 1 (vowels a, i, u, e, o)
        let key: String = chars[i].to_string();
        if let Some(kana) = get_kana(&key, is_katakana) {
            results.push_str(kana);
            i += 1;
            continue;
        }

        // Processing "ー"
        if chars[i] == '-' {
            results.push('ー');
            i += 1;
            continue;
        }

        // Processing "。"
        if chars[i] == '.' {
            results.push('。');
            i += 1;
            continue;
        }

        // Processing "、"
        if chars[i] == ',' {
            results.push('、');
            i += 1;
            continue;
        }

        // If no match is found, just add the character as is
        // This allows mixing Kanji/Kana/Romaji in the input
        results.push(chars[i]);
        i += 1;
    }

    results
}

// Helper functions
fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'i' | 'u' | 'e' | 'o')
}

// Katakana-only foreign syllable combinations (no standard hiragana equivalent)
fn get_katakana_foreign(romaji: &str) -> Option<&'static str> {
    match romaji.to_lowercase().as_str() {
        // F-row
        "fa" => Some("ファ"), "fi" => Some("フィ"), "fe" => Some("フェ"), "fo" => Some("フォ"),
        // V-row (vu is in main table as ゔ→ヴ)
        "va" => Some("ヴァ"), "vi" => Some("ヴィ"), "ve" => Some("ヴェ"), "vo" => Some("ヴォ"),
        // W-row
        "wi" => Some("ウィ"), "we" => Some("ウェ"),
        // T/D foreign combos
        "thi" => Some("ティ"), "thu" => Some("テュ"), "dhi" => Some("ディ"), "dhu" => Some("デュ"),
        // Ts-row
        "tsa" => Some("ツァ"), "tsi" => Some("ツィ"), "tse" => Some("ツェ"), "tso" => Some("ツォ"),
        _ => None,
    }
}

// Database of syllables
fn get_kana(romaji: &str, is_katakana: bool) -> Option<&'static str> {
    // Check katakana-only foreign combos first
    if is_katakana {
        if let Some(kata) = get_katakana_foreign(romaji) {
            return Some(kata);
        }
    }

    let lowered = romaji.to_lowercase();
    let hiragana = match lowered.as_str() {
        // Vowels
        "a" => "あ", "i" => "い", "u" => "う", "e" => "え", "o" => "お",
        // K-row
        "ka" => "か", "ki" => "き", "ku" => "く", "ke" => "け", "ko" => "こ",
        // S-row
        "sa" => "さ", "shi" => "し", "si" => "し", "su" => "す", "se" => "せ", "so" => "そ",
        // T-row
        "ta" => "た", "chi" => "ち", "ti" => "ち", "tsu" => "つ", "tu" => "つ", "te" => "て", "to" => "と",
        // N-row
        "na" => "な", "ni" => "に", "nu" => "ぬ", "ne" => "ね", "no" => "の",
        // H-row
        "ha" => "は", "hi" => "ひ", "fu" => "ふ", "hu" => "ふ", "he" => "へ", "ho" => "ほ",
        // M-row
        "ma" => "ま", "mi" => "み", "mu" => "む", "me" => "め", "mo" => "も",
        // Y-row
        "ya" => "や", "yu" => "ゆ", "yo" => "よ",
        // R-row
        "ra" => "ら", "ri" => "り", "ru" => "る", "re" => "れ", "ro" => "ろ",
        // W-row
        "wa" => "わ", "wo" => "を", "nn" => "ん",
        // G-row (voiced K)
        "ga" => "が", "gi" => "ぎ", "gu" => "ぐ", "ge" => "げ", "go" => "ご",
        // Z-row (voiced S)
        "za" => "ざ", "ji" => "じ", "zi" => "じ", "zu" => "ず", "ze" => "ぜ", "zo" => "ぞ",
        // D-row (voiced T)
        "da" => "だ", "di" => "ぢ", "du" => "づ", "de" => "で", "do" => "ど",
        // B-row (voiced H)
        "ba" => "ば", "bi" => "び", "bu" => "ぶ", "be" => "べ", "bo" => "ぼ",
        // P-row (semi-voiced H)
        "pa" => "ぱ", "pi" => "ぴ", "pu" => "ぷ", "pe" => "ぺ", "po" => "ぽ",
        // V (ゔ exists in unicode, rarely used in hiragana)
        "vu" => "ゔ",
        // Compound K
        "kya" => "きゃ", "kyu" => "きゅ", "kyo" => "きょ",
        // Compound S
        "sha" => "しゃ", "shu" => "しゅ", "sho" => "しょ",
        "sya" => "しゃ", "syu" => "しゅ", "syo" => "しょ",
        // Compound CH
        "cha" => "ちゃ", "chu" => "ちゅ", "cho" => "ちょ",
        "tya" => "ちゃ", "tyu" => "ちゅ", "tyo" => "ちょ",
        // Compound N
        "nya" => "にゃ", "nyu" => "にゅ", "nyo" => "にょ",
        // Compound H
        "hya" => "ひゃ", "hyu" => "ひゅ", "hyo" => "ひょ",
        // Compound M
        "mya" => "みゃ", "myu" => "みゅ", "myo" => "みょ",
        // Compound R
        "rya" => "りゃ", "ryu" => "りゅ", "ryo" => "りょ",
        // Compound G
        "gya" => "ぎゃ", "gyu" => "ぎゅ", "gyo" => "ぎょ",
        // Compound J
        "ja" => "じゃ", "ju" => "じゅ", "jo" => "じょ",
        "jya" => "じゃ", "jyu" => "じゅ", "jyo" => "じょ",
        "zya" => "じゃ", "zyu" => "じゅ", "zyo" => "じょ",
        // Compound B
        "bya" => "びゃ", "byu" => "びゅ", "byo" => "びょ",
        // Compound P
        "pya" => "ぴゃ", "pyu" => "ぴゅ", "pyo" => "ぴょ",
        // Compound D (voiced ぢ)
        "dya" => "ぢゃ", "dyu" => "ぢゅ", "dyo" => "ぢょ",
        // Small kana (x-prefix)
        "xa" => "ぁ", "xi" => "ぃ", "xu" => "ぅ", "xe" => "ぇ", "xo" => "ぉ",
        "xya" => "ゃ", "xyu" => "ゅ", "xyo" => "ょ",
        "xtu" => "っ", "xtsu" => "っ",
        "xwa" => "ゎ",
        _ => return None,
    };

    if !is_katakana {
        Some(hiragana)
    } else {
        Some(hiragana_to_katakana(hiragana))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_vowels_and_rows() {
        assert_eq!(to_kana("aiueo", false, false), "あいうえお");
        assert_eq!(to_kana("kakikukeko", false, false), "かきくけこ");
    }

    #[test]
    fn longest_match_wins_over_shorter_prefixes() {
        // "sha" (3 chars) must not be read as "shi" + stray "a".
        assert_eq!(to_kana("sha", false, false), "しゃ");
        // "xtsu" (4 chars) must not fall back to "xtu"-less parsing.
        assert_eq!(to_kana("xtsu", false, false), "っ");
    }

    #[test]
    fn sokuon_doubles_consonant_into_small_tsu() {
        assert_eq!(to_kana("kitte", false, false), "きって");
        assert_eq!(to_kana("gakkou", false, false), "がっこう");
        // A doubled vowel is not sokuon — "aa" is just two あ.
        assert_eq!(to_kana("aa", false, false), "ああ");
    }

    #[test]
    fn n_apostrophe_forces_syllabic_n_before_a_vowel() {
        assert_eq!(to_kana("kon'ya", false, false), "こんや");
        // Without the apostrophe, "nya" reads as the compound にゃ instead.
        assert_eq!(to_kana("konya", false, false), "こにゃ");
    }

    #[test]
    fn n_before_consonant_or_end_of_word_becomes_syllabic_n() {
        assert_eq!(to_kana("kanji", false, false), "かんじ");
        assert_eq!(to_kana("hon", false, false), "ほん");
        // n before a vowel (not end, no apostrophe) attaches to the row instead.
        assert_eq!(to_kana("na", false, false), "な");
    }

    #[test]
    fn trailing_n_stays_literal_while_live_typing() {
        // live_input=true: a trailing "n" might still become "na"/"nya"/etc. on
        // the next keystroke, so it's held back instead of committing to ん.
        assert_eq!(to_kana("ka n", false, true), "か n");
        // Once a following character arrives, it resolves normally.
        assert_eq!(to_kana("kani", false, true), "かに");
    }

    #[test]
    fn long_vowel_and_punctuation_marks() {
        assert_eq!(to_kana("ka-", false, false), "かー");
        assert_eq!(to_kana("ohayou.", false, false), "おはよう。");
        assert_eq!(to_kana("hai,", false, false), "はい、");
    }

    #[test]
    fn katakana_mode_converts_and_supports_foreign_syllables() {
        assert_eq!(to_kana("kohi", true, false), "コヒ");
        // "fa"/"va" only exist as katakana foreign-sound combos.
        assert_eq!(to_kana("fa", true, false), "ファ");
        assert_eq!(to_kana("va", true, false), "ヴァ");
    }

    #[test]
    fn unrecognized_characters_pass_through_unchanged() {
        // Lets kanji/kana/punctuation already in the input survive untouched.
        assert_eq!(to_kana("漢字ka", false, false), "漢字か");
    }

    #[test]
    fn uppercase_romaji_is_case_insensitive() {
        assert_eq!(to_kana("KA", false, false), "か");
        assert_eq!(to_kana("SHA", true, false), "シャ");
    }
}

// Convert hiragana to katakana
fn hiragana_to_katakana(h: &'static str) -> &'static str {
    match h {
        // Vowels
        "あ" => "ア", "い" => "イ", "う" => "ウ", "え" => "エ", "お" => "オ",
        // Small vowels
        "ぁ" => "ァ", "ぃ" => "ィ", "ぅ" => "ゥ", "ぇ" => "ェ", "ぉ" => "ォ",
        // K-row
        "か" => "カ", "き" => "キ", "く" => "ク", "け" => "ケ", "こ" => "コ",
        // S-row
        "さ" => "サ", "し" => "シ", "す" => "ス", "せ" => "セ", "そ" => "ソ",
        // T-row
        "た" => "タ", "ち" => "チ", "つ" => "ツ", "て" => "テ", "と" => "ト",
        // N-row
        "な" => "ナ", "に" => "ニ", "ぬ" => "ヌ", "ね" => "ネ", "の" => "ノ",
        // H-row
        "は" => "ハ", "ひ" => "ヒ", "ふ" => "フ", "へ" => "ヘ", "ほ" => "ホ",
        // M-row
        "ま" => "マ", "み" => "ミ", "む" => "ム", "め" => "メ", "も" => "モ",
        // Y-row
        "や" => "ヤ", "ゆ" => "ユ", "よ" => "ヨ",
        // Small Y
        "ゃ" => "ャ", "ゅ" => "ュ", "ょ" => "ョ",
        // R-row
        "ら" => "ラ", "り" => "リ", "る" => "ル", "れ" => "レ", "ろ" => "ロ",
        // W-row
        "わ" => "ワ", "ゎ" => "ヮ", "を" => "ヲ", "ん" => "ン",
        // V
        "ゔ" => "ヴ",
        // G-row
        "が" => "ガ", "ぎ" => "ギ", "ぐ" => "グ", "げ" => "ゲ", "ご" => "ゴ",
        // Z-row
        "ざ" => "ザ", "じ" => "ジ", "ず" => "ズ", "ぜ" => "ゼ", "ぞ" => "ゾ",
        // D-row
        "だ" => "ダ", "ぢ" => "ヂ", "づ" => "ヅ", "で" => "デ", "ど" => "ド",
        // B-row
        "ば" => "バ", "び" => "ビ", "ぶ" => "ブ", "べ" => "ベ", "ぼ" => "ボ",
        // P-row
        "ぱ" => "パ", "ぴ" => "ピ", "ぷ" => "プ", "ぺ" => "ペ", "ぽ" => "ポ",
        // Sokuon
        "っ" => "ッ",
        // Compound K
        "きゃ" => "キャ", "きゅ" => "キュ", "きょ" => "キョ",
        // Compound S
        "しゃ" => "シャ", "しゅ" => "シュ", "しょ" => "ショ",
        // Compound CH
        "ちゃ" => "チャ", "ちゅ" => "チュ", "ちょ" => "チョ",
        // Compound N
        "にゃ" => "ニャ", "にゅ" => "ニュ", "にょ" => "ニョ",
        // Compound H
        "ひゃ" => "ヒャ", "ひゅ" => "ヒュ", "ひょ" => "ヒョ",
        // Compound M
        "みゃ" => "ミャ", "みゅ" => "ミュ", "みょ" => "ミョ",
        // Compound R
        "りゃ" => "リャ", "りゅ" => "リュ", "りょ" => "リョ",
        // Compound G
        "ぎゃ" => "ギャ", "ぎゅ" => "ギュ", "ぎょ" => "ギョ",
        // Compound J
        "じゃ" => "ジャ", "じゅ" => "ジュ", "じょ" => "ジョ",
        // Compound B
        "びゃ" => "ビャ", "びゅ" => "ビュ", "びょ" => "ビョ",
        // Compound P
        "ぴゃ" => "ピャ", "ぴゅ" => "ピュ", "ぴょ" => "ピョ",
        // Compound D
        "ぢゃ" => "ヂャ", "ぢゅ" => "ヂュ", "ぢょ" => "ヂョ",
        _ => h,
    }
}
