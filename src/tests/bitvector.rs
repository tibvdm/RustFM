#[cfg(test)]
mod tests {
    use crate::bitvector::Bitvec;

    const BITVEC_SIZE: usize = 10_000;

    #[test]
    fn test_index_operator_empty() {
        let bitvector = Bitvec::new(BITVEC_SIZE);

        for i in 0 .. BITVEC_SIZE {
            assert_eq!(bitvector[i], false);
        }
    }

    #[test]
    fn test_index_operator_full() {
        let mut bitvector = Bitvec::new(BITVEC_SIZE);

        for i in 0 .. BITVEC_SIZE {
            bitvector.set(i, true);
        }

        for i in 0 .. BITVEC_SIZE {
            assert_eq!(bitvector[i], true);
        }
    }

    #[test]
    fn test_index_function() {
        let mut bitvector = Bitvec::new(BITVEC_SIZE);

        for i in 0 .. 1024 {
            bitvector.set(i, true);
        }

        bitvector.calculate_counts();

        for i in 0 .. 1025 {
            assert_eq!(bitvector.rank(i), i);
        }

        for i in 1025 .. BITVEC_SIZE {
            assert_eq!(bitvector.rank(i), 1024);
        }
    }
}
