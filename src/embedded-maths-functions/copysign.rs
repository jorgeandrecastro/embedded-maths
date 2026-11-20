// SPDX-License-Identifier: GPL-2.0-or-later
//! `f32` sign-copy operations.

/// Returns `magnitude` with the sign of `sign`.
#[inline(always)]
pub fn copysign(magnitude: f32, sign: f32) -> f32 {
    f32::from_bits(
        (magnitude.to_bits() & !(1 << 31)) | (sign.to_bits() & (1 << 31)),
    )
}