use crate::options::Options;
pub(crate) use crate::to_ukrainian_node_tree::TO_UKRAINIAN_NODE_TREE;
use crate::utils::is_char_katakana::is_char_katakana;
use crate::utils::katakana_to_hiragana::*;

/// Convert kana to romaji
pub fn to_ukrainian(input: &str) -> String {
    to_ukrainian_with_opt(input, Options::default())
}

static VOWEL_ENDINGS: &[char] = &[
    // 'a' ending Hiraganas and Katanakas
    'あ', 'か', 'さ', 'た', 'な', 'は', 'ま', 'や', 'ら', 'わ', 'が', 'ざ', 'だ', 'ば', 'ぱ',
    'ア', 'カ', 'サ', 'タ', 'ナ', 'ハ', 'マ', 'ヤ', 'ラ', 'ワ', 'ガ', 'ザ', 'ダ', 'バ', 'パ',

    // 'i' ending Hiraganas and Katanakas
    'い', 'き', 'し', 'ち', 'に', 'ひ', 'み', 'り', 'ぎ', 'じ', 'ぢ', 'び', 'ぴ',
    'イ', 'キ', 'シ', 'チ', 'ニ', 'ヒ', 'ミ', 'リ', 'ギ', 'ジ', 'ヂ', 'ビ', 'ピ',

    // 'u' ending Hiraganas and Katanakas
    'う', 'く', 'す', 'つ', 'ぬ', 'ふ', 'む', 'ゆ', 'る', 'ぐ', 'ず', 'づ', 'ぶ', 'ぷ',
    'ウ', 'ク', 'ス', 'ツ', 'ヌ', 'フ', 'ム', 'ユ', 'ル', 'グ', 'ズ', 'ヅ', 'ブ', 'プ',

    // 'e' ending Hiraganas and Katanakas
    'え', 'け', 'せ', 'て', 'ね', 'へ', 'め', 'れ', 'げ', 'ぜ', 'で', 'べ', 'ぺ',
    'エ', 'ケ', 'セ', 'テ', 'ネ', 'ヘ', 'メ', 'レ', 'ゲ', 'ゼ', 'デ', 'ベ', 'ペ',

    // 'o' ending Hiraganas and Katanakas
    'お', 'こ', 'そ', 'と', 'の', 'ほ', 'も', 'よ', 'ろ', 'を', 'ご', 'ぞ', 'ど', 'ぼ', 'ぽ',
    'オ', 'コ', 'ソ', 'ト', 'ノ', 'ホ', 'モ', 'ヨ', 'ロ', 'ヲ', 'ゴ', 'ゾ', 'ド', 'ボ', 'ポ',
];

static I_ENDINGS: &[char] = &[
    // 'i' ending Hiraganas and Katanakas
    'い', 'き', 'し', 'ち', 'に', 'ひ', 'み', 'り', 'ぎ', 'じ', 'ぢ', 'び', 'ぴ',
    'イ', 'キ', 'シ', 'チ', 'ニ', 'ヒ', 'ミ', 'リ', 'ギ', 'ジ', 'ヂ', 'ビ', 'ピ',
];

static CONSONANT_BEGINNINGS: &[char] = &[
    // Ka, Ki, Ku, Ke, Ko series
    'か', 'き', 'く', 'け', 'こ',
    'カ', 'キ', 'ク', 'ケ', 'コ',
    'が', 'ぎ', 'ぐ', 'げ', 'ご',
    'ガ', 'ギ', 'グ', 'ゲ', 'ゴ',

    // Sa, Shi, Su, Se, So series
    'さ', 'し', 'す', 'せ', 'そ',
    'サ', 'シ', 'ス', 'セ', 'ソ',
    'ざ', 'じ', 'ず', 'ぜ', 'ぞ',
    'ザ', 'ジ', 'ズ', 'ゼ', 'ゾ',

    // Ta, Chi, Tsu, Te, To series
    'た', 'ち', 'つ', 'て', 'と',
    'タ', 'チ', 'ツ', 'テ', 'ト',
    'だ', 'ぢ', 'づ', 'で', 'ど',
    'ダ', 'ヂ', 'ヅ', 'デ', 'ド',

    // Na, Ni, Nu, Ne, No series
    'な', 'に', 'ぬ', 'ね', 'の',
    'ナ', 'ニ', 'ヌ', 'ネ', 'ノ',

    // Ha, Hi, Fu, He, Ho series
    'は', 'ひ', 'ふ', 'へ', 'ほ',
    'ハ', 'ヒ', 'フ', 'ヘ', 'ホ',
    'ば', 'び', 'ぶ', 'べ', 'ぼ',
    'バ', 'ビ', 'ブ', 'ベ', 'ボ',
    'ぱ', 'ぴ', 'ぷ', 'ぺ', 'ぽ',
    'パ', 'ピ', 'プ', 'ペ', 'ポ',

    // Ma, Mi, Mu, Me, Mo series
    'ま', 'み', 'む', 'め', 'も',
    'マ', 'ミ', 'ム', 'メ', 'モ',

    // Ya, Yu, Yo series
    'や',       'ゆ',       'よ',
    'ヤ',       'ユ',       'ヨ',

    // Ra, Ri, Ru, Re, Ro series
    'ら', 'り', 'る', 'れ', 'ろ',
    'ラ', 'リ', 'ル', 'レ', 'ロ',

    // Wa, Wo series
    'わ', 'を',
    'ワ', 'ヲ',

    // N (ん/ン) can be considered as well
    'ん',
    'ン',
];

