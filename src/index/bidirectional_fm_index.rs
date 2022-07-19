use serde::{
    Deserialize,
    Serialize
};

use crate::{
    alphabet::{
        Alphabet,
        AlphabetIndex,
        AlphabetPattern,
        AlphabetString,
        Direction
    },
    bitvector::OccurenceTable,
    range::RangePair,
    suffix_array::{
        SparseSuffixArray,
        SuffixArray
    }
};

/// Bidirectional FM index
#[derive(Serialize, Deserialize, Debug)]
pub struct BidirectionalFMIndex<A: Alphabet> {
    /// The original text
    text: AlphabetString<A>,

    /// Burrows Wheeler Transform of the original text
    bwt: AlphabetString<A>,

    /// Counts array
    counts: Vec<usize>,

    /// The sparse suffix array
    sparse_sa: SparseSuffixArray,

    /// Forward occurence table
    normal_occurence_table: OccurenceTable,

    /// Backward occurence table
    reversed_occurence_table: OccurenceTable
}

impl<A: Alphabet> BidirectionalFMIndex<A> {
    pub fn new(text: AlphabetString<A>, sparseness_factor: u32) -> Self {
        let text_length = text.len();

        // Create the suffix array for the forward text
        let forward_sa = SuffixArray::new(&text).into_parts().1;

        // Create the forward BWT from the forward suffix array
        let mut forward_bwt = AlphabetString::<A>::new(text_length + 1);
        let forward_sentinel = Self::forward_bwt_from_sa(&mut forward_bwt, &forward_sa, &text);

        // Create the forward occurence table
        let normal_occurence_table = OccurenceTable::from_bwt(&forward_bwt, forward_sentinel);

        // Create the suffix array for the reversed text
        let backward_sa = SuffixArray::new(
            &text
                .clone()
                .into_iter()
                .rev()
                .collect::<Vec<AlphabetIndex>>()
        )
        .into_parts()
        .1;

        // Create the reversed BWT from the backward suffix array
        let mut backward_bwt = AlphabetString::<A>::new(text_length + 1);
        let backward_sentinel = Self::backward_bwt_from_sa(&mut backward_bwt, &backward_sa, &text);

        // Create the backward occurence table
        let reversed_occurence_table = OccurenceTable::from_bwt(&backward_bwt, backward_sentinel);

        // Initialize the counts table
        let mut counts = vec![0; forward_bwt.alphabet.len()];
        Self::initialize_counts(&mut counts, &forward_bwt, forward_sentinel);

        BidirectionalFMIndex {
            text: text,
            bwt: forward_bwt,
            counts: counts,
            sparse_sa: SparseSuffixArray::from_sa(&forward_sa, sparseness_factor),
            normal_occurence_table: normal_occurence_table,
            reversed_occurence_table: reversed_occurence_table
        }
    }

