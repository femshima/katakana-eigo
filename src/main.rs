use std::{collections::HashMap, error::Error, str::FromStr};

use crate::{arpabet::EnPhoneme, prep::preprocess};

mod arpabet;
mod jp_phoneme;
mod prep;

fn main() -> Result<(), Box<dyn Error>> {
    let cmudict = std::fs::read("cmudict-0.7b")?;

    let mut dictionary = HashMap::new();
    for (row_id, entry_bin) in cmudict.split(|&b| b == b'\n').enumerate() {
        if entry_bin.starts_with(b";;;") || entry_bin.is_empty() {
            continue;
        }

        if let Ok(entry) = String::from_utf8(entry_bin.to_vec()) {
            let mut split: std::str::Split<'_, char> = entry.split(' ');

            let string = split.next().unwrap().to_string();
            let pronunciation = split
                .skip_while(|s| s.is_empty())
                .map(|s| EnPhoneme::from_str(&s.replace('\r', "")))
                .collect::<Result<Vec<EnPhoneme>, _>>()?;

            dictionary.insert(string, pronunciation);
        } else {
            eprintln!("Failed to parse row {} as UTF-8.", row_id);
        }
    }

    let text = "The quick brown fox jumps over the lazy dog.";
    let mut sentence: Vec<EnPhoneme> = vec![];
    for word in text.split([' ', ',', '.']) {
        if let Some(word) = dictionary.get(&word.to_ascii_uppercase()) {
            sentence.extend(word.to_vec());
            sentence.push(EnPhoneme::Space);
        }
    }
    preprocess(&mut sentence);
    dbg!(jp_phoneme::to_ja(&sentence));

    Ok(())
}
