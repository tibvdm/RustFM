use std::{
    ops::{
        Index,
        IndexMut,
        Range
    },
    slice::Iter
};

pub type AlphabetChar = u8;
pub type AlphabetIndex = u8;

// ======================================================================
// == Alphabet
// ======================================================================

pub trait Alphabet: Default {
    fn i2c(&self, i: AlphabetIndex) -> AlphabetChar;
    fn c2i(&self, c: AlphabetChar) -> AlphabetIndex;
    fn len(&self) -> usize;
    fn bits(&self) -> usize;
}

pub struct DNAAlphabet;

impl Alphabet for DNAAlphabet {
    fn i2c(&self, i: AlphabetIndex) -> AlphabetChar {
        assert!(i < 4, "The alphabet contains only 4 characters!");
        return match i {
            0 => b'A',
            1 => b'C',
            2 => b'G',
            _ => b'T'
        };
    }

    fn c2i(&self, c: AlphabetChar) -> AlphabetIndex {
        let i = match c {
            b'A' => 0,
            b'C' => 1,
            b'G' => 2,
            b'T' => 3,
            _ => 4
        };

        assert!(i < 4, "'{}' is not a part of the alphabet!", c);

        return i;
    }

    fn len(&self) -> usize {
        return 4;
    }

    fn bits(&self) -> usize {
        return 2;
    }
}

impl Default for DNAAlphabet {
    fn default() -> Self {
        DNAAlphabet
    }
}

// ======================================================================
// == AlphabetString
// ======================================================================

pub struct AlphabetString<A: Alphabet> {
    bytes: Vec<AlphabetChar>,

    pub alphabet: A
}

impl<A: Alphabet> AlphabetString<A> {
    pub fn bytes(&self) -> &Vec<AlphabetChar> {
        &self.bytes
    }

    pub fn iter(&self) -> Iter<AlphabetChar> {
        self.bytes.iter()
    }

    pub fn len(&self) -> usize {
        return self.bytes.len();
    }
}

impl<A: Alphabet> Index<usize> for AlphabetString<A> {
    type Output = AlphabetChar;

    fn index(&self, pos: usize) -> &Self::Output {
        &self.bytes[pos]
    }
}

impl<A: Alphabet> IndexMut<usize> for AlphabetString<A> {
    fn index_mut(&mut self, pos: usize) -> &mut Self::Output {
        &mut self.bytes[pos]
    }
}

impl<A: Alphabet> From<&str> for AlphabetString<A> {
    fn from(string: &str) -> Self {
        Self {
            bytes:    string.bytes().collect(),
            alphabet: Default::default()
        }
    }
}

// ======================================================================
// == AlphabetIndexString
// ======================================================================

pub struct AlphabetIndexString<A: Alphabet> {
    bytes: Vec<AlphabetIndex>,

    pub alphabet: A
}

impl<A: Alphabet> AlphabetIndexString<A> {
    pub fn new(n: usize) -> Self {
        let bytes = vec![0; n];

        Self {
            bytes:    bytes,
            alphabet: Default::default()
        }
    }

    pub fn iter(&self) -> Iter<AlphabetIndex> {
        self.bytes.iter()
    }

    pub fn bytes(&self) -> &Vec<AlphabetIndex> {
        &self.bytes
    }

    pub fn len(&self) -> usize {
        return self.bytes.len();
    }
}

impl<A: Alphabet> Index<usize> for AlphabetIndexString<A> {
    type Output = AlphabetIndex;

    fn index(&self, pos: usize) -> &Self::Output {
        &self.bytes[pos]
    }
}

impl<A: Alphabet> Index<Range<usize>> for AlphabetIndexString<A> {
    type Output = [AlphabetIndex];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.bytes[range]
    }
}

impl<A: Alphabet> IndexMut<usize> for AlphabetIndexString<A> {
    fn index_mut(&mut self, pos: usize) -> &mut Self::Output {
        &mut self.bytes[pos]
    }
}

impl<A: Alphabet> From<AlphabetString<A>> for AlphabetIndexString<A> {
    fn from(string: AlphabetString<A>) -> Self {
        Self {
            bytes:    string
                .iter()
                .map(|c| string.alphabet.c2i(*c))
                .collect::<Vec<AlphabetIndex>>(),
            alphabet: Default::default()
        }
    }
}

impl<A: Alphabet> From<Vec<u8>> for AlphabetIndexString<A> {
    fn from(bytes: Vec<u8>) -> Self {
        Self {
            bytes:    bytes,
            alphabet: Default::default()
        }
    }
}

// ======================================================================
// == Tests
// ======================================================================

#[cfg(test)]
mod tests {
    use crate::alphabet::{
        Alphabet,
        AlphabetChar,
        AlphabetIndex,
        DNAAlphabet
    };

    const DNA_CHARACTERS: [AlphabetChar; 4] = [b'A', b'C', b'G', b'T'];
    const DNA_INDICES: [AlphabetIndex; 4] = [0, 1, 2, 3];

    #[test]
    fn test_dna_alphabet_c2i() {
        let alphabet = DNAAlphabet::default();

        for i in 0 .. DNA_CHARACTERS.len() {
            assert_eq!(alphabet.c2i(DNA_CHARACTERS[i]), DNA_INDICES[i]);
        }
    }

    #[test]
    fn test_dna_alphabet_i2c() {
        let alphabet = DNAAlphabet::default();

        for i in 0 .. DNA_INDICES.len() {
            assert_eq!(alphabet.i2c(DNA_INDICES[i]), DNA_CHARACTERS[i]);
        }
    }

    #[test]
    fn test_dna_alphabet_len() {
        assert_eq!(DNAAlphabet::default().len(), 4)
    }

    #[test]
    fn test_dna_alphabet_bits() {
        assert_eq!(DNAAlphabet::default().bits(), 2)
    }
}
