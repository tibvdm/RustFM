use std::env;

use rust_fm::{
    alphabet::{
        AlphabetPattern,
        AlphabetString,
        DNAAlphabet
    },
    errors::Result,
    index::fm_index::FMIndex
};

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    let fm_index = FMIndex::new(AlphabetString::<DNAAlphabet>::from("ACCGTAAC"), 1);

    //println!("{:?}", fm_index);

    println!("CGT: {:?}", fm_index.exact_match(&mut AlphabetPattern::<DNAAlphabet>::from("CGT")));
    println!("CCG: {:?}", fm_index.exact_match(&mut AlphabetPattern::<DNAAlphabet>::from("CCG")));
    println!("C: {:?}", fm_index.exact_match(&mut AlphabetPattern::<DNAAlphabet>::from("C")));

    Ok(())
}