// static VOWELS: &[char] = &['а', 'е', 'і', 'о', 'у'];
// static CONSONATS: &[char] = &['к', 'с', 'ш', 'т', 'ч', 'н', 'х', 'ф', 'м', 'р', 'в', 'ґ', 'б', 'п', 'я'];

/// Convert kana to romaji
pub fn to_ukrainian_with_opt(orig: &str, options: Options) -> String {
    let mut result = String::new();
    orig.to_owned().push(' ');
    let mut chars = orig.chars().peekable();

    let mut last_char = 'ä';

    while let Some(c) = chars.next() {


        // vowel + _ + い = "аї", "ії", "уї", "еї", "ої"
        // い + _ + consonant/whitespace/none = "ай", "ій", "уй", "ей", "ой"
        if VOWEL_ENDINGS.contains(&last_char) 
            && c == '_' 
            && chars.next_if(|x| x == &'い' || x == &'イ').is_some() 
        {
            if chars.next_if(|x| x == &'え' || x == &'エ').is_some() {
                result.push_str("їє")
            } else {
                result.push('ї')
            }
        } else if VOWEL_ENDINGS.contains(&last_char)
            && (c == 'い' || c == 'イ')
        {
            result.push('й')
        } else {
            result.push(c);
        }

        last_char = c;
    }

    let result = result.replace('_', "");


    let kana = katakana_to_hiragana_with_opt(&result, true);
    let orig_chars = result.chars().collect::<Vec<_>>();
    let chars = kana.chars().collect::<Vec<_>>();
    let mut ouput = String::with_capacity(orig.len());
    let len = chars.len();
    // Position in the string that is being evaluated
    let mut curr_pos = 0;

    while curr_pos != len {
        let result = TO_UKRAINIAN_NODE_TREE.get(&chars[curr_pos..]);
        // nothing found, pass through
        if result.1 == 0 {
            ouput.push(chars[curr_pos]);
            curr_pos += 1;
        } else {
            let convert_romaji_to_uppercase = {
                if orig_chars[curr_pos..curr_pos + result.1]
                    .iter()
                    .all(|c| is_char_katakana(*c))
                {
                    options.upcase_katakana
                } else {
                    false
                }
            };

            if convert_romaji_to_uppercase {
                ouput.push_str(&result.0.to_uppercase());
            } else {
                ouput.push_str(result.0);
            }
            curr_pos += result.1;
        }
    }
    ouput.replace("іе", "іє")
}

#[cfg(test)]
mod tests {
    use super::{Options, *};
    #[test]
    fn sane_defaults() {
        assert_eq!(to_ukrainian(""), "");
    }

    #[test]
    fn convert_katakana_to_ukrainian() {
        assert_eq!(
            to_ukrainian("ワニカニ　ガ　スゴイ　ダ"),
            "ванікані ґа суґоі да"
        );
    }

    #[test]
    fn convert_hiragana_to_ukrainian() {
        assert_eq!(
            to_ukrainian("わにかに　が　すごい　だ"),
            "ванікані ґа суґоі да"
        );
    }

    #[test]
    fn convert_mixed_kana_to_ukrainian() {
        assert_eq!(
            to_ukrainian("ワニカニ　が　すごい　だ"),
            "ванікані ґа суґоі да"
        );
    }

