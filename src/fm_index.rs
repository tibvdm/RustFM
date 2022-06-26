use crate::bitvector::Bitvec;
use crate::alphabet::Alphabet;
use crate::suffix_array::SparseSuffixArray;

/// FM index
pub struct FMIndex<T: Alphabet> {
    /// The original text
    text: String,

    /// Length of the original text
    text_length: usize,

    /// Burrows Wheeler Transform of the original text
    bwt: String,

    /// The used alphabet
    alphabet: T,

    /// Counts array (TODO: 4 should be a constant somewhere)
    counts: [usize; 4],

    /// Position of the lexicographic smallest item
    dollar_pos: usize,

    /// The sparse suffix array
    sparse_sa: SparseSuffixArray,

    /// occurence table (TODO: 4 should be a constant somewhere)
    occurence_table: [Bitvec; 4]
}

impl<T: Alphabet> FMIndex<T> {
//    pub fn new() -> Self {
//
//    }

    pub fn initialize_counts(&mut self) {
        for c in self.bwt.chars() {
            // TODO: fix alphabet
            println!("TODO");
        }
    }
}
