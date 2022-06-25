/// Bitvector with Jacobson’s rank
pub struct Bitvec {
    /// Size of the bitvector
    N: usize,

    /// The bitvector
    bitvector: Vec<u64>,

    /// Interleaved first and second level counts
    counts: Vec<u64>
}
