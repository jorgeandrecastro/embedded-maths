// SPDX-License-Identifier: GPL-2.0-or-later
//! Fused multiply-add operations for `f32`.

/// Returns `a * b + c` with a fused operation when supported.
#[inline(always)]
pub fn mul_add(a: f32, b: f32, c: f32) -> f32 {
    a.mul_add(b, c)
}