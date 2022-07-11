use std::fmt;

use crate::{
    alphabet::{
        Alphabet,
        AlphabetChar,
        AlphabetIndex,
        AlphabetIndexString,
        AlphabetString,
        DNAAlphabet
    },
    bitvector::OccurenceTable,
    matrix::BandedMatrix,
    range::Range,
    suffix_array::{
        SparseSuffixArray,
        SuffixArray
    },
    tree::{
        Position,
        SearchTree
    }
};

// ======================================================================
// == FMIndex
// ======================================================================

/// FM index
pub struct FMIndex<A: Alphabet> {
    /// The original text
    text: AlphabetIndexString<A>,

    /// Burrows Wheeler Transform of the original text
    bwt: AlphabetIndexString<A>,

    /// Counts array
    counts: Vec<usize>,

    /// Position of the lexicographic smallest item
    sentinel: usize,

    /// The sparse suffix array
    sparse_sa: SparseSuffixArray,

    /// occurence table
    occurence_table: OccurenceTable
}

impl<A: Alphabet> FMIndex<A> {
    pub fn new(text: AlphabetString<A>, sparseness_factor: u32) -> Self {
        let text_length = text.len();

        // Translate each character to its index
        let translated_text = AlphabetIndexString::<A>::from(text);

        // Create the suffix array
        let suffix_array = SuffixArray::new(translated_text.bytes()).into_parts().1;

        // Create BWT from suffix array
        let mut bwt = AlphabetIndexString::<A>::new(text_length + 1);
        let sentinel = Self::bwt_from_sa(&suffix_array, &mut bwt, &translated_text);

        // Initialize the counts table
        let mut counts = vec![0; bwt.alphabet.len()];
        Self::initialize_counts(&mut counts, &bwt, sentinel);

        // Create the occurence table
        let occurence_table = OccurenceTable::from_bwt(&bwt, sentinel);

        FMIndex {
            text:            translated_text,
            bwt:             bwt,
            counts:          counts,
            sentinel:        sentinel,
            sparse_sa:       SparseSuffixArray::from_sa(&suffix_array, sparseness_factor),
            occurence_table: occurence_table
        }
    }

    fn bwt_from_sa(
        sa: &Vec<u32>,
        bwt: &mut AlphabetIndexString<A>,
        text: &AlphabetIndexString<A>
    ) -> usize {
        let mut sentinel = 0;

        for i in 0 .. sa.len() {
            if sa[i] == 0 {
                bwt[i] = 0;
                sentinel = i;
            } else {
                bwt[i] = text[sa[i] as usize - 1];
            }
        }

        return sentinel;
    }

    fn initialize_counts(counts: &mut Vec<usize>, bwt: &AlphabetIndexString<A>, sentinel: usize) {
        // Calculate counts
        for (i, char_i) in bwt.iter().enumerate() {
            if i == sentinel {
                continue;
            }

            counts[(*char_i) as usize] += 1;
        }

        // Calculate the cumulative sum
        let mut s1 = 1;
        for i in 0 .. bwt.alphabet.len() {
            let s2 = counts[i];
            counts[i] = s1;
            s1 += s2;
        }
    }

