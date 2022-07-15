use std::str;

use rand::distributions::{
    Distribution,
    Uniform
};
use rust_fm::alphabet::{
    Alphabet,
    AlphabetChar,
    AlphabetIndex,
    AlphabetPattern,
    AlphabetString,
    DNAAlphabet
};

pub struct AlphabetGenerator<A: Alphabet> {
    alphabet: A
}

impl<A: Alphabet> AlphabetGenerator<A> {
    pub fn generate_indices(&self, amount: usize) -> Vec<AlphabetIndex> {
        let mut rng = rand::thread_rng();

        let distribution = Uniform::<usize>::from(0 .. self.alphabet.len());

        return (0 .. amount)
            .map(|_| distribution.sample(&mut rng) as AlphabetChar)
            .collect();
    }

    pub fn generate_characters(&self, amount: usize) -> Vec<AlphabetChar> {
        return self
            .generate_indices(amount)
            .iter()
            .map(|i| self.alphabet.i2c(*i))
            .collect();
    }

    pub fn generate_string(&self, string_length: usize) -> AlphabetString<A> {
        let characters = self.generate_characters(string_length);
        let input = unsafe { str::from_utf8_unchecked(&characters) };

        return AlphabetString::<A>::from(input);
    }

    pub fn generate_pattern(&self, pattern_size: usize) -> AlphabetPattern<A> {
        let characters = self.generate_characters(pattern_size);
        let input = unsafe { str::from_utf8_unchecked(&characters) };

        return AlphabetPattern::<A>::from(input);
    }
}

impl<A: Alphabet> Default for AlphabetGenerator<A> {
    fn default() -> Self {
        Self {
            alphabet: Default::default()
        }
    }
}
