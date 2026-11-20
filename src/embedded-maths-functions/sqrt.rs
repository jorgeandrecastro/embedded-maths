// SPDX-License-Identifier: GPL-2.0-or-later
//! Square root of `f32` for `no_std`, powered by `embedded-f32-sqrt`.
//!
//! This module calls [`embedded_f32_sqrt::sqrt`] rather than relying on
//! `micromath` or floating-point methods that require `std`. It uses
//! IEEE 754 bit manipulation and Newton-Raphson iterations and is suitable
//! for embedded targets with or without an FPU.
//!
//! # Behavior
//!
//! | Input | Result |
//! |-------|--------|
//! | `>= 0.0` | `Ok(square_root)` |
//! | `-0.0` | `Ok(-0.0)` |
//! | Negative value | `Err(NegativeInput)` |
//! | `NaN` | `Ok(NaN)` |
//! | `+Infinity` | `Ok(+Infinity)` |

pub use embedded_f32_sqrt::NegativeInput;

use embedded_f32_sqrt::sqrt as embedded_sqrt;

/// Computes the square root of `x`.
///
/// Calls [`embedded_f32_sqrt::sqrt`] for the implementation and preserves the
/// sign of negative zero.
pub fn sqrt(x: f32) -> Result<f32, NegativeInput> {
    if x.to_bits() == (-0.0f32).to_bits() {
        return Ok(-0.0);
    }

    embedded_sqrt(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_squares() {
        for &(value, expected) in &[(0.0, 0.0), (1.0, 1.0), (4.0, 2.0), (9.0, 3.0), (25.0, 5.0)] {
            assert_eq!(sqrt(value).unwrap(), expected);
        }
    }

    #[test]
    fn irrational_values() {
        let result = sqrt(2.0).unwrap();
        assert!((result as f64 - (2.0f64).sqrt()).abs() <= 1e-6);
    }

    #[test]
    fn negative_input() {
        assert_eq!(sqrt(-1.0), Err(NegativeInput));
    }

    #[test]
    fn zero_sign_and_special_values() {
        assert_eq!(sqrt(-0.0).unwrap().to_bits(), (-0.0f32).to_bits());
        assert!(sqrt(f32::NAN).unwrap().is_nan());
        assert_eq!(sqrt(f32::INFINITY).unwrap(), f32::INFINITY);
    }
}
