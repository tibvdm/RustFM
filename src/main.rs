use std::env;

use rust_fm::{
    alphabet::{
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

    println!("CGT: {:?}", fm_index.exact_match(&"CGT".as_bytes().to_vec()));
    println!("CCG: {:?}", fm_index.exact_match(&"CCG".as_bytes().to_vec()));
    println!("C: {:?}", fm_index.exact_match(&"C".as_bytes().to_vec()));

    Ok(())
}
