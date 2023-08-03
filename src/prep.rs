use jpreprocess_window::{IterQuintMut, QuadForward};

use crate::arpabet::{EnConsonant, EnPhoneme};

pub fn preprocess(phonemes: &mut Vec<EnPhoneme>) {
    let mut iter = IterQuintMut::new(phonemes);

    while let Some(quint) = iter.next() {
        match QuadForward::from(quint) {
            // Reduction
            QuadForward::Triple(a, &mut EnPhoneme::Space, b)
            | QuadForward::Full(a, &mut EnPhoneme::Space, b, _)
                if a == b =>
            {
                *a = EnPhoneme::None;
            }
            // Flapping
            QuadForward::Full(a, b, d, c) if *d == EnPhoneme::Space => match (&a, &b, &c) {
                (
                    EnPhoneme::Vowel(_, _),
                    EnPhoneme::Consonant(EnConsonant::T),
                    EnPhoneme::Vowel(_, _),
                ) => {
                    *d = EnPhoneme::None;
                    *b = EnPhoneme::Consonant(EnConsonant::L);
                }
                _ => (),
            },
            // QuadForward::Triple(a, b, c) | QuadForward::Full(a, b, c, _) => match (&a, &b, &c) {
            //     (Phoneme::Vowel(_, _), Phoneme::Consonant(Consonant::T), Phoneme::Vowel(_, _)) => {
            //         *b = Phoneme::Consonant(Consonant::L);
            //     }
            //     _ => (),
            // },
            _ => (),
        }
    }

    phonemes.retain(|p| !matches!(*p, EnPhoneme::None));
}
