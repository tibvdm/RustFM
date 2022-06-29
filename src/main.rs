use std::fs::File;

use rust_fm::errors::Result;
use rust_fm::alphabet::DNAAlphabet;
use rust_fm::io::alphabet_reader::AlphabetReader;

fn main() -> Result<()> {
    let mut file = File::open("./data/a.text")?;

    let mut reader = AlphabetReader::new(file, DNAAlphabet::default());

    if let Some(x) = reader.read_character()? {
        println!("{}", x);
    }
    
    Ok(())
}