    //#[test]
    // fn will_convert_punctuation_and_full_width_spaces() {
    // assert_eq!(to_ukrainian(JA_PUNC.join("")), EN_PUNC.join(""));
    //}

    #[test]
    fn use_the_upcase_katakana_flag_to_preserve_casing_works_for_katakana() {
        assert_eq!(
            to_ukrainian_with_opt(
                "ワニカニ",
                Options {
                    upcase_katakana: true,
                    ..Default::default()
                }
            ),
            "ВАНІКАНІ"
        );
    }

    #[test]
    fn use_the_upcase_katakana_flag_to_preserve_casing_works_for_mixed_kana() {
        assert_eq!(
            to_ukrainian_with_opt(
                "ワニカニ　が　すごい　だ",
                Options {
                    upcase_katakana: true,
                    ..Default::default()
                }
            ),
            "ВАНІКАНІ ґа суґоі да"
        );
    }

    #[test]
    fn converts_long_dash_in_hiragana_to_hyphen() {
        assert_eq!(to_ukrainian("ばつげーむ"), "бацуґе-му");
    }

    #[test]
    fn doesnt_confuse_一one_kanji_for_long_dash_ー() {
        assert_eq!(to_ukrainian("一抹げーむ"), "一抹ґе-му");
    }

    #[test]
    fn converts_long_dash_ー_chōonpu_in_katakana_to_long_vowel() {
        assert_eq!(to_ukrainian("スーパー"), "суупаа"); //суупаа
    }

    #[test]
    fn doesnt_convert_オー_to_ou_which_occurs_with_hiragana() {
        assert_eq!(to_ukrainian("缶コーヒー"), "缶коохіі");
    }

    #[test]
    fn spaces_must_be_manually_entered() {
        assert_ne!(to_ukrainian("わにかにがすごいだ"), "ванікані ґа суґоі да");
    }

    mod double_ns_and_double_consonants {
        use super::*;

        #[test]
        fn double_and_single_n() {
            assert_eq!(to_ukrainian("きんにくまん"), "кіннікуман");
        }
        #[test]
        fn n_extravaganza() {
            assert_eq!(to_ukrainian("んんにんにんにゃんやん"), "нннінніннян'ян");
        }
        #[test]
        fn double_consonants() {
            assert_eq!(
                to_ukrainian("かっぱ　たった　しゅっしゅ ちゃっちゃ　やっつ"),
                "каппа татта шюшшю чяччя яццу"
            );
        }
        #[test]
        fn xx_double_consonants() {
            assert_eq!(to_ukrainian("かっぱ"), "каппа");
        }
    }

    mod small_kana {
        use super::*;

        #[test]
        fn small_tsu_doesnt_transliterate() {
            assert_eq!(to_ukrainian("っ"), "");
        }
        #[test]
        fn small_kata_ke_doesnt_transliterate() {
            assert_eq!(to_ukrainian("ヶ"), "ヶ");
        }
        #[test]
        fn small_kata_ka_doesnt_transliterate() {
            assert_eq!(to_ukrainian("ヵ"), "ヵ");
        }
        #[test]
        fn small_ya() {
            assert_eq!(to_ukrainian("ゃ"), "я");
        }
        #[test]
        fn small_yu() {
            assert_eq!(to_ukrainian("ゅ"), "ю");
        }
        #[test]
        fn small_yo() {
            assert_eq!(to_ukrainian("ょ"), "йо");
        }
        #[test]
        fn small_a() {
            assert_eq!(to_ukrainian("ぁ"), "а");
        }
        #[test]
        fn small_i() {
            assert_eq!(to_ukrainian("ぃ"), "і");
        }
        #[test]
        fn small_u() {
            assert_eq!(to_ukrainian("ぅ"), "у");
        }
        #[test]
        fn small_e() {
            assert_eq!(to_ukrainian("ぇ"), "е");
        }
        #[test]
        fn small_o() {
            assert_eq!(to_ukrainian("ぉ"), "о");
        }
    }

    mod apostrophes_in_ambiguous_consonant_vowel_combos {
        use super::*;

        #[test]
        fn おんよみ() {
            assert_eq!(to_ukrainian("おんよみ"), "онйомі");
        }
        #[test]
        fn んよ_んあ_んゆ() {
            assert_eq!(to_ukrainian("んよ んあ んゆ"), "нйо n'a н'ю");
        }
        #[test]
        fn シンヨ() {
            assert_eq!(to_ukrainian("シンヨ"), "шінйо");
        }
    }
}
