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