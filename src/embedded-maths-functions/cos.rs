//! Cosinus pour `f32`, sans dépendance externe.
//!
//! Délègue à la réduction d'argument et aux polynômes de `sin_cos`
//! (erreur de l'ordre de 1e-6, contre 2e-3 pour l'ancienne version).
//! L'approximation parabolique d'origine et le long commentaire qui
//! la décrivait ne sont pas repris.

use super::sin_cos::sin_cos; // adapte le chemin à ton arborescence

/// Cosinus de `x` (en radians).
///
/// - `x` infini ou NaN → NaN
pub fn cos(x: f32) -> f32 {
    sin_cos(x).1
}

#[cfg(test)]
mod tests {
    use super::cos;
    use core::f32::consts::{FRAC_PI_2, FRAC_PI_3, PI, TAU};

    const TOL: f32 = 2e-6;

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() <= TOL
    }

    #[test]
    fn known_angles() {
        assert_eq!(cos(0.0), 1.0);
        assert!(close(cos(FRAC_PI_3), 0.5));
        assert!(close(cos(FRAC_PI_2), 0.0));
        assert!(close(cos(PI), -1.0));
        assert!(close(cos(3.0 * FRAC_PI_2), 0.0));
        assert!(close(cos(TAU), 1.0));
    }

    #[test]
    fn known_values() {
        // Valeurs de référence indépendantes de notre implémentation.
        let cases: &[(f32, f32)] = &[
            (0.5, 0.877_582_6),
            (1.0, 0.540_302_3),
            (2.0, -0.416_146_84),
            (3.0, -0.989_992_5),
            (4.0, -0.653_643_6),
            (5.0, 0.283_662_2),
            (6.0, 0.960_170_3),
        ];
        for &(x, expected) in cases {
            let got = cos(x);
            assert!(close(got, expected), "cos({x}) = {got}, attendu {expected}");
        }
    }

    #[test]
    fn even_symmetry() {
        for i in 1..=100 {
            let x = i as f32 * 0.173;
            assert!(close(cos(-x), cos(x)), "x={x}");
        }
    }

    #[test]
    fn periodicity() {
        for i in 0..=100 {
            let x = i as f32 * 0.061;
            assert!((cos(x + TAU) - cos(x)).abs() <= 5e-6, "x={x}");
        }
    }

    #[test]
    fn bounded() {
        for i in -500..=500 {
            let x = i as f32 * 0.0731;
            assert!(cos(x).abs() <= 1.0 + 1e-6, "x={x}");
        }
    }

    #[test]
    fn special_values() {
        assert!(cos(f32::NAN).is_nan());
        assert!(cos(f32::INFINITY).is_nan());
        assert!(cos(f32::NEG_INFINITY).is_nan());
    }
}
