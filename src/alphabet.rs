pub trait Alphabet {
    fn i2c(i: usize) -> char;
    fn c2i(c: char) -> usize;
}

pub struct DNAAlphabet;

impl Alphabet for DNAAlphabet {
    fn i2c(i: usize) -> char {
        assert!(i < 4, "The alphabet contains only 4 characters!");
        return match i {
            0 => 'A',
            1 => 'C',
            2 => 'G',
            _ => 'T'
        }
    }

    fn c2i(c: char) -> usize {
        let i = match c {
            'A' => 0,
            'C' => 1,
            'G' => 2,
            'T' => 3,
            _   => 4
        };

        assert!(i < 4, "'{}' is not a part of the alphabet!", c);

        return i;
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
