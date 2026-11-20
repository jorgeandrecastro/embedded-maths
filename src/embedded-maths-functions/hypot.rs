// SPDX-License-Identifier: GPL-2.0-or-later
//! Overflow-safe Euclidean norm for two `f32` values.

use embedded_f32_sqrt::sqrt;

/// Returns `sqrt(x * x + y * y)` without avoidable overflow or underflow.
pub fn hypot(x: f32, y: f32) -> f32 {
    let a = x.abs();
    let b = y.abs();
    if a == 0.0 && b == 0.0 {
        return 0.0;
    }
    if a.is_infinite() || b.is_infinite() {
        return f32::INFINITY;
    }
    if a.is_nan() || b.is_nan() {
        return f32::NAN;
    }

    let (high, low) = if a >= b { (a, b) } else { (b, a) };
    let ratio = low / high;
    let factor = sqrt(1.0 + ratio * ratio).unwrap_or(1.0);
    high * factor
}

#[cfg(test)]
mod tests {
    use super::hypot;

    #[test]
    fn known_and_symmetric_values() {
        assert_eq!(hypot(3.0, 4.0), 5.0);
        assert_eq!(hypot(-3.0, 4.0), 5.0);
        assert_eq!(hypot(0.0, 0.0), 0.0);
    }

    #[test]
    fn special_values() {
        assert_eq!(hypot(f32::INFINITY, 1.0), f32::INFINITY);
        assert!(hypot(f32::NAN, 1.0).is_nan());
    }
}
