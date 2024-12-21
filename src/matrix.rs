use num_traits::{One, Zero, Float};
use std::default::Default;
use std::ops::{Add, Sub, Mul};

// Define a Matrix struct
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

// Define a trait for matrix operations
pub trait MatrixOps<T>: Sized {
    // Create a matrix with all elements set to zero
    fn zeroes(rows: usize, cols: usize) -> Self;

    // Create a matrix with all elements set to one
    fn ones(rows: usize, cols: usize) -> Self;

    // Add two matrices
    fn add(&self, other: &Self) -> Self;

    // Subtract one matrix from another
    fn sub(&self, other: &Self) -> Self;

    // Scale a matrix by a scalar
    fn scale(&self, scalar: T) -> Self;

    // Multiply two matrices
    fn mul(&self, other: &Self) -> Self;

    // Transpose the matrix
    fn transpose(&self) -> Self;
}

// Implement the MatrixOps trait for Matrix
impl<T> MatrixOps<T> for Matrix<T>
where
    T: Default + One + Zero + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    fn zeroes(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![T::zero(); rows * cols],
        }
    }

    fn ones(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![T::one(); rows * cols],
        }
    }

    fn add(&self, other: &Self) -> Self {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().zip(other.data.iter()).map(|(a, b)| *a + *b).collect(),
        }
    }

    fn sub(&self, other: &Self) -> Self {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().zip(other.data.iter()).map(|(a, b)| *a - *b).collect(),
        }
    }

    fn scale(&self, scalar: T) -> Self {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().map(|&x| x * scalar).collect(),
        }
    }

    fn mul(&self, other: &Self) -> Self {
        assert_eq!(self.cols, other.rows);
        let mut result = Matrix::zeroes(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = T::zero();
                for k in 0..self.cols {
                    sum = sum + self.data[i * self.cols + k] * other.data[k * other.cols + j];
                }
                result.data[i * other.cols + j] = sum;
            }
        }
        result
    }

    fn transpose(&self) -> Self {
        let mut result = Matrix::zeroes(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j * self.rows + i] = self.data[i * self.cols + j];
            }
        }
        result
    }
}