use std::ops::{ Index };
use bitintr::Popcnt;

const ULL1: u64 = 1;

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
    pub fn new(n: usize) -> Self {
        let bitvector = vec![0, (n as u64 + 63) / 64];
        let counts    = vec![0, (n + 7)  / 4 ];
        Bitvec { n, bitvector, counts }
    }

    /// Index the bitvector by calculating the count levels
    pub fn index(&mut self) {
        let mut level1_counts: usize = 0;
        let mut level2_counts: usize = 0;

        let mut q: usize = 0;
        for w in 0 .. self.n {
            if w % 8 == 0 {
                level1_counts += level2_counts;
                self.counts[q] = level1_counts;

                // Reset level2 counts
                level2_counts  = self.bitvector[w].popcnt() as usize;

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
        let bit:  usize = pos % 64;
        return (self.bitvector[word] & (ULL1 << bit)) != 0;
    }

    /// Set the bit at a position to a different value
    pub fn set(&mut self, pos: usize, value: bool) {
        let word: usize = pos / 64;
        let bit:  usize = pos % 64;
        
        if value {
            self.bitvector[word] |= ULL1 << bit;
        } else {
            self.bitvector[word] &= !(ULL1 << bit);
        }
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

#[cfg(test)]
mod tests {
    use super::Bitvec;

    #[test]
    fn test_index_operator_empty() {
        let bitvector = Bitvec::new(5);

        assert_eq!(bitvector[0], false);
        assert_eq!(bitvector[1], false);
        assert_eq!(bitvector[2], false);
        assert_eq!(bitvector[3], false);
        assert_eq!(bitvector[4], false);
    }

    #[test]
    fn test_index_operator_full() {
        let mut bitvector = Bitvec::new(5);

        bitvector.set(0, true);
        bitvector.set(1, true);
        bitvector.set(2, true);
        bitvector.set(3, true);
        bitvector.set(4, true);

        assert_eq!(bitvector[0], true);
        assert_eq!(bitvector[1], true);
        assert_eq!(bitvector[2], true);
        assert_eq!(bitvector[3], true);
        assert_eq!(bitvector[4], true);
    }
}
