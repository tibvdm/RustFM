use std::fmt;
use std::ops::Range;

use crate::bitvector::Bitvec;
use crate::alphabet::{ Alphabet, AlphabetChar, DNAAlphabet };
use crate::suffix_array::{ SuffixArray, SparseSuffixArray };

/// FM index
pub struct FMIndex<T: Alphabet> {
    /// The original text
    text: Vec<AlphabetChar>,

    /// Burrows Wheeler Transform of the original text
    bwt: Vec<AlphabetChar>,

    /// The used alphabet
    alphabet: T,

    /// Counts array
    counts: Vec<usize>,

    /// Position of the lexicographic smallest item
    dollar_pos: usize,

    /// The sparse suffix array
    sparse_sa: SparseSuffixArray,

    /// occurence table
    occurence_table: Vec<Bitvec>
}

impl<T: Alphabet> FMIndex<T> {
    pub fn new(text: &str, alphabet: T) -> Self {
        let text_length = text.len();

        // Represent text as a vector
        let text_vec = text.bytes().collect();

        // Create the suffix array
        let (_, suffix_array) = SuffixArray::new(text.as_bytes()).into_parts();

        // Create BWT from suffix array
        let mut bwt: Vec<AlphabetChar> = vec![0; text_length + 1];
        let dollar_pos = Self::bwt_from_sa(&suffix_array, &mut bwt, &text_vec);

        // Initialize the counts table
        let mut counts = vec![0; alphabet.len()];
        Self::initialize_counts(&mut counts, &bwt, &alphabet, dollar_pos);

        // initialize the occurence table
        let mut occurence_table = vec![Bitvec::new(text_length + 1); alphabet.len()];
        Self::initialize_occurence_table(&mut occurence_table, &bwt, &alphabet, dollar_pos);

        FMIndex {
            text: text_vec,
            bwt: bwt,
            alphabet: alphabet,
            counts: counts,
            dollar_pos: dollar_pos,
            sparse_sa: SparseSuffixArray::from_sa(&suffix_array, 1),
            occurence_table: occurence_table
        }
    }

    fn bwt_from_sa(sa: &Vec<u32>, bwt: &mut Vec<AlphabetChar>, text: &Vec<AlphabetChar>) -> usize {
        let mut dollar_pos = 0;

        for i in 0 .. sa.len() {
            if sa[i] == 0 {
                bwt[i] = b'A';
                dollar_pos = i;
            } else {
                bwt[i] = text[sa[i] as usize - 1];
            }
        }

        return dollar_pos;
    }

    fn initialize_counts(counts: &mut Vec<usize>, bwt: &Vec<AlphabetChar>, alphabet: &T, dollar_pos: usize) {
        // Calculate counts
        for (i, c) in bwt.iter().enumerate() {
            if i == dollar_pos {
                continue;
            }

            counts[alphabet.c2i(*c)] += 1;
        }

        // Calculate the cumulative sum
        let mut s1 = 1;
        for i in 0 .. alphabet.len() {
            let s2 = counts[i];
            counts[i] = s1;
            s1 = s1 + s2; 
        }
    }

    fn initialize_occurence_table(occurence_table: &mut Vec<Bitvec>, bwt: &Vec<AlphabetChar>, alphabet: &T, dollar_pos: usize) {
        bwt.iter().enumerate().for_each(|(i, c)| {
            if i != dollar_pos {
                occurence_table[alphabet.c2i(*c)].set(i, true);
            }
        });

        // Calculate the counts to allow efficient rank operations
        for i in 0 .. alphabet.len() {
            occurence_table[i].calculate_counts();
        }
    }

    fn occ(&self, char_i: usize, i: usize) -> usize {
        return self.occurence_table[char_i].rank(i);
    }

    fn find_lf(&self, k: usize) -> usize {
        let i = self.alphabet.c2i(self.bwt[k]);
        return self.counts[i] + self.occ(i, k);
    }

    fn find_sa(&self, k: usize) -> u32 {
        let mut i = k;
        let mut j = 0;
        while !self.sparse_sa.contains(i as u32) {
            i = self.find_lf(i);
            j += 1;
        }

        return self.sparse_sa[i] + j;
    }

    fn add_char_left(&self, char_i: usize, range: &mut Range<usize>) -> bool {
        range.start = self.counts[char_i] + self.occ(char_i, range.start);
        range.end   = self.counts[char_i] + self.occ(char_i, range.end);

        return !range.is_empty();
    }

    pub fn exact_match(&self, pattern: &Vec<AlphabetChar>) -> Vec<u32> {
        let mut result = vec![];
        
        let mut range = 0 .. (self.text.len() + 1);

        for c in pattern.iter().rev() {
            if !self.add_char_left(self.alphabet.c2i(*c), &mut range) {
                return result;
            }
        }

        for i in range {
            result.push(self.find_sa(i));
        }

        return result;
    }
}

impl fmt::Debug for FMIndex<DNAAlphabet> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Text: {:?}\nBWT: {:?}\nDollar position: {}\nCounts table: {:?}\nOccurence table: {:?}",
            self.text.iter().map(|x| *x as char).collect::<Vec<char>>(), 
            self.bwt.iter().map(|x| *x as char).collect::<Vec<char>>(),
            self.dollar_pos,
            self.counts,
            self.occurence_table
        )
    }
}
