use std::{
    fmt,
    ops::Index
};

use bitintr::Popcnt;

const ULL1: u64 = 1;

#[derive(Clone)]
/// Bitvector with Jacobson’s rank
pub struct Bitvec {
    /// Size of the bitvector
    n: usize,

    /// The bitvector
    bitvector: Vec<u64>,

    /// Interleaved first and second level counts
    counts: Vec<usize>
}

impl Bitvec {
    /// Create a new bitvector
    pub fn new(n: usize) -> Self {
        let bitvector = vec![0; (n + 63) / 64];
        let counts = vec![0; (n + 7) / 4];
        Bitvec {
            n,
            bitvector,
            counts
        }
    }

    /// Index the bitvector by calculating the count levels
    pub fn calculate_counts(&mut self) {
        let mut level1_counts: usize = 0;
        let mut level2_counts: usize = 0;

        let mut q: usize = 0;
        for w in 0 .. self.bitvector.len() {
            if w % 8 == 0 {
                level1_counts += level2_counts;
                self.counts[q] = level1_counts;

                // Reset level2 counts
                level2_counts = self.bitvector[w].popcnt() as usize;

                // Update interleaving count
                q += 2
            } else {
                self.counts[q - 1] |= level2_counts << (((w % 8) - 1) * 9);
                level2_counts += self.bitvector[w].popcnt() as usize;
            }
        }
    }

    /// Check if a bit is set at a given position
    pub fn get(&self, pos: usize) -> bool {
        let word: usize = pos / 64;
        let bit: usize = pos % 64;
        return (self.bitvector[word] & (ULL1 << bit)) != 0;
    }

    /// Set the bit at a position to a different value
    pub fn set(&mut self, pos: usize, value: bool) {
        let w: usize = pos / 64;
        let b: usize = pos % 64;

        if value {
            self.bitvector[w] |= ULL1 << b;
        } else {
            self.bitvector[w] &= !(ULL1 << b);
        }
    }

    // Get the number of set bits in the range 0 to pos
    pub fn rank(&self, pos: usize) -> usize {
        let l1c: usize = self.level1_counts(pos / 64);
        let l2c: usize = self.level2_counts(pos / 64);
        return l1c + l2c + self.level3_counts(pos / 64, pos % 64);
    }

    /// Get the level 1 counts
    pub fn level1_counts(&self, w: usize) -> usize {
        return self.counts[(w / 8) * 2];
    }

    /// Get the level 2 counts
    pub fn level2_counts(&self, w: usize) -> usize {
        // Interleaved position in counts table
        let q = (w / 8) * 2;
        let t: i64 = (w % 8) as i64 - 1;
        return self.counts[q + 1] >> (t + (t >> 60 & 8)) * 9 & 0x1FF;
    }

    /// Get the level 3 counts
    pub fn level3_counts(&self, w: usize, b: usize) -> usize {
        return ((self.bitvector[w] << 1) << (63 - b)).popcnt() as usize;
    }

    /// Get the length of the bitvector
    pub fn len(&self) -> usize {
        return self.n;
    }
}

impl Index<usize> for Bitvec {
    type Output = bool;

    fn index(&self, pos: usize) -> &Self::Output {
        if self.get(pos) {
            return &true;
        }
        return &false;
    }
}

impl fmt::Debug for Bitvec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(for bv in self.bitvector.iter() {
            write!(f, "{:064b}", bv)?
        })
    }
}
