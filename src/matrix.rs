use std::{
    cmp::{
        max,
        min
    },
    fmt,
    ops::{
        Index,
        IndexMut
    }
};

use crate::alphabet::{
    Alphabet,
    AlphabetIndex,
    AlphabetPattern
};

pub struct BandedMatrix {
    /// Number of rows
    n: usize,

    /// Number of columns
    m: usize,

    /// Width of the band
    b: usize,

    /// Amount of columns per row
    columns_per_row: usize,

    /// The matrix
    matrix: Vec<usize>
}

impl BandedMatrix {
    pub fn new(pattern_size: usize, b: usize) -> Self {
        let n = pattern_size + b + 1;
        let m = pattern_size + 1;
        let columns_per_row = (2 * b + 1) + 2;

        let mut matrix = vec![0; n * columns_per_row];
        Self::initialize_matrix(&mut matrix, columns_per_row, n, m, b);

        Self {
            n:               n,
            m:               m,
            b:               b,
            columns_per_row: columns_per_row,
            matrix:          matrix
        }
    }

    fn initialize_matrix(
        matrix: &mut Vec<usize>,
        columns_per_row: usize,
        n: usize,
        m: usize,
        b: usize
    ) {
        let index = |i, j| i * columns_per_row + j - i + b;

        // initialize top row and left column
        for i in 0 ..= b {
            matrix[index(0, i)] = i;
            matrix[index(i, 0)] = i;
        }

        // Set max to the right of first b rows
        for i in 1 ..= b {
            matrix[index(i, i + b + 1)] = b + 1;
        }

        // Set max to left and right for other rows
        for i in b + 1 .. m - b - 1 {
            matrix[index(i, i + b + 1)] = b + 1;
            matrix[index(i, i - b - 1)] = b + 1;
        }

        // Set max to the left for last b rows
        let maximum = max(m as i64 - b as i64 - 1, b as i64 + 1) as usize;
        for i in maximum .. n {
            matrix[index(i, i - b - 1)] = b + 1;
        }
    }

    fn first_column(&self, row: usize) -> usize {
        max(1, row as i64 - self.b as i64) as usize
    }

    fn last_column(&self, row: usize) -> usize {
        min(self.m - 1, self.b + row)
    }

    fn update_cell(&mut self, mismatch: bool, row: usize, column: usize) -> usize {
        self[[row, column]] = min(
            min(self[[row - 1, column - 1]] + mismatch as usize, self[[row, column - 1]] + 1),
            self[[row - 1, column]] + 1
        );

        return self[[row, column]];
    }

    pub fn update_row<A: Alphabet>(
        &mut self,
        pattern: &AlphabetPattern<A>,
        row: usize,
        c: AlphabetIndex
    ) -> usize {
        let mut minimum = usize::MAX;

        for i in self.first_column(row) ..= self.last_column(row) {
            let tmp_minimum = self.update_cell(c != pattern[i - 1], row, i);
            if tmp_minimum < minimum {
                minimum = tmp_minimum;
            }
        }

        return minimum;
    }

    pub fn in_final_column(&self, row: usize) -> bool {
        return self.last_column(row) == self.m - 1;
    }

    pub fn final_column(&self, row: usize) -> usize {
        return self[[row, self.m - 1]];
    }
}

impl Index<[usize; 2]> for BandedMatrix {
    type Output = usize;

    fn index(&self, pos: [usize; 2]) -> &Self::Output {
        &self.matrix[pos[0] * self.columns_per_row + pos[1] - pos[0] + self.b]
    }
}

impl IndexMut<[usize; 2]> for BandedMatrix {
    fn index_mut(&mut self, pos: [usize; 2]) -> &mut Self::Output {
        &mut self.matrix[pos[0] * self.columns_per_row + pos[1] - pos[0] + self.b]
    }
}

