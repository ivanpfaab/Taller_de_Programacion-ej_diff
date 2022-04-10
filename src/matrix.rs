//! Current module is responsible for handling of the lcs Matrix
use std::cmp;

/// The Matrix is represented here
pub struct Matrix {
    ///Matrix values will be save by a vector
    values: Vec<u32>,
    ///How many columns does the matrix have
    col_size: usize,
    ///How many rows does the matrix have
    row_size: usize,
}

impl Matrix {
    /// To create a new Matrix, there must be 3 parameters available:
    ///
    ///     - The dimensions (amount of columns and rows that are going to be setup)
    ///     - A default value to fill the vector.
    ///
    /// # Example
    ///
    /// ```
    /// let new_matrix = Matrix::new(3,3,0);
    /// ```
    pub fn new(col_size: usize, row_size: usize, default_value: u32) -> Self {
        Matrix {
            values: vec![default_value; col_size * row_size],
            col_size,
            row_size,
        }
    }

    /// Fill the matrix with the lcs algorithm.
    /// Two comparable string vecs need to be passed as parameters.
    ///
    /// # Example
    ///
    /// ```
    /// let vec_1 = vec!["hola","chau"];
    /// let vec_2 = vec!["uno","dos"];
    ///
    /// let new_matrix = Matrix::new(2,2,0);
    /// new_matrix.lcs(vec_1,vec_2);
    /// ```
    pub fn lcs(&mut self, x: &[String], y: &[String]) {
        for (i,item_1) in x.iter().enumerate().take(self.row_size - 1) {
            for (j, item_2) in y.iter().enumerate().take(self.col_size - 1) {
                if item_1 == item_2 {
                    self.values[(j + 1) * (self.col_size) + (i + 1)] =
                        self.values[(j) * (self.col_size) + (i)] + 1;
                } else {
                    self.values[(j + 1) * (self.row_size) + (i + 1)] = cmp::max(
                        self.values[j * (self.col_size) + (i + 1)],
                        self.values[(j + 1) * (self.col_size) + i],
                    )
                }
            }
        }
    }

    /// Print two compared lcs vectors with this method.
    /// Both string vectors need to be passed as parameters along with their lengths.
    ///
    /// # Example
    ///
    ///
    /// ```
    /// let vec_1 = vec!["hola","chau"];
    /// let vec_2 = vec!["uno","dos"];
    ///
    /// let new_matrix = Matrix::new(2,2,0);
    /// new_matrix.lcs(vec_1,vec_2);
    ///
    /// new_matrix.print_diff(vec_1,vec_2,vec_1.len(),vec_2.len());
    /// ```
    pub fn print_diff(self, x: &[String], y: &[String], i: usize, j: usize) {
        if i > 0 && j > 0 && x[i - 1] == y[j - 1] {
            self.print_diff(x, y, i - 1, j - 1);
            println!("  {}", x[i - 1]);
        } else if j > 0
            && (i == 0
                || self.values[(j - 1) * (x.len() + 1) + i]
                    >= self.values[j * (x.len() + 1) + (i - 1)])
        {
            self.print_diff(x, y, i, j - 1);
            println!("> {}", y[j - 1]);
        } else if i > 0
            && (j == 0
                || self.values[(j - 1) * (x.len() + 1) + i]
                    < self.values[j * (x.len() + 1) + (i - 1)])
        {
            self.print_diff(x, y, i - 1, j);
            println!("< {}", x[i - 1]);
        } else {
            println!(" ");
        }
    }
}
