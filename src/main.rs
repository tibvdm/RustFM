use rust_fm::errors::Result;

use rust_fm::fm_index::FMIndex;
use rust_fm::alphabet::DNAAlphabet;

fn main() -> Result<()> {
    let fm_index = FMIndex::new("ACCGTAAC", DNAAlphabet::default()); 

    println!("{:?}", fm_index);

    Ok(())
}
