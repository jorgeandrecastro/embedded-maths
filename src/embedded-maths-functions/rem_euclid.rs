// SPDX-License-Identifier: GPL-2.0-or-later
//! Euclidean remainder for `f32`.

/// Calculates the least non-negative remainder of `x / y`.
pub fn rem_euclid(x: f32, y: f32) -> f32 {
    if y == 0.0 {
        return f32::NAN;
    }
    let mut r = x - super::trunc::trunc(x / y) * y;
    let y_magnitude = f32::from_bits(y.to_bits() & !(1 << 31));
    if r < 0.0 {
        r += y_magnitude;
    }
    if r == 0.0 {
        f32::from_bits((r.to_bits() & !(1 << 31)) | (y.to_bits() & (1 << 31)))
    } else {
        r
    }
}

#[cfg(test)]
mod tests {
    use super::rem_euclid;

    #[test]
    fn least_nonnegative_remainder() {
        assert_eq!(rem_euclid(5.0, 2.0), 1.0);
        assert_eq!(rem_euclid(-5.0, 2.0), 1.0);
        assert_eq!(rem_euclid(5.0, -2.0), 1.0);
    }

    #[test]
    fn zero_divisor() {
        assert!(rem_euclid(1.0, 0.0).is_nan());
    }
}
