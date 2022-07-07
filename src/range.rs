use std::cmp::PartialOrd;

/// Represents a range [start, end[
#[derive(Clone, Copy)]
pub struct Range<T: PartialOrd> {
    /// Start value of the range
    pub start: T,

    /// End value of the range (excluded)
    pub end: T
}

impl<T: PartialOrd> Range<T> {
    pub fn new(start: T, end: T) -> Self {
        Self {
            start: start,
            end:   end
        }
    }

    pub fn empty(&self) -> bool {
        return self.end <= self.start;
    }
}
