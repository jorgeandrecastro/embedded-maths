// SPDX-License-Identifier: GPL-2.0-or-later
//! Euclidean remainder for `f32`.

/// Calculates the least non-negative remainder of `x / y`.
pub fn rem_euclid(x: f32, y: f32) -> f32 {
    if y == 0.0 {
        return f32::NAN;
    }
    let mut r = x - (x / y).trunc() * y;
    if r < 0.0 {
        r += y.abs();
    }
    if r == 0.0 {
        r.copysign(y)
    } else {
        r
    }
}