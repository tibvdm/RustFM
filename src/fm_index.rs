use crate::bitvector::Bitvec;
use crate::alphabet::Alphabet;
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
    counts: Vec<usize>,

    /// Position of the lexicographic smallest item
    dollar_pos: usize,

    /// The sparse suffix array
    sparse_sa: SparseSuffixArray,

    /// occurence table
    occurence_table: Vec<Bitvec>
}

impl<T: Alphabet> FMIndex<T> {
    pub fn new(text: String, alphabet: T) -> Self {
        let text_length = text.len();

        let counts = vec![0; alphabet.len()];
        let occurence_table = vec![Bitvec::new(text_length + 1); alphabet.len()];
        
        FMIndex {
            text: text,
            text_length: text_length,
            bwt: "text".to_string(), // TODO
            alphabet: alphabet,
            counts: counts,
            dollar_pos: 0, // TODO
            sparse_sa: SparseSuffixArray::from_sa(vec![0, 1, 2], 1), // TODO
            occurence_table: occurence_table
        }
    }

    pub fn initialize_counts(&mut self) {
        // Calculate counts
        for c in self.bwt.chars() {
            self.counts[self.alphabet.c2i(c)] += 1;
        }

        // Calculate the cumulative sum
        for i in 1 .. self.alphabet.len() {
            self.counts[i] += self.counts[i - 1];
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_initialize_counts() {

    }
}
