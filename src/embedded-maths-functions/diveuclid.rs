// SPDX-License-Identifier: GPL-2.0-or-later
//! Euclidean division for `f32`.

/// Calculates the quotient of Euclidean division `x / y`.
pub fn div_euclid(x: f32, y: f32) -> f32 {
    if y == 0.0 {
        return f32::NAN;
    }

    let mut quotient = (x / y).trunc();
    let remainder = x - quotient * y;
    if remainder < 0.0 {
        quotient -= 1.0;
    }
    quotient
}