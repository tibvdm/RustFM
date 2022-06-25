use std::ops::Index;

const 1L: u64 = 1;

/// Bitvector with Jacobson’s rank
pub struct Bitvec {
    /// Size of the bitvector
    N: usize,

    /// The bitvector
    bitvector: Vec<u64>,

    /// Interleaved first and second level counts
    counts: Vec<u64>
}

impl Bitvec {
    pub fn new(N: usize) -> Self {
        Bitvec {
            N,
            vec![0, N],
            vec![0, N],
        }
    }
}

impl Index<usize> for Bitvec {
    type Output = bool;

    fn index(&self, pos: usize) -> &Self::Output {
        usize word = pos / 64;
        usize bit  = pos % 64;
        return bitvector[word] & (1L << bit);
    }
}

#[cfg(test)]
mod tests {
    use super::Bitvec;

}
