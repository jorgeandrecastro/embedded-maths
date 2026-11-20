// SPDX-License-Identifier: GPL-2.0-or-later
//! Reciprocal operations for `f32`.

/// Returns the multiplicative inverse of `x`.
#[inline(always)]
pub fn inv(x: f32) -> f32 {
    1.0 / x
}

#[cfg(test)]
mod tests {
    use super::inv;

    #[test]
    fn known_values() {
        assert_eq!(inv(4.0), 0.25);
        assert_eq!(inv(-2.0), -0.5);
        assert_eq!(inv(1.0), 1.0);
    }

    #[test]
    fn zero_and_infinity() {
        assert_eq!(inv(0.0), f32::INFINITY);
        assert_eq!(inv(f32::INFINITY), 0.0);
    }
}