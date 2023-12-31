use jpreprocess_core::pronunciation::phoneme::{Consonant, Vowel};
use jpreprocess_window::{IterQuintMut, Triple};

use crate::arpabet::*;

pub enum JaPhoneme {
    Consonant(Consonant),
    Vowel(Vowel),
    Space,
    None,
}

pub fn to_ja(phonemes: &[EnPhoneme]) -> String {
    let mut phonemes = to_ja_phonemes(phonemes);
    ja_to_ja(&mut phonemes);
    add_vowel(&mut phonemes);

    let mut prevc = None;
    let mut string = String::new();
    for curr in phonemes {
        match curr {
            JaPhoneme::Consonant(Consonant::Nn) => string.push_str("ン"),
            JaPhoneme::Consonant(Consonant::Long) => string.push_str("ー"),
            JaPhoneme::Consonant(consonant) => prevc = Some(consonant),
            JaPhoneme::Vowel(vowel) => {
                if let Some(consonant) = prevc {
                    string.push_str(to_mora(&consonant, &vowel));
                    prevc = None;
                } else {
                    string.push_str(vowel_to_string(&vowel));
                }
            }
            JaPhoneme::Space => string.push_str(" "),

            JaPhoneme::None => unreachable!(),
        }
    }

    string
}

pub fn to_ja_phonemes(phonemes: &[EnPhoneme]) -> Vec<JaPhoneme> {
    let mut result = vec![];

    let mut prev = None;

    for idx in 0..phonemes.len() {
        let send_to_next = |prev: &mut Option<EnConsonant>| {
            *prev = match &phonemes[idx] {
                EnPhoneme::Consonant(consonant) => Some(consonant.to_owned()),
                _ => None,
            }
        };

        match (&prev, &phonemes[idx]) {
            (None, EnPhoneme::Vowel(vowel, _)) => {
                result.push(JaPhoneme::Vowel(katakana_vowel(vowel)))
            }
            (None, EnPhoneme::Consonant(_)) => send_to_next(&mut prev),

            (_, EnPhoneme::None) => (),
            (None, EnPhoneme::Space) => (),

            (Some(EnConsonant::R), EnPhoneme::Space | EnPhoneme::Consonant(_))
                if matches!(result.last(), Some(JaPhoneme::Vowel(Vowel::A))) =>
            {
                result.push(JaPhoneme::Consonant(Consonant::Long));
                send_to_next(&mut prev);
            }

            (Some(EnConsonant::NG), EnPhoneme::Consonant(EnConsonant::CH | EnConsonant::K)) => {
                result.push(JaPhoneme::Consonant(Consonant::Nn));
                send_to_next(&mut prev);
            }
            (Some(EnConsonant::NG), EnPhoneme::Consonant(_)) => {
                result.push(JaPhoneme::Consonant(Consonant::Nn));
                result.push(JaPhoneme::Consonant(Consonant::G));
                send_to_next(&mut prev);
            }

            (Some(consonant), EnPhoneme::Consonant(_) | EnPhoneme::Space) => {
                result.push(JaPhoneme::Consonant(katakana_consonant(&consonant)));
                send_to_next(&mut prev);
            }

            (Some(consonant), EnPhoneme::Vowel(vowel, _)) => {
                if matches!(consonant, EnConsonant::NG) {
                    result.push(JaPhoneme::Consonant(Consonant::Nn));
                    result.push(JaPhoneme::Consonant(Consonant::G));
                } else {
                    result.push(JaPhoneme::Consonant(katakana_consonant(&consonant)));
                }
                result.push(JaPhoneme::Vowel(katakana_vowel(&vowel)));
                prev = None;
            }
        }

        match &phonemes[idx] {
            EnPhoneme::Vowel(EnVowel::ER | EnVowel::IY | EnVowel::UH | EnVowel::UW, _) => {
                result.push(JaPhoneme::Consonant(Consonant::Long))
            }
            EnPhoneme::Vowel(EnVowel::AY | EnVowel::EY | EnVowel::OY, _) => {
                result.push(JaPhoneme::Vowel(Vowel::I))
            }
            EnPhoneme::Vowel(EnVowel::AW | EnVowel::OW, _) => {
                result.push(JaPhoneme::Vowel(Vowel::U))
            }
            EnPhoneme::Space => result.push(JaPhoneme::Space),
            _ => (),
        }
    }
    result
}

