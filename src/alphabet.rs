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
        assert!(i < 5, "The alphabet contains only 4 characters!");
        return match i {
            0 => b'$',
            1 => b'A',
            2 => b'C',
            3 => b'G',
            _ => b'T'
        }
    }

    fn c2i(&self, c: AlphabetChar) -> usize {
        let i = match c {
            b'$' => 0,
            b'A' => 1,
            b'C' => 2,
            b'G' => 3,
            b'T' => 4,
            _    => 5
        };

        assert!(i < 5, "'{}' is not a part of the alphabet!", c);

        return i;
    }

    fn len(&self) -> usize {
        return 5;
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
