use std::ops::Index;

use crate::bitvector::Bitvec;

type SuffixArray = Vec<u32>;

/// Sparse suffix array for FM indices
pub struct SparseSuffixArray {
    /// Control vector to keep track of stored values
    bitvector: Bitvec,

    /// The sparse suffix array
    sparse_sa: Vec<u32>
}

impl SparseSuffixArray {
    /// Construct the sparse suffix array from the entire suffix array
    pub fn from_sa(suffix_array: SuffixArray, sparseness_factor: u32) -> Self {
        let mut bitvector = Bitvec::new(suffix_array.len());
        let mut sparse_sa = Vec::new();

        for i in 0 .. suffix_array.len() {
            if suffix_array[i] % sparseness_factor == 0 {
                sparse_sa.push(suffix_array[i]);
                bitvector.set(i, true);
            }
        }

        bitvector.calculate_counts();

        SparseSuffixArray { bitvector, sparse_sa }
    }

    /// Check whether the sparse suffix array contains the value at a position
    pub fn contains(&self, pos: usize) -> bool {
        return self.bitvector[pos];
    }
}

impl Index<usize> for SparseSuffixArray {
    type Output = u32;

    fn index(&self, pos: usize) -> &Self::Output {
        return &self.sparse_sa[self.bitvector.rank(pos)];
    }
}
