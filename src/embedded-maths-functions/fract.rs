// SPDX-License-Identifier: GPL-2.0-or-later
//! `f32` fractional-part operations.

/// Returns the fractional part of `x`.
#[inline(always)]
pub fn fract(x: f32) -> f32 {
    if x.is_nan() || x.is_infinite() {
        return f32::NAN;
    }
    x - super::trunc::trunc(x)
}

#[cfg(test)]
mod tests {
    use super::fract;

    #[test]
    fn fractional_parts() {
        assert_eq!(fract(3.25), 0.25);
        assert_eq!(fract(-3.25), -0.25);
        assert_eq!(fract(4.0), 0.0);
    }
}
