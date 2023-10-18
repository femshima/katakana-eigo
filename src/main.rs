use std::error::Error;

use katakana_eigo::KatakanaEigo;

fn main() -> Result<(), Box<dyn Error>> {
    let cmudict = std::fs::read("cmudict-0.7b")?;

    let katakana_eigo = KatakanaEigo::from_dict(&cmudict)?;

    let mut text_buf = String::new();
    while std::io::stdin().read_line(&mut text_buf).is_ok() {
        let result = katakana_eigo.katakanize(&text_buf.replace('\n', ""));
        println!("-> {}", result);
        text_buf.clear();
    }

    Ok(())
}
