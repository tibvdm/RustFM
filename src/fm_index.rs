/// FM index
pub struct FMIndex {
    /// The original text
    text: String,

    /// Length of the original text
    text_length: usize,

    /// Burrows Wheeler Transform of the original text
    bwt: String,

    /// Counts array (TODO: 4 should be a constant somewhere)
    counts: [usize; 4],

    /// Position of the lexicographic smallest item
    dollar_pos: usize,

    /// The sparse suffix array
    sparse_sa: SparseSuffixArray,

    /// TODO: occurence table
}