pub fn ja_to_ja(phonemes: &mut Vec<JaPhoneme>) {
    let mut iter = IterQuintMut::new(phonemes);
    while let Some(quint) = iter.next() {
        let d = Triple::from(quint);
        let (prev, curr) = match &d {
            Triple::Last(
                JaPhoneme::Consonant(Consonant::N),
                JaPhoneme::Space | JaPhoneme::Consonant(_),
            )
            | Triple::Last(
                JaPhoneme::Consonant(Consonant::M),
                JaPhoneme::Consonant(Consonant::B | Consonant::P | Consonant::M),
            )
            | Triple::Full(
                JaPhoneme::Consonant(Consonant::N),
                JaPhoneme::Space | JaPhoneme::Consonant(_),
                _,
            )
            | Triple::Full(
                JaPhoneme::Consonant(Consonant::M),
                JaPhoneme::Consonant(Consonant::B | Consonant::P | Consonant::M),
                _,
            ) => (Some(JaPhoneme::Consonant(Consonant::Nn)), None),

            Triple::Full(
                JaPhoneme::Consonant(Consonant::T),
                JaPhoneme::Consonant(Consonant::S),
                JaPhoneme::Space,
            ) => (
                Some(JaPhoneme::Consonant(Consonant::Ts)),
                Some(JaPhoneme::Vowel(Vowel::U)),
            ),
            Triple::Full(
                JaPhoneme::Consonant(Consonant::D),
                JaPhoneme::Consonant(Consonant::Z),
                JaPhoneme::Space,
            ) => (
                Some(JaPhoneme::Consonant(Consonant::Z)),
                Some(JaPhoneme::Vowel(Vowel::U)),
            ),

            Triple::Full(
                JaPhoneme::Consonant(Consonant::K),
                JaPhoneme::Consonant(Consonant::Y),
                JaPhoneme::Vowel(_),
            ) => (
                Some(JaPhoneme::Consonant(Consonant::Ky)),
                Some(JaPhoneme::None),
            ),

            _ => (None, None),
        };
        match d {
            Triple::Single(c) | Triple::First(c, _) => {
                if let Some(curr) = curr {
                    *c = curr;
                }
            }
            Triple::Last(p, c) | Triple::Full(p, c, _) => {
                if let Some(prev) = prev {
                    *p = prev;
                }
                if let Some(curr) = curr {
                    *c = curr;
                }
            }
        }
    }

    phonemes.retain(|p| !matches!(p, JaPhoneme::None));
}

pub fn add_vowel(phonemes: &mut Vec<JaPhoneme>) {
    let mut need_vowel = false;
    let mut i = 0;
    while i < phonemes.len() {
        if need_vowel && !matches!(phonemes[i], JaPhoneme::Vowel(_)) {
            let vowel = match &phonemes[i - 1] {
                JaPhoneme::Consonant(Consonant::D | Consonant::T) => Vowel::O,
                JaPhoneme::Consonant(Consonant::J) => Vowel::I,
                JaPhoneme::Consonant(_) => Vowel::U,
                _ => unreachable!(),
            };
            phonemes.insert(i, JaPhoneme::Vowel(vowel));
            i += 1;
        }

        need_vowel = match phonemes[i] {
            JaPhoneme::Consonant(Consonant::Long) => false,
            JaPhoneme::Consonant(Consonant::Nn) => false,
            JaPhoneme::Consonant(_) => true,
            _ => false,
        };

        i += 1;
    }
}

