// SPDX-License-Identifier: GPL-2.0-or-later
//! `f32` rounding operations.

/// Returns `x` rounded to the nearest integer.
///
/// Values exactly halfway between integers round away from zero.
#[inline(always)]
pub fn round(x: f32) -> f32 {
    if x.is_nan() || x.is_infinite() {
        return x;
    }
    let integer = super::trunc::trunc(x);
    let fraction = x - integer;
    let magnitude = if fraction < 0.0 { -fraction } else { fraction };
    if magnitude >= 0.5 {
        if fraction > 0.0 {
            integer + 1.0
        } else {
            integer - 1.0
        }
    } else {
        integer
    }
}

#[cfg(test)]
mod tests {
    use super::round;

    #[test]
    fn nearest_integer() {
        assert_eq!(round(1.2), 1.0);
        assert_eq!(round(1.8), 2.0);
        assert_eq!(round(-1.5), -2.0);
        assert_eq!(round(-1.4), -1.0);
    }
}