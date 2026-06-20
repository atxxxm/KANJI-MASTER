// Convert romaji to kana
pub fn to_kana(input: &str, is_katakana: bool, live_input: bool) -> String {
    let mut results = String::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
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

        // Prossesing "n" (special case)
        // If "n" comes before a constant (expect y) or at the end of a word -> ん
        if chars[i] == 'n' {
            let is_end = i + 1 >= chars.len();
            
            if live_input && is_end {
                results.push('n');
                i += 1;
                continue;
            }

            let next_is_consonant = !is_end && !is_vowel(chars[i + 1]) && chars[i + 1] != 'y';

            if is_end || next_is_consonant {
                results.push_str(if is_katakana { "ン" } else { "ん" });
                i += 1;
                continue;
            }
        }

        // Processing double consonants (Sokun: tt, kk, pp -> っ)
        // If the current letter matches the next once addd it is a constant
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

// Database of syllables
fn get_kana(romaji: &str, is_katakana: bool) -> Option<&'static str> {
    let lowered = romaji.to_lowercase();
    let hiragana = match lowered.as_str() {
        "a" => "あ", "i" => "い", "u" => "う", "e" => "え", "o" => "お",
        "ka" => "か", "ki" => "き", "ku" => "く", "ke" => "け", "ko" => "こ",
        "sa" => "さ", "shi" => "し", "si" => "し", "su" => "す", "se" => "せ", "so" => "そ",
        "ta" => "た", "chi" => "ち", "ti" => "ち", "tsu" => "つ", "tu" => "つ", "te" => "て", "to" => "と",
        "na" => "な", "ni" => "に", "nu" => "ぬ", "ne" => "ね", "no" => "の",
        "ha" => "は", "hi" => "ひ", "fu" => "ふ", "hu" => "ふ", "he" => "へ", "ho" => "ほ",
        "ma" => "ま", "mi" => "み", "mu" => "む", "me" => "め", "mo" => "も",
        "ya" => "や", "yu" => "ゆ", "yo" => "よ",
        "ra" => "ら", "ri" => "り", "ru" => "る", "re" => "れ", "ro" => "ろ",
        "wa" => "わ", "wo" => "を", "nn" => "ん",
        "ga" => "が", "gi" => "ぎ", "gu" => "ぐ", "ge" => "げ", "go" => "ご",
        "za" => "ざ", "ji" => "じ", "zi" => "じ", "zu" => "ず", "ze" => "ぜ", "zo" => "ぞ",
        "da" => "だ", "di" => "ぢ", "du" => "づ", "de" => "で", "do" => "ど",
        "ba" => "ば", "bi" => "び", "bu" => "ぶ", "be" => "べ", "bo" => "ぼ",
        "pa" => "ぱ", "pi" => "ぴ", "pu" => "ぷ", "pe" => "ぺ", "po" => "ぽ",
        "kya" => "きゃ", "kyu" => "きゅ", "kyo" => "きょ",
        "sha" => "しゃ", "shu" => "しゅ", "sho" => "しょ",
        "cha" => "ちゃ", "chu" => "ちゅ", "cho" => "ちょ",
        "nya" => "にゃ", "nyu" => "にゅ", "nyo" => "にょ",
        "hya" => "ひゃ", "hyu" => "ひゅ", "hyo" => "ひょ",
        "mya" => "みゃ", "myu" => "みゅ", "myo" => "みょ",
        "rya" => "りゃ", "ryu" => "りゅ", "ryo" => "りょ",
        "gya" => "ぎゃ", "gyu" => "ぎゅ", "gyo" => "ぎょ",
        "ja" => "じゃ", "ju" => "じゅ", "jo" => "じょ", "jya" => "じゃ", "jyu" => "じゅ", "jyo" => "じょ",
        "bya" => "びゃ", "byu" => "びゅ", "byo" => "びょ",
        "pya" => "ぴゃ", "pyu" => "ぴゅ", "pyo" => "ぴょ",
        _ => return None,
    };

    if !is_katakana {
        Some(hiragana)
    } else {
        Some(hiragana_to_katakana(hiragana))
    }
}

// Convert hiragana to katakana
fn hiragana_to_katakana(h: &'static str) -> &'static str {
     match h {
        "あ" => "ア", "い" => "イ", "う" => "ウ", "え" => "エ", "お" => "オ",
        "か" => "カ", "き" => "キ", "く" => "ク", "け" => "ケ", "こ" => "コ",
        "さ" => "サ", "し" => "シ", "す" => "ス", "せ" => "セ", "そ" => "ソ",
        "た" => "タ", "ち" => "チ", "つ" => "ツ", "て" => "テ", "と" => "ト",
        "な" => "ナ", "に" => "ニ", "ぬ" => "ヌ", "ね" => "ネ", "の" => "ノ",
        "は" => "ハ", "ひ" => "ヒ", "ふ" => "フ", "へ" => "ヘ", "ほ" => "ホ",
        "ま" => "マ", "み" => "ミ", "む" => "ム", "め" => "メ", "も" => "モ",
        "や" => "ヤ", "ゆ" => "ユ", "よ" => "ヨ",
        "ら" => "ラ", "り" => "リ", "る" => "ル", "れ" => "レ", "ろ" => "ロ",
        "わ" => "ワ", "を" => "ヲ", "ん" => "ン",
        "が" => "ガ", "ぎ" => "ギ", "ぐ" => "グ", "げ" => "ゲ", "ご" => "ゴ",
        "ざ" => "ザ", "じ" => "ジ", "ず" => "ズ", "ぜ" => "ゼ", "ぞ" => "ゾ",
        "だ" => "ダ", "ぢ" => "ヂ", "づ" => "ヅ", "で" => "デ", "ど" => "ド",
        "ば" => "バ", "び" => "ビ", "ぶ" => "ブ", "べ" => "ベ", "ぼ" => "ボ",
        "ぱ" => "パ", "ぴ" => "ピ", "ぷ" => "プ", "ぺ" => "ペ", "ぽ" => "ポ",
        "きゃ" => "キャ", "きゅ" => "キュ", "きょ" => "キョ",
        "しゃ" => "シャ", "しゅ" => "シュ", "しょ" => "ショ",
        "ちゃ" => "チャ", "ちゅ" => "チュ", "ちょ" => "チョ",
        "にゃ" => "ニャ", "にゅ" => "ニュ", "にょ" => "ニョ",
        "ひゃ" => "ヒャ", "ひゅ" => "ヒュ", "ひょ" => "ヒョ",
        "みゃ" => "ミャ", "みゅ" => "ミュ", "みょ" => "ミョ",
        "りゃ" => "リャ", "りゅ" => "リュ", "りょ" => "リョ",
        "ぎゃ" => "ギャ", "ぎゅ" => "ギュ", "ぎょ" => "ギョ",
        "じゃ" => "ジャ", "じゅ" => "ジュ", "じょ" => "ジョ",
        "びゃ" => "ビャ", "byu" => "ビュ", "びょ" => "ビョ",
        "びゅ" => "ビュ",
        "ぴゃ" => "ピャ", "ぴゅ" => "ピュ", "ぴょ" => "ピョ",
        _ => h,
    }
}
