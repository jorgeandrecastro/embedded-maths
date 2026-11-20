//! exp et cosh pour `f32`, sans dépendance externe.
//!
//! exp : x = k·ln2 + r avec |r| ≤ ln2/2, donc exp(x) = 2^k · exp(r),
//! exp(r) par série de Taylor (Horner, degré 7, erreur relative ~1e-7).
//! cosh(x) = (e^x + e^-x) / 2, avec traitement spécial des grands |x|.

use core::f32::consts::{LN_2, LOG2_E};

// ln(2) découpé en deux constantes (Cody-Waite) pour limiter l'erreur de réduction.
const LN2_HI: f32 = 6.931_457_5e-1;
const LN2_LO: f32 = 1.428_606_8e-6;

/// 2^k exact pour k ∈ [-126, 127], construit par les bits.
#[inline]
fn pow2(k: i32) -> f32 {
    f32::from_bits(((k + 127) as u32) << 23)
}

/// Exponentielle de `x`.
///
/// - NaN → NaN, `x > ~88,72` → +∞, `x < -104` → 0
pub fn exp(x: f32) -> f32 {
    if x.is_nan() {
        return f32::NAN;
    }
    if x > 88.722_84 {
        return f32::INFINITY;
    }
    if x < -104.0 {
        return 0.0;
    }

    // k = arrondi(x / ln2), sans `round()` (indisponible en `no_std`).
    let t = x * LOG2_E;
    let k = (if t >= 0.0 { t + 0.5 } else { t - 0.5 }) as i32;
    let kf = k as f32;
    let r = (x - kf * LN2_HI) - kf * LN2_LO;

    // 1 + r + r²/2 + r³/6 + r⁴/24 + r⁵/120 + r⁶/720 + r⁷/5040
    let p = 1.0
        + r * (1.0
            + r * (1.0 / 2.0
                + r * (1.0 / 6.0
                    + r * (1.0 / 24.0
                        + r * (1.0 / 120.0 + r * (1.0 / 720.0 + r * (1.0 / 5040.0)))))));

    // 2^k en deux facteurs pour couvrir k ∈ [-150, 128] sans dépasser l'exposant.
    let k1 = k / 2;
    p * pow2(k1) * pow2(k - k1)
}

/// Cosinus hyperbolique de `x`.
///
/// - NaN → NaN, |x| > ~89,41 → +∞
pub fn cosh(x: f32) -> f32 {
    let a = if x < 0.0 { -x } else { x };
    if a > 88.0 {
        // e^a / 2 = e^(a - ln2) : évite le dépassement intermédiaire de e^a.
        return exp(a - LN_2);
    }
    (exp(a) + exp(-a)) * 0.5
}

#[cfg(test)]
mod tests {
    use super::{cosh, exp};
    use core::f32::consts::{E, PI};

    const REL_TOL: f32 = 5e-6;

    fn rel_close(a: f32, b: f32) -> bool {
        ((a - b) / b).abs() <= REL_TOL
    }

    #[test]
    fn exp_known_values() {
        assert_eq!(exp(0.0), 1.0);
        let cases: &[(f32, f32)] = &[
            (1.0, E),
            (-1.0, 0.367_879_44),
            (0.5, 1.648_721_3),
            (10.0, 22_026.466),
            (-10.0, 4.539_993e-5),
            (88.0, 1.651_636_3e38),
        ];
        for &(x, expected) in cases {
            let got = exp(x);
            assert!(rel_close(got, expected), "exp({x}) = {got}, attendu {expected}");
        }
    }

    #[test]
    fn exp_limits() {
        assert_eq!(exp(100.0), f32::INFINITY);
        assert_eq!(exp(-200.0), 0.0);
        assert_eq!(exp(f32::INFINITY), f32::INFINITY);
        assert_eq!(exp(f32::NEG_INFINITY), 0.0);
        assert!(exp(f32::NAN).is_nan());
    }

    #[test]
    fn exp_addition_law() {
        // exp(a + b) = exp(a) · exp(b)
        for i in -40..=40 {
            let a = i as f32 * 0.37;
            let b = 1.234;
            assert!(rel_close(exp(a + b), exp(a) * exp(b)), "a={a}");
        }
    }

    #[test]
    fn cosh_known_values() {
        assert_eq!(cosh(0.0), 1.0);
        let cases: &[(f32, f32)] = &[
            (1.0, 1.543_080_6),
            (2.0, 3.762_195_7),
            (3.0, 10.067_662),
            (5.0, 74.209_95),
            (10.0, 11_013.233),
            (PI, 11.591_953),
        ];
        for &(x, expected) in cases {
            let got = cosh(x);
            assert!(rel_close(got, expected), "cosh({x}) = {got}, attendu {expected}");
            assert!(rel_close(cosh(-x), expected), "cosh(-{x})");
        }
    }

    #[test]
    fn cosh_at_least_one_and_even() {
        for i in -200..=200 {
            let x = i as f32 * 0.31;
            assert!(cosh(x) >= 1.0, "x={x}");
            assert_eq!(cosh(x), cosh(-x), "x={x}");
        }
    }

    #[test]
    fn cosh_large_values() {
        assert!(cosh(89.0).is_finite()); // exp(89) seul déborderait
        assert_eq!(cosh(90.0), f32::INFINITY);
        assert_eq!(cosh(f32::NEG_INFINITY), f32::INFINITY);
        assert!(cosh(f32::NAN).is_nan());
    }
}