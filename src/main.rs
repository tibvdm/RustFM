use std::{
    env,
    fs::File,
    io::{
        BufReader,
        BufWriter
    }
};

use rust_fm::{
    alphabet::{
        AlphabetPattern,
        AlphabetString,
        DNAAlphabet
    },
    errors::Result,
    index::bidirectional_fm_index::BidirectionalFMIndex,
    io::Binary
};

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    let fm_index = BidirectionalFMIndex::<DNAAlphabet>::new(AlphabetString::from("ACCGTAAC"), 1);

    let f = File::create("./tmp/foo").expect("Unable to create file");
    let f = BufWriter::new(f);

    println!("TEST");

    fm_index.to_bin(f)?;

    let f2 = File::open("./tmp/foo").expect("Unable to create file");
    let f2 = BufReader::new(f2);
    let fm_loaded = BidirectionalFMIndex::from_bin(f2)?;

    println!("{:?}", fm_loaded);

    println!("CGT: {:?}", fm_loaded.exact_match(&AlphabetPattern::<DNAAlphabet>::from("CGT")));
    println!("CCG: {:?}", fm_loaded.exact_match(&AlphabetPattern::<DNAAlphabet>::from("CCG")));
    println!("C: {:?}", fm_loaded.exact_match(&AlphabetPattern::<DNAAlphabet>::from("C")));

    Ok(())
}
