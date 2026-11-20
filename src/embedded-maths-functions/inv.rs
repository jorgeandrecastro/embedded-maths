// SPDX-License-Identifier: GPL-2.0-or-later
//! Reciprocal operations for `f32`.

/// Returns the multiplicative inverse of `x`.
#[inline(always)]
pub fn inv(x: f32) -> f32 {
    1.0 / x
}