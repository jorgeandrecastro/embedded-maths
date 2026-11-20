//! Sinus pour `f32`, sans dépendance externe.
//!
//! Utilise la réduction d'argument et les polynômes de `sin_cos`
//! (erreur de l'ordre de 1e-6, contre 2e-3 pour l'ancienne version via `cos`).

use super::sin_cos::sin_cos; // adapte le chemin à ton arborescence

/// Sinus de `x` (en radians).
///
/// - `x` infini ou NaN → NaN
pub fn sin(x: f32) -> f32 {
    sin_cos(x).0
}

#[cfg(test)]
mod tests {
    use super::sin;
    use core::f32::consts::{FRAC_PI_2, FRAC_PI_6, PI, TAU};

    const TOL: f32 = 2e-6;

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() <= TOL
    }

    #[test]
    fn known_angles() {
        assert_eq!(sin(0.0), 0.0);
        assert!(close(sin(FRAC_PI_6), 0.5));
        assert!(close(sin(FRAC_PI_2), 1.0));
        assert!(close(sin(PI), 0.0));
        assert!(close(sin(3.0 * FRAC_PI_2), -1.0));
        assert!(close(sin(TAU), 0.0));
    }

    #[test]
    fn known_values() {
        // Valeurs de référence connues, indépendantes de notre implémentation.
        let cases: &[(f32, f32)] = &[
            (0.5, 0.479_425_5),
            (1.0, 0.841_470_96),
            (2.0, 0.909_297_4),
            (3.0, 0.141_120_01),
            (4.0, -0.756_802_5),
            (5.0, -0.958_924_3),
            (6.0, -0.279_415_5),
        ];
        for &(x, expected) in cases {
            let got = sin(x);
            assert!(close(got, expected), "sin({x}) = {got}, attendu {expected}");
        }
    }

    #[test]
    fn odd_symmetry() {
        for i in 1..=100 {
            let x = i as f32 * 0.173;
            assert!(close(sin(-x), -sin(x)), "x={x}");
        }
    }

    #[test]
    fn periodicity() {
        for i in 0..=100 {
            let x = i as f32 * 0.061;
            assert!((sin(x + TAU) - sin(x)).abs() <= 5e-6, "x={x}");
        }
    }

    #[test]
    fn bounded() {
        for i in -500..=500 {
            let x = i as f32 * 0.0731;
            assert!(sin(x).abs() <= 1.0 + 1e-6, "x={x}");
        }
    }

    #[test]
    fn special_values() {
        assert!(sin(f32::NAN).is_nan());
        assert!(sin(f32::INFINITY).is_nan());
        assert!(sin(f32::NEG_INFINITY).is_nan());
    }
}