    fn find_lf(&self, k: usize) -> usize {
        if k == self.sentinel {
            return 0;
        }

        let char_i = self.bwt[k] as usize;
        return self.counts[char_i] + self.occurence_table.occ(char_i, k);
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

    pub fn alphabet(&self) -> &A {
        return &self.bwt.alphabet;
    }

    pub fn add_char_left(
        &self,
        char_i: usize,
        range: &Range<usize>,
        new_range: &mut Range<usize>
    ) -> bool {
        new_range.start = self.counts[char_i] + self.occurence_table.occ(char_i, range.start);
        new_range.end = self.counts[char_i] + self.occurence_table.occ(char_i, range.end);

        return !new_range.empty();
    }

    pub fn exact_match(&self, pattern: &Vec<AlphabetChar>) -> Vec<u32> {
        let mut result = vec![];

        let mut range = Range::new(0, self.text.len() + 1);

        for c in pattern.iter().rev() {
            if !self.add_char_left(self.alphabet().c2i(*c) as usize, &range.clone(), &mut range) {
                return result;
            }
        }

        for i in range.start .. range.end {
            result.push(self.find_sa(i));
        }

        return result;
    }

    pub fn approximate_match(&self, pattern: &Vec<AlphabetChar>, k: usize) -> Vec<Position> {
        let mut occurences: Vec<Position> = vec![];

        // TODO: create pattern struct to avoid this reverse step
        let reversed_pattern = pattern
            .iter()
            .rev()
            .map(|c| self.alphabet().c2i(*c))
            .collect::<Vec<AlphabetIndex>>();

        println!("Pattern: {:?}", pattern);
        println!("Reversed pattern: {:?}", reversed_pattern);

        let mut matrix = BandedMatrix::new(pattern.len(), k);

        let mut search_tree = SearchTree::new(self);

        search_tree.extend_search_space(&Range::new(0, self.text.len() + 1), 0);

        while let Some(item) = search_tree.next() {
            let min_edit_distance =
                matrix.update_row(&reversed_pattern, item.row(), item.character());

            if min_edit_distance < k {
                search_tree.extend_search_space(item.range(), item.row());
            }

            if matrix.in_final_column(item.row()) {
                let value = matrix.final_column(item.row());

                println!("VALUE: {:?}", value);

                if value <= k {
                    occurences.push(item);
                }
            }
        }

        // TODO: test and filter redundant matches
        return occurences;
    }
}

impl fmt::Debug for FMIndex<DNAAlphabet> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Text: {:?}\nBWT: {:?}\nSentinel position: {}\nCounts table: {:?}", /* \nOccurence table: {:?}", */
            self.text.iter().map(|x| *x as char).collect::<Vec<char>>(),
            self.bwt.iter().map(|x| *x as char).collect::<Vec<char>>(),
            self.sentinel,
            self.counts,
            //self.occurence_table
        )
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
            AlphabetChar,
            AlphabetIndexString,
            AlphabetString,
            DNAAlphabet
        },
        index::fm_index::FMIndex,
        suffix_array::SuffixArray
    };

    const INPUT: &str = "AACTAGGGCAATGTTCAACG";
    const BWT: &str = "GCACAATATGAACGGATCTAG";

    const INPUT_VEC: [AlphabetChar; 20] = [
        b'A', b'A', b'C', b'T', b'A', b'G', b'G', b'G', b'C', b'A', b'A', b'T', b'G', b'T', b'T',
        b'C', b'A', b'A', b'C', b'G'
    ];

    const BWT_VEC: [AlphabetChar; 21] = [
        b'G', b'C', b'A', b'C', b'A', b'A', b'T', b'A', b'T', b'G', b'A', b'A', b'C', b'G', b'G',
        b'A', b'T', b'C', b'T', b'A', b'G'
    ];
    const BWT_DOLLAR_POS: usize = 2;

    #[test]
    fn test_bwt_from_sa() {
        let translated_input_vec =
            AlphabetIndexString::<DNAAlphabet>::from(AlphabetString::<DNAAlphabet>::from(INPUT));
        let translated_bwt_vec =
            AlphabetIndexString::<DNAAlphabet>::from(AlphabetString::<DNAAlphabet>::from(BWT));

        let suffix_array = SuffixArray::new(&INPUT_VEC.to_vec()).into_parts().1;

        let mut bwt = AlphabetIndexString::new(21);
        FMIndex::<DNAAlphabet>::bwt_from_sa(&suffix_array, &mut bwt, &translated_input_vec);

        assert_eq!(bwt[0 .. BWT_DOLLAR_POS], translated_bwt_vec[0 .. BWT_DOLLAR_POS]);
        assert_eq!(bwt[BWT_DOLLAR_POS + 1 .. 21], translated_bwt_vec[BWT_DOLLAR_POS + 1 .. 21]);
    }

    #[test]
    fn test_initialize_counts() {
        let translated_bwt_vec =
            AlphabetIndexString::<DNAAlphabet>::from(AlphabetString::<DNAAlphabet>::from(BWT));

        let mut counts = vec![0; DNAAlphabet::default().len()];
        FMIndex::<DNAAlphabet>::initialize_counts(&mut counts, &translated_bwt_vec, BWT_DOLLAR_POS);

        let counts_results: [usize; 4] = [1, 8, 12, 17];

        assert_eq!(counts, counts_results);
    }

    #[test]
    fn test_find_lf() {
        let fm_index = FMIndex::new(AlphabetString::<DNAAlphabet>::from(INPUT), 1);

        let lf_results: Vec<usize> =
            vec![12, 8, 0, 9, 1, 2, 17, 3, 18, 13, 4, 5, 10, 14, 15, 6, 19, 11, 20, 7, 16];

        for i in 0 .. BWT_VEC.len() {
            assert_eq!(fm_index.find_lf(i), lf_results[i]);
        }
    }

    #[test]
    fn test_find_sa() {
        let fm_index = FMIndex::new(AlphabetString::<DNAAlphabet>::from(INPUT), 3);

        let sa_results: Vec<u32> =
            vec![20, 16, 0, 9, 17, 1, 4, 10, 15, 8, 18, 2, 19, 7, 6, 5, 12, 3, 14, 11, 13];

        for i in 0 .. BWT_VEC.len() {
            assert_eq!(fm_index.find_sa(i), sa_results[i]);
        }
    }

    #[test]
    fn test_exact_match() {
        let fm_index = FMIndex::new(AlphabetString::<DNAAlphabet>::from(INPUT), 3);

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

    //    #[test]
    //    fn test_approximate_match() {
    //        let fm_index = FMIndex::new(INPUT_VEC.to_vec(), DNAAlphabet::default(), 3);
    //
    //        let pattern: Vec<AlphabetChar> = vec![b'C', b'T', b'A', b'G', b'G', b'T'];
    //
    //        let res = fm_index.approximate_match(&pattern, 1);
    //
    //        println!("{:?}", res);
    //
    //        assert_eq!(true, false);
    //    }

    // const INPUT_VEC: [AlphabetChar; 20] = [
    //     b'A', b'A', b'C', b'T', b'A', b'G', b'G', b'G', b'C', b'A', b'A', b'T', b'G', b'T', b'T',
    //     b'C', b'A', b'A', b'C', b'G'
    // ];
}
