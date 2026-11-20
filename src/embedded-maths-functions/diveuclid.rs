// SPDX-License-Identifier: GPL-2.0-or-later
//! Euclidean division for `f32`.

/// Calculates the quotient of Euclidean division `x / y`.
pub fn div_euclid(x: f32, y: f32) -> f32 {
    if y == 0.0 {
        return f32::NAN;
    }

    let mut quotient = super::trunc::trunc(x / y);
    let remainder = x - quotient * y;
    if remainder < 0.0 {
        quotient -= 1.0;
    }
    quotient
}

#[cfg(test)]
mod tests {
    use super::div_euclid;

    #[test]
    fn euclidean_quotients() {
        assert_eq!(div_euclid(5.0, 2.0), 2.0);
        assert_eq!(div_euclid(-5.0, 2.0), -3.0);
        assert_eq!(div_euclid(5.0, -2.0), -2.0);
    }

    #[test]
    fn zero_divisor() {
        assert!(div_euclid(1.0, 0.0).is_nan());
    }
}
