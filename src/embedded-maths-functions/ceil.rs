// SPDX-License-Identifier: GPL-2.0-or-later
//! `f32` ceiling operations.

/// Returns the smallest integer greater than or equal to `x`.
#[inline(always)]
pub fn ceil(x: f32) -> f32 {
    x.ceil()
}