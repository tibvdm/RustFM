use crate::bitvector::Bitvec;
use crate::alphabet::Alphabet;
use crate::suffix_array::SuffixArray;
use crate::suffix_array::SparseSuffixArray;

/// FM index
pub struct FMIndex<T: Alphabet> {
    /// The original text
    text: String,

    /// Length of the original text
    text_length: usize,

    /// Burrows Wheeler Transform of the original text
    bwt: String,

    /// The used alphabet
    alphabet: T,

    /// Counts array
    counts: Vec<u32>,

    /// Position of the lexicographic smallest item
    dollar_pos: usize,

    /// The sparse suffix array
    sparse_sa: SparseSuffixArray,

    /// occurence table
    occurence_table: Vec<Bitvec>
}

impl<T: Alphabet> FMIndex<T> {
    pub fn new(text: String, alphabet: T) -> Self {
        let bwt = "ACCAGT".to_string();
        let text_length = text.len();

        // Initialize the counts table
        let mut counts = vec![0; alphabet.len()];
        Self::initialize_counts(&mut counts, &bwt, &alphabet);

        // initialize the occurence table
        let mut occurence_table = vec![Bitvec::new(text_length + 1); alphabet.len()];
        Self::initialize_occurence_table(&mut occurence_table, &bwt, &alphabet);

        FMIndex {
            text: text,
            text_length: text_length,
            bwt: bwt, // TODO
            alphabet: alphabet,
            counts: counts,
            dollar_pos: 0, // TODO
            sparse_sa: SparseSuffixArray::from_sa(SuffixArray::new(&[1, 2, 3]), 1), // TODO
            occurence_table: occurence_table
        }
    }

    fn initialize_counts(counts: &mut Vec<u32>, bwt: &String, alphabet: &T) {
        // Calculate counts
        for c in bwt.chars() {
            counts[alphabet.c2i(c)] += 1;
        }

        // Calculate the cumulative sum
        for i in 1 .. alphabet.len() {
            counts[i] += counts[i - 1];
        }
    }

    fn initialize_occurence_table(occurence_table: &mut Vec<Bitvec>, bwt: &String, alphabet: &T) {
        bwt.chars().enumerate().for_each(|(i, c)| {
            if c != '$' {
                occurence_table[alphabet.c2i(c)].set(i, true);
            }
        });

        // Calculate the counts to allow efficient rank operations
        for i in 0 .. alphabet.len() {
            occurence_table[i].calculate_counts();
        }
    }

//    fn occ(&self, char_i: u32, i: u32) -> usize {
//        if char_i == 0 {
//            return if i > self.dollar_pos { 1 } else { 0 };
//        }
//
//        return self.occurence_table[char_i - 1].rank(i);
//    }
//
//    fn find_lf(&self, k: u32) -> u32 {
//        // Fix this later (String -> &str)
//        let i = self.alphabet.c2i(self.bwt.chars().nth(k).unwrap());
//        return self.counts[i] + self.occ(i, k);
//    }
//
//    fn find_sa(&self, k: u32) -> u32 {
//        let mut i = k;
//        let mut j = 0;
//        while self.sparse_sa.contains(i) {
//            i = self.find_lf(i);
//            j += 1;
//        }
//
//        return self.sparse_sa[i] + j;
//    }
}
