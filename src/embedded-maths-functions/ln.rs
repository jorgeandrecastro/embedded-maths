//! Logarithme népérien approché pour `f32`, sans dépendance externe.
//!
//! Méthode : x = m · 2^e avec m ∈ [√2/2, √2), donc
//!   ln(x) = e·ln(2) + ln(m)
//!   ln(m) = 2·atanh(s) = 2·(s + s³/3 + s⁵/5 + s⁷/7 + s⁹/9 + …),  s = (m-1)/(m+1)
//! Avec |s| ≤ 0,1716, cinq termes suffisent à la précision d'un `f32`.

use core::f32::consts::{LN_2, SQRT_2};

/// Approximation du logarithme népérien de `x`.
///
/// - `x < 0` ou NaN → NaN
/// - `x == 0`       → -∞
/// - `x == +∞`      → +∞
/// - gère les dénormalisés
pub fn ln(x: f32) -> f32 {
    if x.is_nan() || x < 0.0 {
        return f32::NAN;
    }
    if x == 0.0 {
        return f32::NEG_INFINITY;
    }
    if x == f32::INFINITY {
        return x;
    }

    let mut bits = x.to_bits();
    let mut exp: i32 = 0;

    // Nombre dénormalisé : on le remet à l'échelle (× 2^24).
    if bits < 0x0080_0000 {
        bits = (x * 16_777_216.0).to_bits();
        exp = -24;
    }

    exp += ((bits >> 23) & 0xFF) as i32 - 127;

    // Mantisse normalisée dans [1, 2)
    let mut m = f32::from_bits((bits & 0x007F_FFFF) | 0x3F80_0000);

    // Recentrage dans [√2/2, √2) pour minimiser |s|
    if m > SQRT_2 {
        m *= 0.5;
        exp += 1;
    }

    let s = (m - 1.0) / (m + 1.0);
    let s2 = s * s;

    // 2·s·(1 + s²/3 + s⁴/5 + s⁶/7 + s⁸/9), évalué par Horner
    let series = 1.0 + s2 * (1.0 / 3.0 + s2 * (1.0 / 5.0 + s2 * (1.0 / 7.0 + s2 * (1.0 / 9.0))));
    let ln_m = 2.0 * s * series;

    exp as f32 * LN_2 + ln_m
}

#[cfg(test)]
mod tests {
    use super::ln;
    use core::f32::consts::LN_10;

    const MAX_REL_ERROR: f32 = 1e-5;

    #[test]
    fn one_is_exactly_zero() {
        assert_eq!(ln(1.0), 0.0);
    }

    #[test]
    fn powers_of_ten() {
        // ln(10^k) = k · ln(10)
        for k in -37..=38 {
            if k == 0 {
                continue;
            }
            let x = 10f32.powi(k);
            let expected = k as f32 * LN_10;
            let got = ln(x);
            let rel = ((got - expected) / expected).abs();
            assert!(
                rel <= MAX_REL_ERROR,
                "k={k}: {got} vs {expected} (rel {rel})"
            );
        }
    }

    #[test]
    fn subnormal() {
        let got = ln(1e-40);
        let expected = -40.0 * LN_10;
        assert!(((got - expected) / expected).abs() <= MAX_REL_ERROR);
    }

    #[test]
    fn special_values() {
        assert!(ln(-1.0).is_nan());
        assert!(ln(f32::NAN).is_nan());
        assert_eq!(ln(0.0), f32::NEG_INFINITY);
        assert_eq!(ln(f32::INFINITY), f32::INFINITY);
    }
}
