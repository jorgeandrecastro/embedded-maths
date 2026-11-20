// SPDX-License-Identifier: GPL-2.0-or-later
//! Arc cosine of `f32` for `no_std`, without `libm`.
//!
//! # Méthode
//!
//! Pour `a = |x| ∈ [0, 1]` :
//!
//! ```text
//! acos(a) ≈ √(1 - a) · P(a)
//! ```
//!
//! où `P` est le polynôme de degré 7 d'Abramowitz & Stegun, formule 4.4.46
//! (erreur théorique ≤ 2·10⁻⁸). Pour `x < 0`, on utilise
//! `acos(x) = π - acos(|x|)`.
//!
//! La racine carrée utilise `embedded-f32-sqrt`, sans `micromath`.
//!
//! # Comportement
//!
//! | Entrée        | Sortie                   |
//! |---------------|--------------------------|
//! | `1.0`         | `0.0` exactement         |
//! | `-1.0`        | `π` exactement           |
//! | `|x| > 1`     | `NaN`                    |
//! | `NaN`         | `NaN`                    |
//! | `±∞`          | `NaN`                    |
//!
//! Résultat dans `[0, π]`.

use core::f32::consts::{FRAC_PI_2, PI};
use embedded_f32_sqrt::sqrt;

// Coefficients A&S 4.4.46.
const C0: f32 = FRAC_PI_2;
const C1: f32 = -0.214_598_8;
const C2: f32 = 0.088_978_99;
const C3: f32 = -0.050_174_304;
const C4: f32 = 0.030_891_88;
const C5: f32 = -0.017_088_126;
const C6: f32 = 0.006_670_09;
const C7: f32 = -0.001_262_491_1;

