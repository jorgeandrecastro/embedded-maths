// SPDX-License-Identifier: GPL-2.0-or-later
//! `f32` sign-copy operations.

/// Returns `magnitude` with the sign of `sign`.
#[inline(always)]
pub fn copysign(magnitude: f32, sign: f32) -> f32 {
    f32::from_bits((magnitude.to_bits() & !(1 << 31)) | (sign.to_bits() & (1 << 31)))
}

#[cfg(test)]
mod tests {
    use super::copysign;

    #[test]
    fn copies_sign_and_zero() {
        assert_eq!(copysign(3.5, -1.0), -3.5);
        assert_eq!(copysign(-3.5, 1.0), 3.5);
        assert_eq!(copysign(0.0, -0.0).to_bits(), (-0.0f32).to_bits());
    }
}
