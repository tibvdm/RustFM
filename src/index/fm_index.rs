use std::fmt;

//use serde::{Serialize, Deserialize};
use crate::{
    alphabet::{
        Alphabet,
        AlphabetPattern,
        AlphabetString,
        DNAAlphabet,
        Direction
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
//#[derive(Serialize, Deserialize)]
pub struct FMIndex<A: Alphabet> {
    /// The original text
    text: AlphabetString<A>,

    /// Burrows Wheeler Transform of the original text
    bwt: AlphabetString<A>,

    /// Counts array
    counts: Vec<usize>,

    /// The sparse suffix array
    sparse_sa: SparseSuffixArray,

    /// occurence table
    occurence_table: OccurenceTable
}

impl<A: Alphabet> FMIndex<A> {
    /// construct a new FM index from a text
    pub fn new(text: AlphabetString<A>, sparseness_factor: u32) -> Self {
        let text_length = text.len();

        // Create the suffix array
        let sa = SuffixArray::new(&text).into_parts().1;

        // Create BWT from suffix array
        let mut bwt = AlphabetString::<A>::new(text_length + 1);
        let sentinel = Self::bwt_from_sa(&mut bwt, &sa, &text);

        // Initialize the counts table
        let mut counts = vec![0; bwt.alphabet.len()];
        Self::initialize_counts(&mut counts, &bwt, sentinel);

        // Create the occurence table
        let occurence_table = OccurenceTable::from_bwt(&bwt, sentinel);

        FMIndex {
            text:            text,
            bwt:             bwt,
            counts:          counts,
            sparse_sa:       SparseSuffixArray::from_sa(&sa, sparseness_factor),
            occurence_table: occurence_table
        }
    }

    /// Construct the Burrows Wheeler Transformation from the suffix array
    fn bwt_from_sa(bwt: &mut AlphabetString<A>, sa: &Vec<u32>, text: &AlphabetString<A>) -> usize {
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

    /// Construct the counts table
    fn initialize_counts(counts: &mut Vec<usize>, bwt: &AlphabetString<A>, sentinel: usize) {
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

    /// Find the previous character using the LF property
    fn find_lf(&self, k: usize) -> usize {
        if k == self.occurence_table.sentinel {
            return 0;
        }

        let char_i = self.bwt[k] as usize;
        return self.counts[char_i] + self.occurence_table.occ(char_i, k);
    }

    /// Find the correct position in the original text
    fn find_sa(&self, k: usize) -> u32 {
        let mut i = k;
        let mut j = 0;
        while !self.sparse_sa.contains(i as u32) {
            i = self.find_lf(i);
            j += 1;
        }

        return self.sparse_sa[i] + j;
    }

    /// Try to add a character to the left
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

    /// Perform an exact match for a given pattern
    pub fn exact_match(&self, pattern: &mut AlphabetPattern<A>) -> Vec<u32> {
        let mut result = vec![];

        let mut range = Range::new(0, self.text.len() + 1);

        pattern.set_direction(Direction::BACKWARD);

        for i in 0 .. pattern.len() {
            if !self.add_char_left(pattern[i] as usize, &range.clone(), &mut range) {
                return result;
            }
        }

        for i in range.start .. range.end {
            result.push(self.find_sa(i));
        }

        return result;
    }

    /// Perform an approximate match for a given pattern
    pub fn approximate_match(&self, pattern: &mut AlphabetPattern<A>, k: usize) -> Vec<Position> {
        let mut occurences: Vec<Position> = vec![];

        pattern.set_direction(Direction::BACKWARD);

        let mut matrix = BandedMatrix::new(pattern.len(), k);

        let mut search_tree = SearchTree::new(self);

        search_tree.extend_search_space(&Range::new(0, self.text.len() + 1), 0);

        while let Some(item) = search_tree.next() {
            let min_edit_distance = matrix.update_row(&pattern, item.row(), item.character());

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
            self.occurence_table.sentinel,
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
            AlphabetPattern,
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
        let translated_input_vec = AlphabetString::<DNAAlphabet>::from(INPUT);
        let translated_bwt_vec = AlphabetString::<DNAAlphabet>::from(BWT);

        let suffix_array = SuffixArray::new(&INPUT_VEC.to_vec()).into_parts().1;

        let mut bwt = AlphabetString::new(21);
        FMIndex::<DNAAlphabet>::bwt_from_sa(&mut bwt, &suffix_array, &translated_input_vec);

        assert_eq!(bwt[0 .. BWT_DOLLAR_POS], translated_bwt_vec[0 .. BWT_DOLLAR_POS]);
        assert_eq!(bwt[BWT_DOLLAR_POS + 1 .. 21], translated_bwt_vec[BWT_DOLLAR_POS + 1 .. 21]);
    }

    #[test]
    fn test_initialize_counts() {
        let translated_bwt_vec = AlphabetString::<DNAAlphabet>::from(BWT);

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
        let mut exact_match_single = vec![
            AlphabetPattern::<DNAAlphabet>::from("A"),
            AlphabetPattern::<DNAAlphabet>::from("C"),
            AlphabetPattern::<DNAAlphabet>::from("G"),
            AlphabetPattern::<DNAAlphabet>::from("T"),
        ];

        let mut exact_match_double = vec![
            AlphabetPattern::<DNAAlphabet>::from("AA"),
            AlphabetPattern::<DNAAlphabet>::from("AC"),
            AlphabetPattern::<DNAAlphabet>::from("AG"),
            AlphabetPattern::<DNAAlphabet>::from("AT"),
        ];

        let mut exact_match_start = AlphabetPattern::<DNAAlphabet>::from("AACT");
        let mut exact_match_end = AlphabetPattern::<DNAAlphabet>::from("AACG");
        let mut exact_match_not = AlphabetPattern::<DNAAlphabet>::from("CCC");

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
            let mut result = fm_index.exact_match(&mut exact_match_single[i]);
            result.sort();

            assert_eq!(result, exact_match_single_results[i]);
        }

        for i in 0 .. exact_match_double.len() {
            let mut result = fm_index.exact_match(&mut exact_match_double[i]);
            result.sort();

            assert_eq!(result, exact_match_double_results[i]);
        }

        assert_eq!(fm_index.exact_match(&mut exact_match_start), exact_match_start_results);
        assert_eq!(fm_index.exact_match(&mut exact_match_end), exact_match_end_results);
        assert_eq!(fm_index.exact_match(&mut exact_match_not), exact_match_not_results);
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
