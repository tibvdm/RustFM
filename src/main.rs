use rust_fm::errors::Result;

use rust_fm::fm_index::FMIndex;
use rust_fm::alphabet::DNAAlphabet;

fn main() -> Result<()> {
    let fm_index = FMIndex::new("ACCGTAAC", DNAAlphabet::default()); 

    println!("{:?}", fm_index);

    //println!("{:?}", fm_index.exact_match(&"CGT".as_bytes().to_vec()));
    //println!("{:?}", fm_index.exact_match(&"CCG".as_bytes().to_vec()));
    println!("{:?}", fm_index.exact_match(&"C".as_bytes().to_vec()));

    Ok(())
}
