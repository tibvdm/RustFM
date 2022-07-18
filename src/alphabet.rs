use std::ops::{
    Deref,
    DerefMut,
    Index
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

#[derive(Clone)]
pub struct AlphabetString<A: Alphabet> {
    bytes: Vec<AlphabetIndex>,

    pub alphabet: A
}

impl<A: Alphabet> AlphabetString<A> {
    pub fn new(n: usize) -> Self {
        let bytes: Vec<AlphabetIndex> = vec![0; n];

        Self {
            bytes:    bytes,
            alphabet: Default::default()
        }
    }
}

// Please don't hate me Rust gods
impl<A: Alphabet> Deref for AlphabetString<A> {
    type Target = Vec<AlphabetIndex>;

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

// Please don't hate me Rust gods
impl<A: Alphabet> DerefMut for AlphabetString<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}

impl<A: Alphabet> From<&str> for AlphabetString<A> {
    fn from(string: &str) -> Self {
        let alphabet: A = Default::default();

        Self {
            bytes:    string.bytes().map(|c| alphabet.c2i(c)).collect(),
            alphabet: alphabet
        }
    }
}

// ======================================================================
// == AlphabetPattern
// ======================================================================

pub enum Direction {
    FORWARD,
    BACKWARD
}

impl Default for Direction {
    fn default() -> Self {
        Direction::FORWARD
    }
}

pub struct AlphabetPattern<A: Alphabet> {
    pattern: AlphabetString<A>,

    pattern_length: usize,

    direction: Direction
}

impl<A: Alphabet> AlphabetPattern<A> {
    pub fn new(pattern: &str, direction: Direction) -> Self {
        Self {
            pattern:        AlphabetString::<A>::from(pattern),
            pattern_length: pattern.len(),
            direction:      direction
        }
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn len(&self) -> usize {
        return self.pattern_length;
    }
}

impl<A: Alphabet> Index<usize> for AlphabetPattern<A> {
    type Output = AlphabetIndex;

    fn index(&self, i: usize) -> &Self::Output {
        match self.direction {
            Direction::FORWARD => &self.pattern[i],
            Direction::BACKWARD => &self.pattern[self.pattern_length - i - 1]
        }
    }
}

impl<A: Alphabet> From<&str> for AlphabetPattern<A> {
    fn from(string: &str) -> Self {
        Self {
            pattern:        AlphabetString::<A>::from(string),
            pattern_length: string.len(),
            direction:      Default::default()
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
