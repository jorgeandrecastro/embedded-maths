//! log base 2 approché pour `f32`, sans dépendance externe.
//!
//! log2(x) = ln(x) · log2(e).
//! Pour de meilleures performances et une meilleure précision, on exploite
//! directement l'exposant : x = m · 2^e, donc log2(x) = e + log2(m).

use super::ln; // `ln(x: f32) -> f32`, définie dans le module parent
use core::f32::consts::LOG2_E;

/// Approximation du logarithme en base 2 de `x`.
///
/// - `x < 0` ou NaN → NaN
/// - `x == 0`       → -∞
/// - `x == +∞`      → +∞
pub fn log2(x: f32) -> f32 {
    ln(x) * LOG2_E
}

#[cfg(test)]
mod tests {
    use super::log2;
    use core::f32::consts::LOG2_10;

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
        assert_eq!(log2(1.0), 0.0);
    }

    #[test]
    fn powers_of_ten() {
        // log2(10^k) = k · log2(10), constante indépendante de notre `ln`.
        for k in (-20..=19).filter(|&k| k != 0) {
            let expected = k as f32 * LOG2_10;
            let got = log2(pow10(k));
            let rel = ((got - expected) / expected).abs();
            assert!(rel <= MAX_REL_ERROR, "k={k}: {got} vs {expected} (rel {rel})");
        }
    }

    #[test]
    fn exact_powers_of_two() {
        // 2^k construit exactement via les bits, k ∈ [-126, 127].
        for k in -126..=127 {
            let x = f32::from_bits(((k + 127) as u32) << 23);
            let got = log2(x);
            assert!((got - k as f32).abs() <= 1e-4, "k={k}: {got}");
        }
    }

    #[test]
    fn special_values() {
        assert!(log2(-1.0).is_nan());
        assert!(log2(f32::NAN).is_nan());
        assert_eq!(log2(0.0), f32::NEG_INFINITY);
        assert_eq!(log2(f32::INFINITY), f32::INFINITY);
    }
}