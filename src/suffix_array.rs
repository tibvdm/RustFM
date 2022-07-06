use std::ops::Index;

pub use suffix_array::SuffixArray;

use crate::bitvector::Bitvec;

/// Sparse suffix array for FM indices
pub struct SparseSuffixArray {
    /// Control vector to keep track of stored values
    pub bitvector: Bitvec,

    /// The sparse suffix array
    pub sparse_sa: Vec<u32>
}

impl SparseSuffixArray {
    /// Construct the sparse suffix array from the entire suffix array
    pub fn from_sa(sa: &Vec<u32>, sparseness_factor: u32) -> Self {
        let mut bitvector = Bitvec::new(sa.len());
        let mut sparse_sa = Vec::new();

        for i in 0 .. sa.len() {
            if sa[i] % sparseness_factor == 0 {
                sparse_sa.push(sa[i]);
                bitvector.set(i, true);
            }
        }

        bitvector.calculate_counts();

        SparseSuffixArray {
            bitvector,
            sparse_sa
        }
    }

    /// Check whether the sparse suffix array contains the value at a position
    pub fn contains(&self, pos: u32) -> bool {
        return self.bitvector[pos as usize];
    }
}

impl Index<usize> for SparseSuffixArray {
    type Output = u32;

    fn index(&self, pos: usize) -> &Self::Output {
        return &self.sparse_sa[self.bitvector.rank(pos)];
    }
}
