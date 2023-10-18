use std::{collections::HashMap, str::FromStr};

use arpabet::EnPhoneme;
use prep::preprocess;

pub mod arpabet;
pub mod jp_phoneme;
pub mod prep;

pub struct KatakanaEigo {
    dictionary: HashMap<String, Vec<EnPhoneme>>,
}

impl KatakanaEigo {
    pub fn from_dict(dict: &[u8]) -> Result<Self, strum::ParseError> {
        let mut dictionary = HashMap::new();
        for (row_id, entry_bin) in dict.split(|&b| b == b'\n').enumerate() {
            if entry_bin.starts_with(b";;;") || entry_bin.is_empty() {
                continue;
            }

            if let Ok(entry) = String::from_utf8(entry_bin.to_vec()) {
                let mut split: std::str::Split<'_, char> = entry.split(' ');

                let string = split.next().unwrap().to_string();
                let pronunciation = split
                    .skip_while(|s| s.is_empty())
                    .map(|s| s.replace('\r', ""))
                    .map(|s| EnPhoneme::from_str(&s))
                    .collect::<Result<Vec<EnPhoneme>, _>>()?;

                dictionary.insert(string, pronunciation);
            } else {
                eprintln!("Failed to parse row {} as UTF-8.", row_id);
            }
        }
        Ok(Self { dictionary })
    }

    pub fn katakanize(&self, input_text: &str) -> String {
        let mut sentence: Vec<EnPhoneme> = vec![];
        for word in input_text.split([' ', ',', '.']) {
            if let Some(word) = self.dictionary.get(&word.to_ascii_uppercase()) {
                sentence.extend(word.to_vec());
                sentence.push(EnPhoneme::Space);
            }
        }
        preprocess(&mut sentence);
        jp_phoneme::to_ja(&sentence)
    }
}
