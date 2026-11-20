//! log_b(x) approché pour `f32`, sans dépendance externe.
//!
//! Formule du changement de base : log_b(x) = ln(x) / ln(b).

use super::ln; // `ln(x: f32) -> f32`, définie dans le module parent

/// Approximation du logarithme de `x` dans la base `base`.
///
/// - `x < 0`, `base < 0` ou NaN → NaN
/// - `base == 1`               → ±∞, ou NaN si x == 1
/// - `x == 0`                  → -∞ si base > 1, +∞ si 0 < base < 1
pub fn log(x: f32, base: f32) -> f32 {
    ln(x) / ln(base)
}

#[cfg(test)]
mod tests {
    use super::log;
    use core::f32::consts::LN_10;

    const MAX_REL_ERROR: f32 = 1e-5;

    // Valeurs de référence indépendantes de notre `ln`.
    const LN_3: f32 = 1.098_612_3;
    const LN_5_5: f32 = 1.704_748_1;
    const LN_12_7: f32 = 2.541_602;

    /// 10^k par multiplications successives (pas de `powi` en `no_std`).
    fn pow10(k: i32) -> f32 {
        let mut r = 1.0f32;
        for _ in 0..k.abs() {
            r *= 10.0;
        }
        if k < 0 { 1.0 / r } else { r }
    }

    /// Vérifie log_base(10^k) = k · ln(10) / ln(base) pour k ∈ [-20, 19].
    fn check_powers_of_ten(base: f32, ln_base: f32) {
        for k in (-20..=19).filter(|&k| k != 0) {
            let expected = k as f32 * LN_10 / ln_base;
            let got = log(pow10(k), base);
            let rel = ((got - expected) / expected).abs();
            assert!(
                rel <= MAX_REL_ERROR,
                "base={base}, k={k}: {got} vs {expected} (rel {rel})"
            );
        }
    }

    #[test]
    fn one_is_zero() {
        for base in [3.0, 5.5, 12.7] {
            assert_eq!(log(1.0, base), 0.0);
        }
    }

    #[test]
    fn base_3() {
        check_powers_of_ten(3.0, LN_3);
    }

    #[test]
    fn base_5_5() {
        check_powers_of_ten(5.5, LN_5_5);
    }

    #[test]
    fn base_12_7() {
        check_powers_of_ten(12.7, LN_12_7);
    }

    #[test]
    fn exact_powers_of_two() {
        // log_2(2^k) = k, avec 2^k construit exactement via les bits.
        for k in -100..=100 {
            let x = f32::from_bits(((k + 127) as u32) << 23);
            let got = log(x, 2.0);
            assert!((got - k as f32).abs() <= 1e-4, "k={k}: {got}");
        }
    }

    #[test]
    fn special_values() {
        assert!(log(-1.0, 3.0).is_nan());
        assert!(log(3.0, -1.0).is_nan());
        assert!(log(f32::NAN, 3.0).is_nan());
        assert_eq!(log(0.0, 3.0), f32::NEG_INFINITY);
        assert_eq!(log(0.0, 0.5), f32::INFINITY);
        assert!(log(5.0, 1.0).is_infinite());
        assert!(log(1.0, 1.0).is_nan());
    }
}