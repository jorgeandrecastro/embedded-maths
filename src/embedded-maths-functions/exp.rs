// SPDX-License-Identifier: GPL-2.0-or-later
//! Exponential function `e^x` for `f32` on `no_std` embedded targets,
//! powered by `embedded-exp-f32`.
//!
//! Uses Cody-Waite range reduction, a `(2, 2)` Padé approximation over the
//! reduced interval, and IEEE 754 exponent reconstruction without `libm`,
//! heap allocation, or loops over `k`.
//!
//! # Behavior & Notes
//!
//! | Input / Condition | Behavior |
//! |-------------------|----------|
//! | `x = 0.0` | Returns `1.0` |
//! | `x = 1.0` | Returns `~e` (`core::f32::consts::E`) |
//! | `x <= -87.33654` | Underflows to `0.0` |
//! | `x >= 88.72284` | Overflows to `f32::INFINITY` |
//! | `fast-path` feature | Skips explicit bounds/NaN checks when `x` is already validated |

pub use embedded_exp_f32::exp_f32;

/// Computes `e^x` in `f32` without `libm` or `std`.
pub fn exp(x: f32) -> f32 {
    exp_f32(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_and_known_values() {
        assert_eq!(exp(0.0), 1.0);
        assert!((exp(1.0) - core::f32::consts::E).abs() < 1e-4);
        assert!((exp(-1.0) - 1.0 / core::f32::consts::E).abs() < 1e-4);
    }

    #[test]
    fn bounds_and_overflow_underflow() {
        assert_eq!(exp(-100.0), 0.0);
        assert_eq!(exp(100.0), f32::INFINITY);
        assert!(exp(f32::NAN).is_nan());
    }
}
