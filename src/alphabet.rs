pub type AlphabetChar = u8;
pub type AlphabetIndex = u8;

pub trait Alphabet {
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

// pub struct ProteinAlphabet;
//
// impl Alphabet for ProteinAlphabet {
//
// }

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
