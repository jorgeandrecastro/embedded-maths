// SPDX-License-Identifier: GPL-2.0-or-later
//! 2x2 and 3x3 `f32` matrices for `no_std` embedded systems,
//! powered by `embedded-matrix`.
//!
//! Provides `Matrix2x2` and `Matrix3x3` with native operators (`+`, `-`, `*`,
//! unary `-`, matrix-vector multiplication, `+=`, `-=`, `*=`), determinant,
//! trace, transpose, inversion (`inv`), and linear system solving (`solve`)
//! without heap allocation, `libm`, or `unsafe`.
//!
//! # Behavior & Notes
//!
//! | Operation / Input | Behavior |
//! |-------------------|----------|
//! | `Matrix2x2::new` / `Matrix3x3::new` | Builds from row-major `[[f32; N]; N]` (`identity`, `zero`, `from_diagonal`) |
//! | `inv()` | Returns `Result<M, MatrixError>` using a scale-relative singularity check |
//! | `solve(b)` | Solves `A * x = b` -> `Result<[f32; N], MatrixError>` |
//! | `rotation` / `rotation_x/y/z` | Available when the `trig` feature is enabled on `embedded-matrix` |
//! | Singular matrix | Returns `Err(MatrixError::SingularMatrix)` |
//! | `NaN`, `±Infinity`, or overflow | Returns `Err(MatrixError::NonFiniteValue)` |

pub use embedded_matrix::{Matrix2x2, Matrix3x3, MatrixError};

/// Inverts a 2x2 matrix.
pub fn inv_2x2(m: Matrix2x2) -> Result<Matrix2x2, MatrixError> {
    m.inv()
}

/// Inverts a 3x3 matrix.
pub fn inv_3x3(m: Matrix3x3) -> Result<Matrix3x3, MatrixError> {
    m.inv()
}

/// Solves a 2x2 linear system `A * x = b`.
pub fn solve_2x2(a: Matrix2x2, b: [f32; 2]) -> Result<[f32; 2], MatrixError> {
    a.solve(b)
}

/// Solves a 3x3 linear system `A * x = b`.
pub fn solve_3x3(a: Matrix3x3, b: [f32; 3]) -> Result<[f32; 3], MatrixError> {
    a.solve(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ops_inv_and_solve() {
        let a = Matrix2x2::new([[1.0, 2.0], [3.0, 4.0]]);
        assert_eq!(a.det(), -2.0);
        let inv = inv_2x2(a).unwrap();
        let prod = a * inv;
        assert!(prod.approx_eq(&Matrix2x2::identity(), 1e-5));

        let m3 = Matrix3x3::new([
            [1.0, 2.0, 3.0],
            [0.0, 1.0, 4.0],
            [5.0, 6.0, 0.0],
        ]);
        let x = solve_3x3(m3, [1.0, 2.0, 3.0]).unwrap();
        let rhs = m3 * x;
        assert!((rhs[0] - 1.0).abs() < 1e-4);
        assert!((rhs[1] - 2.0).abs() < 1e-4);
        assert!((rhs[2] - 3.0).abs() < 1e-4);
    }

    #[test]
    fn error_cases() {
        let singular = Matrix2x2::new([[1.0, 2.0], [2.0, 4.0]]);
        assert_eq!(inv_2x2(singular), Err(MatrixError::SingularMatrix));

        let nan_mat = Matrix2x2::new([[f32::NAN, 0.0], [0.0, 1.0]]);
        assert_eq!(inv_2x2(nan_mat), Err(MatrixError::NonFiniteValue));
    }
}