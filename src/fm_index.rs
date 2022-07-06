use std::{
    fmt,
    ops::Range
};

use crate::{
    alphabet::{
        Alphabet,
        AlphabetChar,
        DNAAlphabet
    },
    bitvector::Bitvec,
    suffix_array::{
        SparseSuffixArray,
        SuffixArray
    }
};

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
    pub fn new(text: Vec<AlphabetChar>, alphabet: T, sparseness_factor: u32) -> Self {
        let text_length = text.len();

        // Create the suffix array
        let (_, suffix_array) = SuffixArray::new(&text).into_parts();

        // Create BWT from suffix array
        let mut bwt: Vec<AlphabetChar> = vec![0; text_length + 1];
        let dollar_pos = Self::bwt_from_sa(&suffix_array, &mut bwt, &text);

        // Initialize the counts table
        let mut counts = vec![0; alphabet.len()];
        Self::initialize_counts(&mut counts, &bwt, &alphabet, dollar_pos);

        // initialize the occurence table
        let mut occurence_table = vec![Bitvec::new(text_length + 1); alphabet.len()];
        Self::initialize_occurence_table(&mut occurence_table, &bwt, &alphabet, dollar_pos);

        FMIndex {
            text:            text,
            bwt:             bwt,
            alphabet:        alphabet,
            counts:          counts,
            dollar_pos:      dollar_pos,
            sparse_sa:       SparseSuffixArray::from_sa(&suffix_array, sparseness_factor),
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

    fn initialize_counts(
        counts: &mut Vec<usize>,
        bwt: &Vec<AlphabetChar>,
        alphabet: &T,
        dollar_pos: usize
    ) {
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
            s1 += s2;
        }
    }

    fn initialize_occurence_table(
        occurence_table: &mut Vec<Bitvec>,
        bwt: &Vec<AlphabetChar>,
        alphabet: &T,
        dollar_pos: usize
    ) {
        // TODO compare if to .filter()
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
        if k == self.dollar_pos {
            return 0;
        }

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
        range.end = self.counts[char_i] + self.occ(char_i, range.end);

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
        write!(
            f,
            "Text: {:?}\nBWT: {:?}\nDollar position: {}\nCounts table: {:?}\nOccurence table: {:?}",
            self.text.iter().map(|x| *x as char).collect::<Vec<char>>(),
            self.bwt.iter().map(|x| *x as char).collect::<Vec<char>>(),
            self.dollar_pos,
            self.counts,
            self.occurence_table
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        alphabet::{
            Alphabet,
            AlphabetChar,
            DNAAlphabet
        },
        bitvector::Bitvec,
        fm_index::FMIndex,
        suffix_array::SuffixArray
    };

    const INPUT_VEC: [AlphabetChar; 20] = [
        b'A', b'A', b'C', b'T', b'A', b'G', b'G', b'G', b'C', b'A', b'A', b'T', b'G', b'T', b'T',
        b'C', b'A', b'A', b'C', b'G'
    ];

    const BWT_VEC: [AlphabetChar; 21] = [
        b'G', b'C', b'$', b'C', b'A', b'A', b'T', b'A', b'T', b'G', b'A', b'A', b'C', b'G', b'G',
        b'A', b'T', b'C', b'T', b'A', b'G'
    ];

    #[test]
    fn test_bwt_from_sa() {
        let suffix_array = SuffixArray::new(&INPUT_VEC.to_vec()).into_parts().1;

        let mut bwt: Vec<AlphabetChar> = vec![0; 21];
        let dollar_pos =
            FMIndex::<DNAAlphabet>::bwt_from_sa(&suffix_array, &mut bwt, &INPUT_VEC.to_vec());

        assert_eq!(bwt[0 .. dollar_pos], BWT_VEC.to_vec()[0 .. dollar_pos]);
        assert_eq!(bwt[dollar_pos + 1 .. 21], BWT_VEC.to_vec()[dollar_pos + 1 .. 21]);
    }

    #[test]
    fn test_initialize_counts() {
        let alphabet = DNAAlphabet::default();

        let mut counts = vec![0; alphabet.len()];
        FMIndex::<DNAAlphabet>::initialize_counts(&mut counts, &BWT_VEC.to_vec(), &alphabet, 2);

        let counts_results: [usize; 4] = [1, 8, 12, 17];

        assert_eq!(counts, counts_results);
    }

    #[test]
    fn test_initialize_occurence_table() {
        let alphabet = DNAAlphabet::default();

        let mut occurence_table = vec![Bitvec::new(21); alphabet.len()];
        FMIndex::<DNAAlphabet>::initialize_occurence_table(
            &mut occurence_table,
            &BWT_VEC.to_vec(),
            &alphabet,
            2
        );

        let mut result = vec![Bitvec::new(21); alphabet.len()];
        for i in 0 .. BWT_VEC.len() {
            if i == 2 {
                continue;
            }

            result[alphabet.c2i(BWT_VEC[i])].set(i, true);
        }

        assert_eq!(occurence_table, result);
    }

    #[test]
    fn test_occ() {
        let fm_index = FMIndex::new(INPUT_VEC.to_vec(), DNAAlphabet::default(), 1);

        let occ_results: Vec<Vec<usize>> = vec![
            vec![0, 0, 0, 0, 0, 1, 2, 2, 3, 3, 3, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7],
            vec![0, 0, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4],
            vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 4, 4, 4, 4, 4, 4],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 4, 4],
        ];

        for i in 0 .. BWT_VEC.len() {
            for j in 0 .. DNAAlphabet::default().len() {
                assert_eq!(fm_index.occ(j, i), occ_results[j][i]);
            }
        }
    }

    #[test]
    fn test_find_lf() {
        let fm_index = FMIndex::new(INPUT_VEC.to_vec(), DNAAlphabet::default(), 1);

        let lf_results: Vec<usize> =
            vec![12, 8, 0, 9, 1, 2, 17, 3, 18, 13, 4, 5, 10, 14, 15, 6, 19, 11, 20, 7, 16];

        for i in 0 .. BWT_VEC.len() {
            assert_eq!(fm_index.find_lf(i), lf_results[i]);
        }
    }

    #[test]
    fn test_find_sa() {
        let fm_index = FMIndex::new(INPUT_VEC.to_vec(), DNAAlphabet::default(), 3);

        let sa_results: Vec<u32> =
            vec![20, 16, 0, 9, 17, 1, 4, 10, 15, 8, 18, 2, 19, 7, 6, 5, 12, 3, 14, 11, 13];

        for i in 0 .. BWT_VEC.len() {
            assert_eq!(fm_index.find_sa(i), sa_results[i]);
        }
    }

    #[test]
    fn test_add_char_left() {
        let fm_index = FMIndex::new(INPUT_VEC.to_vec(), DNAAlphabet::default(), 3);

        // TODO
    }

    #[test]
    fn test_exact_match() {
        let fm_index = FMIndex::new(INPUT_VEC.to_vec(), DNAAlphabet::default(), 3);

        // Define all test cases
        let exact_match_single: Vec<Vec<AlphabetChar>> =
            vec![vec![b'A'], vec![b'C'], vec![b'G'], vec![b'T']];
        let exact_match_double: Vec<Vec<AlphabetChar>> =
            vec![vec![b'A', b'A'], vec![b'A', b'C'], vec![b'A', b'G'], vec![b'A', b'T']];
        let exact_match_start: Vec<AlphabetChar> = vec![b'A', b'A', b'C', b'T'];
        let exact_match_end: Vec<AlphabetChar> = vec![b'A', b'A', b'C', b'G'];
        let exact_match_not: Vec<AlphabetChar> = vec![b'C', b'C', b'C'];

        // Define all test results
        let exact_match_single_results: Vec<Vec<u32>> = vec![
            vec![0, 1, 4, 9, 10, 16, 17],
            vec![2, 8, 15, 18],
            vec![5, 6, 7, 12, 19],
            vec![3, 11, 13, 14],
        ];
        let exact_match_double_results: Vec<Vec<u32>> =
            vec![vec![0, 9, 16], vec![1, 17], vec![4], vec![10]];
        let exact_match_start_results: Vec<u32> = vec![0];
        let exact_match_end_results: Vec<u32> = vec![16];
        let exact_match_not_results: Vec<u32> = vec![];

        for i in 0 .. exact_match_single.len() {
            let mut result = fm_index.exact_match(&exact_match_single[i]);
            result.sort();

            assert_eq!(result, exact_match_single_results[i]);
        }

        for i in 0 .. exact_match_double.len() {
            let mut result = fm_index.exact_match(&exact_match_double[i]);
            result.sort();

            assert_eq!(result, exact_match_double_results[i]);
        }

        assert_eq!(fm_index.exact_match(&exact_match_start), exact_match_start_results);
        assert_eq!(fm_index.exact_match(&exact_match_end), exact_match_end_results);
        assert_eq!(fm_index.exact_match(&exact_match_not), exact_match_not_results);
    }
}