    /// Construct the Burrows Wheeler Transformation from the suffix array
    fn forward_bwt_from_sa(
        bwt: &mut AlphabetString<A>,
        sa: &Vec<u32>,
        text: &AlphabetString<A>
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

    /// Construct the reversed Burrows Wheeler Transformation from the backward suffix array
    fn backward_bwt_from_sa(
        bwt: &mut AlphabetString<A>,
        sa: &Vec<u32>,
        text: &AlphabetString<A>
    ) -> usize {
        let mut sentinel = 0;

        for i in 0 .. sa.len() {
            if sa[i] == 0 {
                bwt[i] = 0;
                sentinel = i;
            } else {
                bwt[i] = text[text.len() - sa[i] as usize];
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

    pub fn add_char_left(
        &self,
        char_i: usize,
        range_pair: &RangePair<usize>,
        range_pair_new: &mut RangePair<usize>
    ) -> bool {
        let occ_start = self
            .normal_occurence_table
            .occ(char_i, range_pair.normal_range.start);
        let occ_end = self
            .normal_occurence_table
            .occ(char_i, range_pair.normal_range.end);

        let count = self.counts[char_i];

        range_pair_new.normal_range.start = count + occ_start;
        range_pair_new.normal_range.end = count + occ_end;

        let x = self
            .normal_occurence_table
            .cumulative_occ(char_i, range_pair.normal_range.end)
            - self
                .normal_occurence_table
                .cumulative_occ(char_i, range_pair.normal_range.start);
        let y = range_pair_new.normal_range.width();

        range_pair_new.reversed_range.start = range_pair.reversed_range.start + x;
        range_pair_new.reversed_range.end = range_pair.reversed_range.start + x + y;

        return !range_pair_new.empty();
    }

    pub fn add_char_right(
        &self,
        char_i: usize,
        range_pair: &RangePair<usize>,
        range_pair_new: &mut RangePair<usize>
    ) -> bool {
        let occ_start = self
            .reversed_occurence_table
            .occ(char_i, range_pair.reversed_range.start);
        let occ_end = self
            .reversed_occurence_table
            .occ(char_i, range_pair.reversed_range.end);

        let count = self.counts[char_i];

        range_pair_new.reversed_range.start = count + occ_start;
        range_pair_new.reversed_range.end = count + occ_end;

        let x = self
            .reversed_occurence_table
            .cumulative_occ(char_i, range_pair.reversed_range.end)
            - self
                .reversed_occurence_table
                .cumulative_occ(char_i, range_pair.reversed_range.start);
        let y = range_pair_new.reversed_range.width();

        range_pair_new.normal_range.start = range_pair.normal_range.start + x;
        range_pair_new.normal_range.end = range_pair.normal_range.start + x + y;

        return !range_pair_new.empty();
    }

    /// Perform an exact match for a given pattern
    pub fn exact_match(&self, pattern: &AlphabetPattern<A>) -> RangePair<usize> {
        let mut range_pair = RangePair::from((0, self.text.len() + 1, 0, self.text.len() + 1));

        match pattern.direction() {
            Direction::FORWARD => {
                for i in 0 .. pattern.len() {
                    if !self.add_char_right(
                        pattern[i] as usize,
                        &range_pair.clone(),
                        &mut range_pair
                    ) {
                        return RangePair::from((0, 0, 0, 0));
                    }
                }
            }

            Direction::BACKWARD => {
                for i in 0 .. pattern.len() {
                    if !self.add_char_left(
                        pattern[i] as usize,
                        &range_pair.clone(),
                        &mut range_pair
                    ) {
                        return RangePair::from((0, 0, 0, 0));
                    }
                }
            }
        }

        return range_pair;
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
            AlphabetPattern,
            AlphabetString,
            DNAAlphabet,
            Direction
        },
        index::bidirectional_fm_index::BidirectionalFMIndex,
        range::RangePair,
        suffix_array::SuffixArray
    };

    const INPUT: &str = "AACTAGGGCAATGTTCAACG";
    //                 $AAAAAAACCCCGGGGGTTTT
    const BWT: &str = "GCACAATATGAACGGATCTAG";
    const BWT_SENTINEL: usize = 2;

    const INPUT_REV: &str = "GCAACTTGTAACGGGATCAA";
    //                     $AAAAAAACCCCGGGGGTTTT
    const BWT_REV: &str = "AACTCAAGTGAAGAGCTGATC";
    const BWT_REV_SENTINEL: usize = 13;

    #[test]
    fn test_forward_bwt_from_sa() {
        let input = AlphabetString::<DNAAlphabet>::from(INPUT);

        let suffix_array = SuffixArray::new(&input).into_parts().1;

        let bwt_result = AlphabetString::<DNAAlphabet>::from(BWT);

        let mut bwt = AlphabetString::new(21);
        BidirectionalFMIndex::<DNAAlphabet>::forward_bwt_from_sa(&mut bwt, &suffix_array, &input);

        assert_eq!(bwt[0 .. BWT_SENTINEL], bwt_result[0 .. BWT_SENTINEL]);
        assert_eq!(bwt[BWT_SENTINEL + 1 .. 21], bwt_result[BWT_SENTINEL + 1 .. 21]);
    }

    #[test]
    fn test_backward_bwt_from_sa() {
        let input = AlphabetString::<DNAAlphabet>::from(INPUT);
        let input_rev = AlphabetString::<DNAAlphabet>::from(INPUT_REV);

        let suffix_array = SuffixArray::new(&input_rev).into_parts().1;

        let bwt_result = AlphabetString::<DNAAlphabet>::from(BWT_REV);

        let mut bwt = AlphabetString::new(21);
        BidirectionalFMIndex::<DNAAlphabet>::backward_bwt_from_sa(&mut bwt, &suffix_array, &input);

        assert_eq!(bwt[0 .. BWT_REV_SENTINEL], bwt_result[0 .. BWT_REV_SENTINEL]);
        assert_eq!(bwt[BWT_REV_SENTINEL + 1 .. 21], bwt_result[BWT_REV_SENTINEL + 1 .. 21]);
    }

    #[test]
    fn test_initialize_counts() {
        let bwt = AlphabetString::<DNAAlphabet>::from(BWT);

        let mut counts = vec![0; DNAAlphabet::default().len()];
        BidirectionalFMIndex::<DNAAlphabet>::initialize_counts(&mut counts, &bwt, BWT_SENTINEL);

        let counts_results: [usize; 4] = [1, 8, 12, 17];

        assert_eq!(counts, counts_results);
    }

    #[test]
    fn test_add_char_left() {
        let index =
            BidirectionalFMIndex::<DNAAlphabet>::new(AlphabetString::<DNAAlphabet>::from(INPUT), 1);

        let mut range_pair = RangePair::from((0, 21, 0, 21));

        assert_eq!(index.add_char_left(1, &range_pair.clone(), &mut range_pair), true);
        assert_eq!(range_pair, RangePair::<usize>::from((8, 12, 8, 12)));

        assert_eq!(index.add_char_left(2, &range_pair.clone(), &mut range_pair), true);
        assert_eq!(range_pair, RangePair::<usize>::from((13, 14, 10, 11)));

        assert_eq!(index.add_char_left(2, &range_pair.clone(), &mut range_pair), true);
        assert_eq!(range_pair, RangePair::<usize>::from((14, 15, 10, 11)));

        assert_eq!(index.add_char_left(3, &range_pair.clone(), &mut range_pair), false);
        assert_eq!(range_pair, RangePair::<usize>::from((19, 19, 11, 11)));
    }

    #[test]
    fn test_add_char_right() {
        let index =
            BidirectionalFMIndex::<DNAAlphabet>::new(AlphabetString::<DNAAlphabet>::from(INPUT), 1);

        let mut range_pair = RangePair::from((0, 21, 0, 21));

        assert_eq!(index.add_char_right(1, &range_pair.clone(), &mut range_pair), true);
        assert_eq!(range_pair, RangePair::<usize>::from((8, 12, 8, 12)));

        assert_eq!(index.add_char_right(3, &range_pair.clone(), &mut range_pair), true);
        assert_eq!(range_pair, RangePair::<usize>::from((11, 12, 18, 19)));

        assert_eq!(index.add_char_right(0, &range_pair.clone(), &mut range_pair), true);
        assert_eq!(range_pair, RangePair::<usize>::from((11, 12, 7, 8)));

        assert_eq!(index.add_char_right(3, &range_pair.clone(), &mut range_pair), false);
        assert_eq!(range_pair, RangePair::<usize>::from((12, 12, 18, 18)));
    }

    #[test]
    fn test_exact_match_backwards() {
        let index = BidirectionalFMIndex::new(AlphabetString::<DNAAlphabet>::from(INPUT), 1);

        // Define all test cases
        let exact_match_single = vec![
            AlphabetPattern::<DNAAlphabet>::new("A", Direction::BACKWARD),
            AlphabetPattern::<DNAAlphabet>::new("C", Direction::BACKWARD),
            AlphabetPattern::<DNAAlphabet>::new("G", Direction::BACKWARD),
            AlphabetPattern::<DNAAlphabet>::new("T", Direction::BACKWARD),
        ];

        let exact_match_double = vec![
            AlphabetPattern::<DNAAlphabet>::new("AA", Direction::BACKWARD),
            AlphabetPattern::<DNAAlphabet>::new("AC", Direction::BACKWARD),
            AlphabetPattern::<DNAAlphabet>::new("AG", Direction::BACKWARD),
            AlphabetPattern::<DNAAlphabet>::new("AT", Direction::BACKWARD),
        ];

        let exact_match_start = AlphabetPattern::<DNAAlphabet>::new("AACT", Direction::BACKWARD);
        let exact_match_end = AlphabetPattern::<DNAAlphabet>::new("AACG", Direction::BACKWARD);
        let exact_match_not = AlphabetPattern::<DNAAlphabet>::new("CCC", Direction::BACKWARD);

        // Define all results
        let exact_match_single_results = vec![
            RangePair::from((1, 8, 1, 8)),
            RangePair::from((8, 12, 8, 12)),
            RangePair::from((12, 17, 12, 17)),
            RangePair::from((17, 21, 17, 21)),
        ];

        let exact_match_double_results = vec![
            RangePair::from((1, 4, 2, 5)),
            RangePair::from((4, 6, 8, 10)),
            RangePair::from((6, 7, 12, 13)),
            RangePair::from((7, 8, 17, 18)),
        ];

        let exact_match_start_results = RangePair::<usize>::from((2, 3, 18, 19));
        let exact_match_end_results = RangePair::<usize>::from((1, 2, 13, 14));
        let exact_match_not_results = RangePair::<usize>::from((0, 0, 0, 0));

        for i in 0 .. exact_match_single.len() {
            assert_eq!(index.exact_match(&exact_match_single[i]), exact_match_single_results[i]);
        }

        for i in 0 .. exact_match_double.len() {
            assert_eq!(index.exact_match(&exact_match_double[i]), exact_match_double_results[i]);
        }

        assert_eq!(index.exact_match(&exact_match_start), exact_match_start_results);
        assert_eq!(index.exact_match(&exact_match_end), exact_match_end_results);
        assert_eq!(index.exact_match(&exact_match_not), exact_match_not_results);
    }

    // TODO: Verify this test again
    #[test]
    fn test_exact_match_forwards() {
        let index = BidirectionalFMIndex::new(AlphabetString::<DNAAlphabet>::from(INPUT), 1);

        // Define all test cases
        let exact_match_single = vec![
            AlphabetPattern::<DNAAlphabet>::new("A", Direction::FORWARD),
            AlphabetPattern::<DNAAlphabet>::new("C", Direction::FORWARD),
            AlphabetPattern::<DNAAlphabet>::new("G", Direction::FORWARD),
            AlphabetPattern::<DNAAlphabet>::new("T", Direction::FORWARD),
        ];

        let exact_match_double = vec![
            AlphabetPattern::<DNAAlphabet>::new("AA", Direction::FORWARD),
            AlphabetPattern::<DNAAlphabet>::new("AC", Direction::FORWARD),
            AlphabetPattern::<DNAAlphabet>::new("AG", Direction::FORWARD),
            AlphabetPattern::<DNAAlphabet>::new("AT", Direction::FORWARD),
        ];

        let exact_match_start = AlphabetPattern::<DNAAlphabet>::new("AACT", Direction::FORWARD);
        let exact_match_end = AlphabetPattern::<DNAAlphabet>::new("AACG", Direction::FORWARD);
        let exact_match_not = AlphabetPattern::<DNAAlphabet>::new("CCC", Direction::FORWARD);

        // Define all results
        let exact_match_single_results = vec![
            RangePair::from((1, 8, 1, 8)),
            RangePair::from((8, 12, 8, 12)),
            RangePair::from((12, 17, 12, 17)),
            RangePair::from((17, 21, 17, 21)),
        ];

        let exact_match_double_results = vec![
            RangePair::from((1, 4, 2, 5)),
            RangePair::from((4, 6, 8, 10)),
            RangePair::from((6, 7, 12, 13)),
            RangePair::from((7, 8, 17, 18)),
        ];

        let exact_match_start_results = RangePair::<usize>::from((2, 3, 18, 19));
        let exact_match_end_results = RangePair::<usize>::from((1, 2, 13, 14));
        let exact_match_not_results = RangePair::<usize>::from((0, 0, 0, 0));

        for i in 0 .. exact_match_single.len() {
            assert_eq!(index.exact_match(&exact_match_single[i]), exact_match_single_results[i]);
        }

        for i in 0 .. exact_match_double.len() {
            assert_eq!(index.exact_match(&exact_match_double[i]), exact_match_double_results[i]);
        }

        assert_eq!(index.exact_match(&exact_match_start), exact_match_start_results);
        assert_eq!(index.exact_match(&exact_match_end), exact_match_end_results);
        assert_eq!(index.exact_match(&exact_match_not), exact_match_not_results);
    }
}
