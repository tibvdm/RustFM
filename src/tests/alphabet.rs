#[cfg(test)]
mod tests {
    use crate::alphabet::{
        Alphabet,
        AlphabetChar,
        DNAAlphabet
    };

    const DNA_CHARACTERS: [AlphabetChar; 4] = [b'A', b'C', b'G', b'T'];
    const DNA_INDICES: [usize; 4] = [0, 1, 2, 3];

    #[test]
    fn test_dna_alphabet_c2i() {
        let alphabet = DNAAlphabet::default();

        for i in 0..DNA_CHARACTERS.len() {
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
