use std::io::{ Read, BufReader, Bytes };

use crate::errors::Result;
use crate::alphabet::Alphabet;

const BUFFER_SIZE: usize = 10_000_000;

pub struct AlphabetReader<R: Read, A: Alphabet> {
    /// Bytes yet to process
    bytes: Bytes<BufReader<R>>,

    /// The "to process" bytes
    current: ProcessedU16,

    /// The used alphabet
    alphabet: A
}

// Idee: hou 16 bits bij. Vanaf er minstens 8 bits weg zijn, lees je een niewe byte in

impl<R: Read, A: Alphabet> AlphabetReader<R, A> {
    /// Create a new AlphabetReader
    pub fn new(reader: R, alphabet: A) -> Self {
        let bytes = BufReader::with_capacity(BUFFER_SIZE, reader)
            .bytes();

        let current = ProcessedU16::default();

        Self { bytes, current, alphabet }
    }

    /// TODO
    pub fn read_character(&mut self) -> Result<Option<char>> {
        let bits_per_char = self.alphabet.bits() as u8;

        if self.current.needs_byte() {
            if let Some(byte) = self.bytes.next() {
                self.current.add_byte(byte?);
            }
        }

        Ok(Some(self.alphabet.i2c(self.current.get(bits_per_char).into())))
    }
}

/// Keeps track of processed bits (TODO: better name)
pub struct ProcessedU16 {
    /// 16 bits of data
    double_byte: u16,

    /// Integer indicating the processed bits
    processed: u8
}

impl ProcessedU16 {
    /// Return the first n remaining bits
    pub fn get(&mut self, n: u8) -> u8 {
        let bitmask = !(u16::MAX << self.processed);
        let bits    = (self.double_byte & bitmask) >> (self.processed - n);
        
        self.processed += n;

        return bits as u8;
    }

    /// Add an extra byte of data
    pub fn add_byte(&mut self, byte: u8) {
        self.double_byte <<= 8;
        self.double_byte |= byte as u16;
        self.processed -= 8;
    }

    pub fn needs_byte(&self) -> bool {
        return self.processed >= 8;
    }
}

impl Default for ProcessedU16 {
    fn default() -> Self {
        Self { double_byte: 0, processed: 0 }
    }
}
