use std::cmp::PartialOrd;

/// Represents a range [start, end[
#[derive(Clone, Copy, PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use crate::range::Range;

    #[test]
    fn test_range_new() {
        let range = Range::new(0, 5);

        assert_eq!(range.start, 0);
        assert_eq!(range.end, 5);
        assert_eq!(range.empty(), false);
    }

    #[test]
    fn test_range_empty() {
        let range = Range::new(0, 0);

        assert_eq!(range.start, 0);
        assert_eq!(range.end, 0);
        assert_eq!(range.empty(), true);
    }
}
