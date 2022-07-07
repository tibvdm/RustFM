use crate::{
    alphabet::{
        Alphabet,
        AlphabetIndex
    },
    fm_index::FMIndex,
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

pub struct SearchTree<'a, A: Alphabet> {
    /// The fm index over which we span the tree
    fm_index: &'a FMIndex<A>,

    /// The search space
    search_space: Vec<Position>
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
