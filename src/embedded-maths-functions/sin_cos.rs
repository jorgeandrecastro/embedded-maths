//! sin, cos et sin_cos pour `f32`, sans dépendance externe.
//!
//! Méthode :
//!   1. k = arrondi(x · 2/π), r = x - k·π/2 ∈ [-π/4, π/4]
//!      (π/2 découpé en trois constantes pour limiter la perte de précision)
//!   2. sin(r) et cos(r) par séries de Taylor évaluées avec Horner
//!   3. le quadrant (k mod 4) permute et change le signe des résultats

// π/2 découpé en trois parties (Cody-Waite) : les deux premières ont peu de
// bits significatifs, donc k·PIO2_x est exact pour k modéré.
const PIO2_1: f32 = 1.570_312_5;
const PIO2_2: f32 = 4.837_512_969_970_703e-4;
const PIO2_3: f32 = 7.549_789_948_768_648e-8;

/// Sinus et cosinus de `r` pour |r| ≤ π/4.
#[inline]
fn kernel(r: f32) -> (f32, f32) {
    let r2 = r * r;

    // sin(r) = r · (1 - r²/6 + r⁴/120 - r⁶/5040 + r⁸/362880)
    let sin = r
        * (1.0
            + r2 * (-1.0 / 6.0
                + r2 * (1.0 / 120.0 + r2 * (-1.0 / 5040.0 + r2 * (1.0 / 362_880.0)))));

    // cos(r) = 1 - r²/2 + r⁴/24 - r⁶/720 + r⁸/40320 - r¹⁰/3628800
    let cos = 1.0
        + r2 * (-0.5
            + r2 * (1.0 / 24.0
                + r2 * (-1.0 / 720.0 + r2 * (1.0 / 40_320.0 + r2 * (-1.0 / 3_628_800.0)))));

    (sin, cos)
}

/// Calcule `(sin(x), cos(x))` en une seule réduction d'argument.
///
/// - `x` infini ou NaN → `(NaN, NaN)`
/// - précision de l'ordre de 1e-6 pour |x| ≲ 8000 ; elle se dégrade au-delà.
pub fn sin_cos(x: f32) -> (f32, f32) {
    if !x.is_finite() {
        return (f32::NAN, f32::NAN);
    }

    // Arrondi au plus proche sans `round()` (indisponible en `no_std`).
    let t = x * core::f32::consts::FRAC_2_PI;
    let k = (if t >= 0.0 { t + 0.5 } else { t - 0.5 }) as i32;
    let kf = k as f32;

    let r = ((x - kf * PIO2_1) - kf * PIO2_2) - kf * PIO2_3;
    let (s, c) = kernel(r);

    match k & 3 {
        0 => (s, c),
        1 => (c, -s),
        2 => (-s, -c),
        _ => (-c, s),
    }
}

/// Sinus de `x`.
pub fn sin(x: f32) -> f32 {
    sin_cos(x).0
}

/// Cosinus de `x`.
pub fn cos(x: f32) -> f32 {
    sin_cos(x).1
}

#[cfg(test)]
mod tests {
    use super::{cos, sin, sin_cos};
    use core::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, PI, TAU};

    const TOL: f32 = 2e-6;

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() <= TOL
    }

    #[test]
    fn known_angles() {
        let cases: &[(f32, f32, f32)] = &[
            (0.0, 0.0, 1.0),
            (FRAC_PI_6, 0.5, 0.866_025_4),
            (FRAC_PI_4, FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            (FRAC_PI_3, 0.866_025_4, 0.5),
            (FRAC_PI_2, 1.0, 0.0),
            (PI, 0.0, -1.0),
            (3.0 * FRAC_PI_2, -1.0, 0.0),
            (TAU, 0.0, 1.0),
        ];
        for &(x, es, ec) in cases {
            let (s, c) = sin_cos(x);
            assert!(close(s, es), "sin({x}) = {s}, attendu {es}");
            assert!(close(c, ec), "cos({x}) = {c}, attendu {ec}");
        }
    }

    #[test]
    fn pythagorean_identity() {
        // ~46 points couvrant [0, 2π] et au-delà, sans table de vecteurs.
        for i in -200..=200 {
            let x = i as f32 * 0.137;
            let (s, c) = sin_cos(x);
            assert!((s * s + c * c - 1.0).abs() <= 4e-6, "x={x}: {}", s * s + c * c);
        }
    }

    #[test]
    fn parity() {
        for i in 1..=100 {
            let x = i as f32 * 0.173;
            assert!(close(sin(-x), -sin(x)), "sin impair, x={x}");
            assert!(close(cos(-x), cos(x)), "cos pair, x={x}");
        }
    }

    #[test]
    fn periodicity() {
        for i in 0..=100 {
            let x = i as f32 * 0.061;
            assert!((sin(x + TAU) - sin(x)).abs() <= 5e-6, "x={x}");
            assert!((cos(x + TAU) - cos(x)).abs() <= 5e-6, "x={x}");
        }
    }

    #[test]
    fn phase_shift() {
        // sin(x + π/2) = cos(x)
        for i in 0..=100 {
            let x = i as f32 * 0.063;
            assert!((sin(x + FRAC_PI_2) - cos(x)).abs() <= 5e-6, "x={x}");
        }
    }

    #[test]
    fn special_values() {
        assert!(sin(f32::NAN).is_nan());
        assert!(cos(f32::INFINITY).is_nan());
        assert!(sin_cos(f32::NEG_INFINITY).0.is_nan());
    }
}