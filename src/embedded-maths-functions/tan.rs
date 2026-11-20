//! Tangente pour `f32`, sans dépendance externe.
//!
//! tan(x) = sin(x) / cos(x), avec une seule réduction d'argument grâce à `sin_cos`.
//! Erreur relative de l'ordre de 1e-5 loin des pôles (contre 0,6 pour l'ancienne version).

use super::sin_cos::sin_cos; // adapte le chemin à ton arborescence

/// Tangente de `x` (en radians).
///
/// - `x` infini ou NaN → NaN
/// - près de π/2 + kπ, le résultat est très grand (cos(x) tend vers 0)
pub fn tan(x: f32) -> f32 {
    let (s, c) = sin_cos(x);
    s / c
}

#[cfg(test)]
mod tests {
    use super::tan;
    use core::f32::consts::{FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, PI};

    const REL_TOL: f32 = 5e-5;

    fn rel_close(a: f32, b: f32) -> bool {
        (a - b).abs() <= REL_TOL * b.abs().max(1.0)
    }

    #[test]
    fn zero() {
        assert_eq!(tan(0.0), 0.0);
    }

    #[test]
    fn known_angles() {
        assert!(rel_close(tan(FRAC_PI_6), 0.577_350_3));
        assert!(rel_close(tan(FRAC_PI_4), 1.0));
        assert!(rel_close(tan(FRAC_PI_3), 1.732_050_8));
        assert!(tan(PI).abs() <= 1e-5);
    }

    #[test]
    fn known_values() {
        // Valeurs de référence indépendantes de notre implémentation.
        let cases: &[(f32, f32)] = &[
            (0.5, 0.546_302_5),
            (1.0, 1.557_407_7),
            (2.0, -2.185_04),
            (3.0, -0.142_546_54),
            (4.0, 1.157_821_3),
            (5.0, -3.380_514),
            (6.0, -0.291_006_2),
        ];
        for &(x, expected) in cases {
            let got = tan(x);
            assert!(
                rel_close(got, expected),
                "tan({x}) = {got}, attendu {expected}"
            );
        }
    }

    #[test]
    fn odd_symmetry() {
        for i in 1..=100 {
            let x = i as f32 * 0.0137; // reste loin de π/2
            assert!(rel_close(tan(-x), -tan(x)), "x={x}");
        }
    }

    #[test]
    fn period_pi() {
        for i in 0..=100 {
            let x = i as f32 * 0.0113; // x ∈ [0, 1.13]
            assert!(rel_close(tan(x + PI), tan(x)), "x={x}");
        }
    }

    #[test]
    fn sign_by_quadrant() {
        assert!(tan(1.0) > 0.0); // Q1
        assert!(tan(2.0) < 0.0); // Q2
        assert!(tan(4.0) > 0.0); // Q3
        assert!(tan(5.0) < 0.0); // Q4
    }

    #[test]
    fn special_values() {
        assert!(tan(f32::NAN).is_nan());
        assert!(tan(f32::INFINITY).is_nan());
        assert!(tan(f32::NEG_INFINITY).is_nan());
    }
}
