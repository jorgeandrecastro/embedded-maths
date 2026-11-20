// SPDX-License-Identifier: GPL-2.0-or-later
//! `f32` floor operations.

/// Returns the largest integer less than or equal to `x`.
#[inline(always)]
pub fn floor(x: f32) -> f32 {
    x.floor()
}