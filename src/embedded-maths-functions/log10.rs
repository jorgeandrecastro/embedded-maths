//! log base 10 approché pour `f32`, sans dépendance externe.
//!
//! log10(x) = ln(x) · log10(e).

use super::ln; // `ln(x: f32) -> f32`, définie dans le module parent
use core::f32::consts::LOG10_E;

/// Approximation du logarithme en base 10 de `x`.
///
/// - `x < 0` ou NaN → NaN
/// - `x == 0`       → -∞
/// - `x == +∞`      → +∞
pub fn log10(x: f32) -> f32 {
    ln(x) * LOG10_E
}

#[cfg(test)]
mod tests {
    use super::log10;

    const MAX_REL_ERROR: f32 = 1e-5;

    /// 10^k par multiplications successives (pas de `powi` en `no_std`).
    fn pow10(k: i32) -> f32 {
        let mut r = 1.0f32;
        for _ in 0..k.abs() {
            r *= 10.0;
        }
        if k < 0 { 1.0 / r } else { r }
    }

    #[test]
    fn one_is_zero() {
        assert_eq!(log10(1.0), 0.0);
    }

    #[test]
    fn powers_of_ten() {
        // log10(10^k) = k
        for k in (-20..=19).filter(|&k| k != 0) {
            let expected = k as f32;
            let got = log10(pow10(k));
            let rel = ((got - expected) / expected).abs();
            assert!(rel <= MAX_REL_ERROR, "k={k}: {got} vs {expected} (rel {rel})");
        }
    }

    #[test]
    fn known_values() {
        // log10(2) et log10(5) : constantes indépendantes de notre `ln`.
        assert!((log10(2.0) - 0.301_03).abs() <= 1e-5);
        assert!((log10(5.0) - 0.698_97).abs() <= 1e-5);
    }

    #[test]
    fn special_values() {
        assert!(log10(-1.0).is_nan());
        assert!(log10(f32::NAN).is_nan());
        assert_eq!(log10(0.0), f32::NEG_INFINITY);
        assert_eq!(log10(f32::INFINITY), f32::INFINITY);
    }
}