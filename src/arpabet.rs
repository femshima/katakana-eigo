use std::str::FromStr;

use strum_macros::EnumString;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum EnPhoneme {
    Vowel(EnVowel, Accent),
    Consonant(EnConsonant),
    Space,
    None,
}

impl FromStr for EnPhoneme {
    type Err = strum::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(consonant) = EnConsonant::from_str(s) {
            return Ok(Self::Consonant(consonant));
        }

        let (phoneme, accent_str) = s.split_at(2);

        let accent = match accent_str {
            "0" => Accent::None,
            "1" => Accent::Primary,
            "2" => Accent::Secondary,
            _ => Accent::Default,
        };

        let vowel = EnVowel::from_str(phoneme)?;

        Ok(Self::Vowel(vowel, accent))
    }
}

#[derive(EnumString, PartialEq, Eq, Clone, Debug)]
pub enum EnVowel {
    AA,
    AE,
    AH,
    AO,
    AW,
    AY,
    EH,
    ER,
    EY,
    IH,
    IY,
    OW,
    OY,
    UH,
    UW,
}

#[derive(EnumString, PartialEq, Eq, Clone, Debug)]
pub enum EnConsonant {
    B,
    CH,
    D,
    DH,
    F,
    G,
    HH,
    JH,
    K,
    L,
    M,
    N,
    NG,
    P,
    R,
    S,
    SH,
    T,
    TH,
    V,
    W,
    Y,
    Z,
    ZH,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Accent {
    None,
    Primary,
    Secondary,

    Default,
}
