// SPDX-License-Identifier: GPL-2.0-or-later
//! Fast inverse square root `1 / sqrt(x)` for `f32` on `no_std` embedded systems,
//! powered by `embedded-invsqrt-f32`.
//!
//! Uses IEEE 754 bit-level initial estimation refined with Newton-Raphson
//! iterations (`y = y * (1.5 - 0.5 * x * y * y)`), with zero dependencies,
//! no `libm`, no heap allocation, and `#![forbid(unsafe_code)]`.
//!
//! # Behavior & Notes
//!
//! | Input / Condition | Behavior |
//! |-------------------|----------|
//! | `x > 0.0` (finite) | Returns `Ok(approx_inv_sqrt)` refined via Newton-Raphson |
//! | `x = 4.0` | Returns `Ok(~0.5)` |
//! | Negative or invalid input | Returns `Err(...)` without panicking |
//! | `NaN` or `±Infinity` | Handled explicitly by `embedded-invsqrt-f32` |

pub use embedded_invsqrt_f32::inv_sqrt as embedded_inv_sqrt;

/// Computes an approximate inverse square root `1 / sqrt(x)` for `f32`.
pub fn inv_sqrt(x: f32) -> Result<f32, embedded_invsqrt_f32::NegativeOrZeroInput> {
    if x.is_nan() {
        return Err(embedded_invsqrt_f32::NegativeOrZeroInput);
    }
    embedded_inv_sqrt(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_and_positive_values() {
        let r4 = inv_sqrt(4.0).unwrap();
        assert!((r4 - 0.5).abs() < 1e-3);

        let r1 = inv_sqrt(1.0).unwrap();
        assert!((r1 - 1.0).abs() < 1e-3);
    }

    #[test]
    fn error_and_special_cases() {
        assert!(inv_sqrt(-1.0).is_err());
        assert!(inv_sqrt(f32::NAN).is_err());
    }
}