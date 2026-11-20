// SPDX-License-Identifier: GPL-2.0-or-later
//! `f32` rounding operations.

/// Returns `x` rounded to the nearest integer.
///
/// Values exactly halfway between integers round away from zero.
#[inline(always)]
pub fn round(x: f32) -> f32 {
    x.round()
}