impl fmt::Debug for BandedMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(for i in 0 .. self.n {
            let first_column = self.first_column(i);
            let last_column = self.last_column(i);

            let mut row: String = "row: ".to_owned();

            let mut start_column = 0;

            if first_column == 1 && i <= self.b {
                row += format!("{} ", self[[i, 0]]).as_str();
                //row += " ";
                start_column += 1;
            }

            for _ in start_column .. first_column {
                row += "  ";
            }

            for j in first_column ..= last_column {
                row += format!("{} ", self[[i, j]]).as_str();
            }

            for _ in last_column + 1 .. self.m {
                row += "  ";
            }

            writeln!(f, "{}", row)?
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        alphabet::{
            AlphabetPattern,
            DNAAlphabet
        },
        matrix::BandedMatrix
    };

    #[test]
    fn test_new() {
        let banded_matrix = BandedMatrix::new(6, 1);

        let result = vec![
            0, 0, 1, 0, 0, 1, 0, 0, 2, 2, 0, 0, 0, 2, 2, 0, 0, 0, 2, 2, 0, 0, 0, 2, 2, 0, 0, 0, 0,
            2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
        ];

        assert_eq!(banded_matrix.matrix, result);
        assert_eq!(banded_matrix.n, 8);
        assert_eq!(banded_matrix.m, 7);
        assert_eq!(banded_matrix.b, 1);
        assert_eq!(banded_matrix.columns_per_row, 5);
    }

    #[test]
    fn test_first_column() {
        let banded_matrix = BandedMatrix::new(6, 1);

        let column_results = vec![(1, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)];

        for i in 1 .. banded_matrix.n {
            assert_eq!(banded_matrix.first_column(i), column_results[i - 1].0);
            assert_eq!(banded_matrix[[i, banded_matrix.first_column(i)]], column_results[i - 1].1);
        }
    }

    #[test]
    fn test_last_column() {
        let banded_matrix = BandedMatrix::new(6, 1);

        let column_results = vec![(2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (6, 0), (6, 0)];

        for i in 1 .. banded_matrix.n {
            assert_eq!(banded_matrix.last_column(i), column_results[i - 1].0);
            assert_eq!(banded_matrix[[i, banded_matrix.last_column(i)]], column_results[i - 1].1);
        }
    }

    #[test]
    fn test_update_cell_match() {
        let mut banded_matrix = BandedMatrix::new(6, 1);

        assert_eq!(banded_matrix[[1, 1]], 0);

        banded_matrix.update_cell(false, 1, 1);

        assert_eq!(banded_matrix[[1, 1]], 0);
    }

    #[test]
    fn test_update_cell_mismatch() {
        let mut banded_matrix = BandedMatrix::new(6, 1);

        assert_eq!(banded_matrix[[1, 1]], 0);

        banded_matrix.update_cell(true, 1, 1);

        assert_eq!(banded_matrix[[1, 1]], 1);
    }

    #[test]
    fn test_update_row() {
        let mut banded_matrix = BandedMatrix::new(6, 1);

        let pattern = AlphabetPattern::<DNAAlphabet>::from("ACAAGT");

        assert_eq!(banded_matrix[[1, 1]], 0);
        assert_eq!(banded_matrix[[1, 2]], 0);

        let min_edit_distance = banded_matrix.update_row(&pattern, 1, 0);

        assert_eq!(banded_matrix[[1, 1]], 0);
        assert_eq!(banded_matrix[[1, 2]], 1);
        assert_eq!(min_edit_distance, 0);
    }

    #[test]
    fn test_in_final_column() {
        let banded_matrix = BandedMatrix::new(6, 1);

        let in_final_column_results = vec![false, false, false, false, false, true, true, true];

        for i in 0 .. banded_matrix.m {
            assert_eq!(banded_matrix.in_final_column(i), in_final_column_results[i]);
        }
    }

    #[test]
    fn test_final_column() {
        let mut banded_matrix = BandedMatrix::new(6, 1);

        for i in 5 ..= 7 {
            assert_eq!(banded_matrix.final_column(i), 0);

            banded_matrix[[i, 6]] = 1;

            assert_eq!(banded_matrix.final_column(i), 1);
        }
    }
}
