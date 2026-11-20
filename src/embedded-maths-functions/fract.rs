// SPDX-License-Identifier: GPL-2.0-or-later
//! `f32` fractional-part operations.

/// Returns the fractional part of `x`.
#[inline(always)]
pub fn fract(x: f32) -> f32 {
    x.fract()
}