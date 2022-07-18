use num_traits::int::PrimInt;

/// Represents a range [start, end[
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Range<T: PrimInt> {
    /// Start value of the range
    pub start: T,

    /// End value of the range (excluded)
    pub end: T
}

impl<T: PrimInt> Range<T> {
    pub fn new(start: T, end: T) -> Self {
        Self {
            start: start,
            end:   end
        }
    }

    pub fn width(&self) -> T {
        if self.empty() {
            return T::zero();
        }
        return self.end - self.start;
    }

    pub fn empty(&self) -> bool {
        return self.end <= self.start;
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct RangePair<T: PrimInt> {
    pub normal_range: Range<T>,

    pub reversed_range: Range<T>
}

impl<T: PrimInt> RangePair<T> {
    pub fn new(normal_range: Range<T>, reversed_range: Range<T>) -> Self {
        Self {
            normal_range:   normal_range,
            reversed_range: reversed_range
        }
    }

    pub fn width(&self) -> T {
        self.normal_range.width()
    }

    pub fn empty(&self) -> bool {
        self.normal_range.empty()
    }
}

impl<T: PrimInt> From<(T, T, T, T)> for RangePair<T> {
    fn from(tup: (T, T, T, T)) -> Self {
        Self {
            normal_range:   Range::new(tup.0, tup.1),
            reversed_range: Range::new(tup.2, tup.3)
        }
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
