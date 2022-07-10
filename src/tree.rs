use std::fmt;

use crate::{
    alphabet::{
        Alphabet,
        AlphabetIndex,
        DNAAlphabet
    },
    index::fm_index::FMIndex,
    range::Range
};

pub struct Position {
    /// Range over the suffix array
    range: Range<usize>,

    /// Depth of the position node
    depth: usize,

    /// Index of the character
    index: AlphabetIndex
}

impl Position {
    pub fn new(range: Range<usize>, depth: usize, index: AlphabetIndex) -> Self {
        Self {
            range,
            depth,
            index
        }
    }

    pub fn range(&self) -> &Range<usize> {
        return &self.range;
    }

    pub fn character(&self) -> AlphabetIndex {
        return self.index;
    }

    pub fn row(&self) -> usize {
        return self.depth;
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        return self.range == other.range;
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: do not hardcode alphabet
        write!(
            f,
            "range: {:?}, depth: {}, char: {}",
            self.range,
            self.depth,
            DNAAlphabet::default().i2c(self.index) as char
        )
    }
}

pub struct SearchTree<'a, A: Alphabet> {
    /// The fm index over which we span the tree
    fm_index: &'a FMIndex<A>,

    /// The search space
    pub search_space: Vec<Position> // TODO: remove pub
}

impl<'a, A: Alphabet> SearchTree<'a, A> {
    pub fn new(fm_index: &'a FMIndex<A>) -> Self {
        let search_space = vec![];

        Self {
            fm_index:     fm_index,
            search_space: search_space
        }
    }

    pub fn extend_search_space(&mut self, range: &Range<usize>, depth: usize) {
        let mut range_copy = *range;
        for i in 0 .. self.fm_index.alphabet().len() {
            if self.fm_index.add_char_left(i, range, &mut range_copy) {
                println!("{:?} --> {:?}", range, range_copy);
                self.search_space
                    .push(Position::new(range_copy, depth + 1, i as AlphabetIndex));
            }
        }
    }
}

impl<'a, A: Alphabet> Iterator for SearchTree<'a, A> {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        return self.search_space.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        alphabet::{
            Alphabet,
            AlphabetChar,
            AlphabetIndex,
            DNAAlphabet
        },
        tree::{
            FMIndex,
            Position,
            Range,
            SearchTree
        }
    };

    const INPUT_VEC: [AlphabetChar; 20] = [
        b'A', b'A', b'C', b'T', b'A', b'G', b'G', b'G', b'C', b'A', b'A', b'T', b'G', b'T', b'T',
        b'C', b'A', b'A', b'C', b'G'
    ];

    #[test]
    fn test_position_new() {
        let position = Position::new(Range::new(0, 5), 0, 0);

        assert_eq!(position.range().start, 0);
        assert_eq!(position.range().end, 5);
        assert_eq!(position.character(), 0);
        assert_eq!(position.row(), 0);
    }

    #[test]
    fn test_search_tree_new() {
        let fm_index = FMIndex::new(INPUT_VEC.to_vec(), DNAAlphabet::default(), 1);

        let search_tree = SearchTree::new(&fm_index);

        assert_eq!(search_tree.search_space, vec![]);
    }

    #[test]
    fn test_extend_search_space() {
        let fm_index = FMIndex::new(INPUT_VEC.to_vec(), DNAAlphabet::default(), 1);

        let mut search_tree = SearchTree::new(&fm_index);

        let extend_results = vec![(1, 8), (8, 12), (12, 16), (17, 21)];

        search_tree.extend_search_space(&Range::new(0, 20), 0);

        assert_eq!(search_tree.search_space.len(), 4);

        for i in 0 .. fm_index.alphabet().len() {
            assert_eq!(search_tree.search_space[i].range().start, extend_results[i].0);
            assert_eq!(search_tree.search_space[i].range().end, extend_results[i].1);
            assert_eq!(search_tree.search_space[i].character(), i as AlphabetIndex);
            assert_eq!(search_tree.search_space[i].row(), 1);
        }
    }

    #[test]
    fn test_iterator() {
        let fm_index = FMIndex::new(INPUT_VEC.to_vec(), DNAAlphabet::default(), 1);

        let mut search_tree = SearchTree::new(&fm_index);

        search_tree.extend_search_space(&Range::new(0, 20), 0);

        let mut i = 0;
        while let Some(_) = search_tree.next() {
            if i < 4 {
                search_tree.extend_search_space(&Range::new(0, 20), 0);
            }

            i += 1;
        }

        assert_eq!(i, 20);
    }
}
