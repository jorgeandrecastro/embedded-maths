// SPDX-License-Identifier: GPL-2.0-or-later
//! `f32` ceiling operations.

/// Returns the smallest integer greater than or equal to `x`.
#[inline(always)]
pub fn ceil(x: f32) -> f32 {
    if x.is_nan() || x.is_infinite() {
        return x;
    }
    let integer = super::trunc::trunc(x);
    if x == integer || x == 0.0 {
        integer
    } else if x > 0.0 {
        integer + 1.0
    } else {
        integer
    }
}

#[cfg(test)]
mod tests {
    use super::ceil;

    #[test]
    fn positive_negative_and_special_values() {
        assert_eq!(ceil(1.2), 2.0);
        assert_eq!(ceil(-1.2), -1.0);
        assert_eq!(ceil(3.0), 3.0);
        assert_eq!(ceil(f32::INFINITY), f32::INFINITY);
        assert!(ceil(f32::NAN).is_nan());
    }
}