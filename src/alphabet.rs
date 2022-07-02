pub type AlphabetChar = u8;

pub trait Alphabet {
    fn i2c(&self, i: usize) -> AlphabetChar;
    fn c2i(&self, c: AlphabetChar) -> usize;
    fn len(&self) -> usize;
    fn bits(&self) -> usize;
}

pub struct DNAAlphabet;

impl Alphabet for DNAAlphabet {
    fn i2c(&self, i: usize) -> AlphabetChar {
        assert!(i < 4, "The alphabet contains only 4 characters!");
        return match i {
            0 => b'A',
            1 => b'C',
            2 => b'G',
            _ => b'T'
        }
    }

    fn c2i(&self, c: AlphabetChar) -> usize {
        let i = match c {
            b'A' => 0,
            b'C' => 1,
            b'G' => 2,
            b'T' => 3,
            _   => 4
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