/// Arc cosinus de `x`, en radians, dans `[0, π]`.
///
/// Retourne `NaN` si `|x| > 1` ou si `x` est `NaN`.
pub fn acos(x: f32) -> f32 {
    let a = if x < 0.0 { -x } else { x };

    // `!(a <= 1.0)` capte NaN et |x| > 1 en une comparaison.
    if a.is_nan() || a > 1.0 {
        return f32::NAN;
    }

    // Horner.
    let p = C0 + a * (C1 + a * (C2 + a * (C3 + a * (C4 + a * (C5 + a * (C6 + a * C7))))));
    // Ici 1.0 - a ∈ [0, 1] (domaine déjà validé) : l'Err n'arrive jamais.
    let r = sqrt(1.0 - a).unwrap_or(0.0) * p;

    if x < 0.0 { PI - r } else { r }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32::consts;

    /// Erreur absolue tolérée. L'ulp de f32 vers π vaut ~2,4·10⁻⁷.
    const TOL: f32 = 1e-6;

    #[test]
    fn valeurs_remarquables() {
        let vecteurs: &[(f32, f32)] = &[
            (1.0, 0.0),
            (0.866_025_4, consts::FRAC_PI_6),
            (consts::FRAC_1_SQRT_2, consts::FRAC_PI_4),
            (0.5, consts::FRAC_PI_3),
            (0.0, consts::FRAC_PI_2),
            (-0.5, 2.0 * consts::FRAC_PI_3),
            (-consts::FRAC_1_SQRT_2, 3.0 * consts::FRAC_PI_4),
            (-0.866_025_4, 5.0 * consts::FRAC_PI_6),
            (-1.0, consts::PI),
        ];
        for &(x, attendu) in vecteurs {
            let r = acos(x);
            assert!(
                (r - attendu).abs() <= TOL,
                "acos({x}) = {r}, attendu {attendu}"
            );
        }
    }

    #[test]
    fn bornes_exactes() {
        assert_eq!(acos(1.0).to_bits(), 0.0f32.to_bits());
        assert_eq!(acos(-1.0), PI);
    }

    #[test]
    fn zeros_signes() {
        assert!((acos(0.0) - consts::FRAC_PI_2).abs() <= TOL);
        assert!((acos(-0.0) - consts::FRAC_PI_2).abs() <= TOL);
    }

    #[test]
    fn hors_domaine_donne_nan() {
        for &x in &[
            1.000_000_1,
            -1.000_000_1,
            2.0,
            -2.0,
            f32::MAX,
            f32::MIN,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::NAN,
            -f32::NAN,
        ] {
            assert!(acos(x).is_nan(), "acos({x:e}) devrait être NaN");
        }
    }

    #[test]
    fn petites_valeurs_et_subnormaux() {
        for &x in &[
            f32::EPSILON,
            -f32::EPSILON,
            f32::MIN_POSITIVE,
            -f32::MIN_POSITIVE,
            f32::from_bits(1),
            f32::from_bits(0x8000_0001),
        ] {
            assert!((acos(x) - consts::FRAC_PI_2).abs() <= TOL, "x = {x:e}");
        }
    }

    #[test]
    fn voisinage_des_bornes() {
        // 1 - ulp : la racine amplifie les erreurs, point le plus délicat.
        let sous_un = f32::from_bits(1.0f32.to_bits() - 1);
        for &x in &[sous_un, -sous_un, 0.999_9, -0.999_9, 0.999, -0.999] {
            let attendu = (x as f64).acos() as f32;
            assert!((acos(x) - attendu).abs() <= TOL, "x = {x}");
        }
    }

    #[test]
    fn symetrie() {
        // acos(x) + acos(-x) = π
        let mut x = 0.0f32;
        while x <= 1.0 {
            let somme = acos(x) + acos(-x);
            assert!((somme - PI).abs() <= TOL, "x = {x}, somme = {somme}");
            x += 1.0 / 997.0;
        }
    }

    #[test]
    fn dans_l_intervalle_0_pi() {
        for i in 0..=20_000 {
            let x = -1.0 + 2.0 * (i as f32) / 20_000.0;
            let r = acos(x.clamp(-1.0, 1.0));
            assert!((0.0..=PI).contains(&r), "x = {x}, r = {r}");
        }
    }

    #[test]
    fn monotone_decroissante() {
        let mut prec = acos(-1.0);
        for i in 1..=100_000 {
            let x = -1.0 + 2.0 * (i as f32) / 100_000.0;
            let r = acos(x.clamp(-1.0, 1.0));
            // Tolérance d'un ulp pour le bruit d'arrondi.
            assert!(r <= prec + 3e-7, "non monotone en x = {x}: {r} > {prec}");
            prec = r;
        }
    }

    #[test]
    fn balayage_vs_f64() {
        let n = 200_000;
        let mut pire = 0.0f64;
        for i in 0..=n {
            let x = -1.0 + 2.0 * (i as f64) / (n as f64);
            let got = acos(x as f32) as f64;
            let want = (x as f32 as f64).acos();
            let err = (got - want).abs();
            if err > pire {
                pire = err;
            }
            assert!(err <= TOL as f64, "x = {x}, got = {got}, want = {want}");
        }
        // Affiché avec `cargo test -- --nocapture`.
        std::println!("erreur max sur [-1, 1] : {pire:e}");
    }

    #[test]
    fn sqrt_interne() {
        for i in 0..=10_000 {
            let x = (i as f32) / 10_000.0;
            let got = sqrt(x).expect("racine positive attendue") as f64;
            let want = (x as f64).sqrt();
            assert!((got - want).abs() <= 1e-6, "x = {x}");
        }
        assert_eq!(sqrt(0.0), Ok(0.0));
        assert!((sqrt(1.0).unwrap() - 1.0).abs() <= 1e-7);
        // Plus petit normal : la racine doit converger malgré l'estimation grossière.
        let petit = sqrt(f32::MIN_POSITIVE).unwrap() as f64;
        assert!((petit - (f32::MIN_POSITIVE as f64).sqrt()).abs() / petit <= 1e-6);
    }
}