pub fn katakana_consonant(consonant: &EnConsonant) -> Consonant {
    match consonant {
        EnConsonant::B => Consonant::B,
        EnConsonant::CH => Consonant::Ch,
        EnConsonant::D => Consonant::D,
        EnConsonant::DH => Consonant::Dy,
        EnConsonant::F => Consonant::F,
        EnConsonant::G => Consonant::G,
        EnConsonant::HH => Consonant::H,
        EnConsonant::JH => Consonant::J,
        EnConsonant::K => Consonant::K,
        EnConsonant::L => Consonant::R,
        EnConsonant::M => Consonant::M,
        EnConsonant::N => Consonant::N,
        EnConsonant::NG => Consonant::G,
        EnConsonant::P => Consonant::P,
        EnConsonant::R => Consonant::R,
        EnConsonant::S => Consonant::S,
        EnConsonant::SH => Consonant::Sh,
        EnConsonant::T => Consonant::T,
        EnConsonant::TH => Consonant::Sh,
        EnConsonant::V => Consonant::V,
        EnConsonant::W => Consonant::W,
        EnConsonant::Y => Consonant::Y,
        EnConsonant::Z => Consonant::Z,
        EnConsonant::ZH => Consonant::J,
    }
}

pub fn katakana_vowel(vowel: &EnVowel) -> Vowel {
    match vowel {
        EnVowel::AA => Vowel::A,
        EnVowel::AE => Vowel::A,
        EnVowel::AH => Vowel::A,
        EnVowel::AO => Vowel::O,
        EnVowel::AW => Vowel::A,
        EnVowel::AY => Vowel::A,
        EnVowel::EH => Vowel::E,
        EnVowel::ER => Vowel::A,
        EnVowel::EY => Vowel::E,
        EnVowel::IH => Vowel::I,
        EnVowel::IY => Vowel::I,
        EnVowel::OW => Vowel::O,
        EnVowel::OY => Vowel::O,
        EnVowel::UH => Vowel::U,
        EnVowel::UW => Vowel::U,
    }
}

