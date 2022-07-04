use std::env;

use rust_fm::errors::Result;

use rust_fm::fm_index::FMIndex;
use rust_fm::alphabet::DNAAlphabet;

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    let fm_index = FMIndex::new("ACCGTAAC".bytes().collect(), DNAAlphabet::default()); 

    //println!("{:?}", fm_index);

    println!("CGT: {:?}", fm_index.exact_match(&"CGT".as_bytes().to_vec()));
    println!("CCG: {:?}", fm_index.exact_match(&"CCG".as_bytes().to_vec()));
    println!("C: {:?}", fm_index.exact_match(&"C".as_bytes().to_vec()));

    Ok(())
}
