use std::fs::File;

use rust_fm::errors::Result;
use rust_fm::alphabet::DNAAlphabet;

fn main() -> Result<()> {
    let mut file = File::open("./data/a.text")?;
    
    Ok(())
}
