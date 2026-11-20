// SPDX-License-Identifier: GPL-2.0-or-later
//! `f32` floor operations.

/// Returns the largest integer less than or equal to `x`.
#[inline(always)]
pub fn floor(x: f32) -> f32 {
    if x.is_nan() || x.is_infinite() {
        return x;
    }
    let integer = super::trunc::trunc(x);
    if x == integer || x == 0.0 {
        integer
    } else if x < 0.0 {
        integer - 1.0
    } else {
        integer
    }
}

#[cfg(test)]
mod tests {
    use super::floor;

    #[test]
    fn positive_negative_and_special_values() {
        assert_eq!(floor(1.2), 1.0);
        assert_eq!(floor(-1.2), -2.0);
        assert_eq!(floor(-3.0), -3.0);
        assert_eq!(floor(f32::NEG_INFINITY), f32::NEG_INFINITY);
        assert!(floor(f32::NAN).is_nan());
    }
}