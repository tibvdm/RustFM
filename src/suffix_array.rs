/// Sparse suffix array for FM indices
pub struct SparseSuffixArray {
    /// The sparseness factor
    sparseness_factor: u32,

    /// TODO: bitvector
    bitvector: u32,

    /// The sparse suffix array
    sparse_sa: Vec<u32>
}