pub fn to_mora(katakana_consonant: &Consonant, katakana_vowel: &Vowel) -> &'static str {
    match (katakana_consonant, katakana_vowel) {
        (Consonant::B, Vowel::A) => "バ",
        (Consonant::B, Vowel::I) => "ビ",
        (Consonant::B, Vowel::U) => "ブ",
        (Consonant::B, Vowel::E) => "ベ",
        (Consonant::B, Vowel::O) => "ボ",
        (Consonant::Ch, Vowel::A) => "チャ",
        (Consonant::Ch, Vowel::I) => "チ",
        (Consonant::Ch, Vowel::U) => "チュ",
        (Consonant::Ch, Vowel::E) => "チェ",
        (Consonant::Ch, Vowel::O) => "チョ",
        (Consonant::D, Vowel::A) => "ダ",
        (Consonant::D, Vowel::I) => "ディ",
        (Consonant::D, Vowel::U) => "ドゥ",
        (Consonant::D, Vowel::E) => "デ",
        (Consonant::D, Vowel::O) => "ド",
        (Consonant::Dy, Vowel::A) => "ザ",
        (Consonant::Dy, Vowel::I) => "ディ",
        (Consonant::Dy, Vowel::U) => "デュ",
        (Consonant::Dy, Vowel::E) => "ゼ",
        (Consonant::Dy, Vowel::O) => "ジョ",
        (Consonant::F, Vowel::A) => "ファ",
        (Consonant::F, Vowel::I) => "フィ",
        (Consonant::F, Vowel::U) => "フ",
        (Consonant::F, Vowel::E) => "フェ",
        (Consonant::F, Vowel::O) => "フォ",
        (Consonant::G, Vowel::A) => "ガ",
        (Consonant::G, Vowel::I) => "ギ",
        (Consonant::G, Vowel::U) => "グ",
        (Consonant::G, Vowel::E) => "ゲ",
        (Consonant::G, Vowel::O) => "ゴ",
        (Consonant::H, Vowel::A) => "ハ",
        (Consonant::H, Vowel::I) => "ヒ",
        (Consonant::H, Vowel::U) => "フ",
        (Consonant::H, Vowel::E) => "ヘ",
        (Consonant::H, Vowel::O) => "ホ",
        (Consonant::J, Vowel::A) => "ジャ",
        (Consonant::J, Vowel::I) => "ジ",
        (Consonant::J, Vowel::U) => "ジュ",
        (Consonant::J, Vowel::E) => "ジェ",
        (Consonant::J, Vowel::O) => "ジョ",
        (Consonant::K, Vowel::A) => "カ",
        (Consonant::K, Vowel::I) => "キ",
        (Consonant::K, Vowel::U) => "ク",
        (Consonant::K, Vowel::E) => "ケ",
        (Consonant::K, Vowel::O) => "コ",
        (Consonant::R, Vowel::A) => "ラ",
        (Consonant::R, Vowel::I) => "リ",
        (Consonant::R, Vowel::U) => "ル",
        (Consonant::R, Vowel::E) => "レ",
        (Consonant::R, Vowel::O) => "ロ",
        (Consonant::M, Vowel::A) => "マ",
        (Consonant::M, Vowel::I) => "ミ",
        (Consonant::M, Vowel::U) => "ム",
        (Consonant::M, Vowel::E) => "メ",
        (Consonant::M, Vowel::O) => "モ",
        (Consonant::N, Vowel::A) => "ナ",
        (Consonant::N, Vowel::I) => "ニ",
        (Consonant::N, Vowel::U) => "ヌ",
        (Consonant::N, Vowel::E) => "ネ",
        (Consonant::N, Vowel::O) => "ノ",
        (Consonant::P, Vowel::A) => "パ",
        (Consonant::P, Vowel::I) => "ピ",
        (Consonant::P, Vowel::U) => "プ",
        (Consonant::P, Vowel::E) => "ペ",
        (Consonant::P, Vowel::O) => "ポ",
        (Consonant::S, Vowel::A) => "サ",
        (Consonant::S, Vowel::I) => "シ",
        (Consonant::S, Vowel::U) => "ス",
        (Consonant::S, Vowel::E) => "セ",
        (Consonant::S, Vowel::O) => "ソ",
        (Consonant::Sh, Vowel::A) => "シャ",
        (Consonant::Sh, Vowel::I) => "シ",
        (Consonant::Sh, Vowel::U) => "シュ",
        (Consonant::Sh, Vowel::E) => "シェ",
        (Consonant::Sh, Vowel::O) => "ショ",
        (Consonant::T, Vowel::A) => "タ",
        (Consonant::T, Vowel::I) => "ティ",
        (Consonant::T, Vowel::U) => "トゥ",
        (Consonant::T, Vowel::E) => "テ",
        (Consonant::T, Vowel::O) => "ト",
        (Consonant::V, Vowel::A) => "ヴァ",
        (Consonant::V, Vowel::I) => "ヴィ",
        (Consonant::V, Vowel::U) => "ヴ",
        (Consonant::V, Vowel::E) => "ヴェ",
        (Consonant::V, Vowel::O) => "ヴォ",
        (Consonant::W, Vowel::A) => "ワ",
        (Consonant::W, Vowel::I) => "ウィ",
        (Consonant::W, Vowel::U) => "ウ",
        (Consonant::W, Vowel::E) => "ウェ",
        (Consonant::W, Vowel::O) => "ヲ",
        (Consonant::Y, Vowel::A) => "ヤ",
        (Consonant::Y, Vowel::I) => "イ",
        (Consonant::Y, Vowel::U) => "ユ",
        (Consonant::Y, Vowel::E) => "イェ",
        (Consonant::Y, Vowel::O) => "ヨ",
        (Consonant::Z, Vowel::A) => "ザ",
        (Consonant::Z, Vowel::I) => "ズィ",
        (Consonant::Z, Vowel::U) => "ズ",
        (Consonant::Z, Vowel::E) => "ゼ",
        (Consonant::Z, Vowel::O) => "ゾ",
        (Consonant::Ts, Vowel::U) => "ツ",
        (Consonant::Ky, Vowel::A) => "キャ",
        (Consonant::Ky, Vowel::I) => "キ",
        (Consonant::Ky, Vowel::U) => "キュ",
        (Consonant::Ky, Vowel::E) => "ケ",
        (Consonant::Ky, Vowel::O) => "キョ",
        _ => unreachable!(),
    }
}

pub fn vowel_to_string(katakana_vowel: &Vowel) -> &'static str {
    match katakana_vowel {
        Vowel::A => "ア",
        Vowel::I => "イ",
        Vowel::U => "ウ",
        Vowel::E => "エ",
        Vowel::O => "オ",
        _ => "",
    }
}
