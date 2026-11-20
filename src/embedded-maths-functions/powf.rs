// SPDX-License-Identifier: GPL-2.0-or-later
//! Floating-point power for `f32`.

use super::exp::exp;
use super::ln::ln;

/// Returns `base` raised to the floating-point power `exponent`.
pub fn powf(base: f32, exponent: f32) -> f32 {
    if base == 1.0 {
        return 1.0;
    }
    if exponent == 0.0 {
        return 1.0;
    }
    if base < 0.0 {
        return f32::NAN;
    }
    if exponent.is_nan() {
        return f32::NAN;
    }
    if base.is_infinite() {
        return if exponent > 0.0 { f32::INFINITY } else { 0.0 };
    }
    exp(exponent * ln(base))
}

#[cfg(test)]
mod tests {
    use super::powf;

    #[test]
    fn known_values() {
        assert_eq!(powf(2.0, 3.0) as i32, 8);
        assert_eq!(powf(9.0, 0.5) as i32, 3);
        assert_eq!(powf(7.0, 0.0), 1.0);
        assert_eq!(powf(1.0, 123.0), 1.0);
    }

    #[test]
    fn special_values() {
        assert!(powf(-2.0, 0.5).is_nan());
        assert!(powf(2.0, f32::NAN).is_nan());
        assert_eq!(powf(f32::INFINITY, 2.0), f32::INFINITY);
    }
}