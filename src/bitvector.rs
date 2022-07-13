use std::{
    fmt,
    ops::Index
};

use bitintr::Popcnt;

use crate::alphabet::{
    Alphabet,
    AlphabetString
};

const ULL1: u64 = 1;

// ======================================================================
// == Bitvec
// ======================================================================

#[derive(Clone, PartialEq)]
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

    /// Get the length of the bitvector
    pub fn len(&self) -> usize {
        return self.n;
    }

    /// Get the level 1 counts
    fn level1_counts(&self, w: usize) -> usize {
        return self.counts[(w / 8) * 2];
    }

    /// Get the level 2 counts
    fn level2_counts(&self, w: usize) -> usize {
        // Interleaved position in counts table
        let q = (w / 8) * 2;
        let t: i64 = (w % 8) as i64 - 1;
        return self.counts[q + 1] >> (t + (t >> 60 & 8)) * 9 & 0x1FF;
    }

    /// Get the level 3 counts
    fn level3_counts(&self, w: usize, b: usize) -> usize {
        return ((self.bitvector[w] << 1) << (63 - b)).popcnt() as usize;
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

// ======================================================================
// == OccurenceTable
// ======================================================================

pub struct OccurenceTable {
    // TODO: make array? because 2D vec now?
    table: Vec<Bitvec>,

    /// Position of the sentinel character
    sentinel: usize
}

impl OccurenceTable {
    pub fn from_bwt<A: Alphabet>(bwt: &AlphabetString<A>, sentinel: usize) -> Self {
        let alphabet_length = bwt.alphabet.len();

        let mut table = vec![Bitvec::new(bwt.len()); alphabet_length];

        // TODO compare if to .filter()
        bwt.iter().enumerate().for_each(|(i, char_i)| {
            if i != sentinel {
                for j in (*char_i) as usize .. alphabet_length {
                    table[j].set(i, true);
                }
            }
        });

        // Calculate the counts to allow efficient rank operations
        for i in 0 .. alphabet_length {
            table[i].calculate_counts();
        }

        Self {
            table,
            sentinel
        }
    }

    pub fn occ(&self, char_i: usize, i: usize) -> usize {
        if char_i == 0 {
            return self.table[char_i].rank(i);
        }
        return self.table[char_i].rank(i) - self.table[char_i - 1].rank(i);
    }

    pub fn cumulative_occ(&self, char_i: usize, i: usize) -> usize {
        if char_i == 0 {
            return (self.sentinel < i) as usize;
        }
        return self.table[char_i - 1].rank(i) + (self.sentinel < i) as usize;
    }
}

// ======================================================================
// == Tests
// ======================================================================

#[cfg(test)]
mod tests {
    use crate::{
        alphabet::{
            Alphabet,
            AlphabetIndex,
            AlphabetString,
            DNAAlphabet
        },
        bitvector::{
            Bitvec,
            OccurenceTable
        }
    };

    const BITVEC_SIZE: usize = 10_000;

    const BWT: &str = "GCACAATATGAACGGATCTAG";
    const BWT_INDEX_VEC: [AlphabetIndex; 21] =
        [2, 1, 0, 1, 0, 0, 3, 0, 3, 2, 0, 0, 1, 2, 2, 0, 3, 1, 3, 0, 2];
    const SENTINEL_POS: usize = 2;

    #[test]
    fn test_index_operator_empty() {
        let bitvector = Bitvec::new(BITVEC_SIZE);

        for i in 0 .. BITVEC_SIZE {
            assert_eq!(bitvector[i], false);
        }
    }

    #[test]
    fn test_index_operator_full() {
        let mut bitvector = Bitvec::new(BITVEC_SIZE);

        for i in 0 .. BITVEC_SIZE {
            bitvector.set(i, true);
        }

        for i in 0 .. BITVEC_SIZE {
            assert_eq!(bitvector[i], true);
        }
    }

    #[test]
    fn test_index_function() {
        let mut bitvector = Bitvec::new(BITVEC_SIZE);

        for i in 0 .. 1024 {
            bitvector.set(i, true);
        }

        bitvector.calculate_counts();

        for i in 0 .. 1025 {
            assert_eq!(bitvector.rank(i), i);
        }

        for i in 1025 .. BITVEC_SIZE {
            assert_eq!(bitvector.rank(i), 1024);
        }
    }

    #[test]
    fn test_initialize_occurence_table() {
        let alphabet = DNAAlphabet::default();

        let occurence_table =
            OccurenceTable::from_bwt(&AlphabetString::<DNAAlphabet>::from(BWT), SENTINEL_POS);

        let mut result = vec![Bitvec::new(21); alphabet.len()];
        for i in 0 .. BWT_INDEX_VEC.len() {
            if i == SENTINEL_POS {
                continue;
            }

            for j in BWT_INDEX_VEC[i] as usize .. alphabet.len() {
                result[j].set(i, true);
            }
        }

        assert_eq!(occurence_table.table, result);
    }

    #[test]
    fn test_occ() {
        let occurence_table =
            OccurenceTable::from_bwt(&AlphabetString::<DNAAlphabet>::from(BWT), SENTINEL_POS);

        let occ_results: Vec<Vec<usize>> = vec![
            vec![0, 0, 0, 0, 0, 1, 2, 2, 3, 3, 3, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7],
            vec![0, 0, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4],
            vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 4, 4, 4, 4, 4, 4],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 4, 4],
        ];

        for i in 0 .. BWT_INDEX_VEC.len() {
            for j in 0 .. DNAAlphabet::default().len() {
                assert_eq!(occurence_table.occ(j, i), occ_results[j][i]);
            }
        }
    }

    #[test]
    fn test_cumulative_occ() {
        let occurence_table =
            OccurenceTable::from_bwt(&AlphabetString::<DNAAlphabet>::from(BWT), SENTINEL_POS);

        let occ_results: Vec<Vec<usize>> = vec![
            vec![0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![0, 0, 0, 1, 1, 2, 3, 3, 4, 4, 4, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8],
            vec![0, 0, 1, 2, 3, 4, 5, 5, 6, 6, 6, 7, 8, 9, 9, 9, 10, 10, 11, 11, 12],
            vec![0, 1, 2, 3, 4, 5, 6, 6, 7, 7, 8, 9, 10, 11, 12, 13, 14, 14, 15, 15, 16],
        ];

        for i in 0 .. BWT_INDEX_VEC.len() {
            for j in 0 .. DNAAlphabet::default().len() {
                assert_eq!(occurence_table.cumulative_occ(j, i), occ_results[j][i]);
            }
        }
    }
}
