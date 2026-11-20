// SPDX-License-Identifier: GPL-2.0-or-later
//! Arc tangent of `f32` for `no_std`, without `libm`.
//!
//! # Méthode
//!
//! Soit `a = |x|`. On réduit l'argument à `|t| <= tan(π/8)` :
//!
//! * `a > tan(3π/8)` : `atan(a) = π/2 + atan(-1/a)`
//! * `a > tan(π/8)`  : `atan(a) = π/4 + atan((a-1)/(a+1))`
//! * sinon           : `atan(a) = atan(a)`
//!
//! puis `atan(t) ≈ t + t·z·Q(z)`, avec `z = t²` et `Q` un polynôme minimax
//! de degré 3 en `z` (coefficients de l'`atanf` de la bibliothèque Cephes,
//! S. L. Moshier). Le signe de `x` est réappliqué sur le résultat.
//!
//! La forme `t + t·z·Q(z)` conserve la précision relative près de 0
//! (jusqu'aux subnormaux).
//!
//! # Comportement
//!
//! | Entrée   | Sortie                       |
//! |----------|------------------------------|
//! | `±0.0`   | `±0.0` (signe conservé)      |
//! | `±∞`     | `±π/2`                       |
//! | `NaN`    | `NaN`                        |
//!
//! Résultat dans `[-π/2, π/2]`.

use core::f32::consts::{FRAC_2_PI, FRAC_PI_2, FRAC_PI_4};

const SIGN_BIT: u32 = 1 << 31;

/// tan(3π/8) = 1 + √2
const TAN_3PI_8: f32 = 2.414_213_6;
/// tan(π/8) = √2 − 1
const TAN_PI_8: f32 = 0.414_213_57;

// Polynôme minimax : atan(t) ≈ t + t·z·(P0 + z·(P1 + z·(P2 + z·P3))).
const P0: f32 = -0.333_329_5;
const P1: f32 = 0.199_777_1;
const P2: f32 = -0.138_776_86;
const P3: f32 = 0.080_537_445;

/// Arc tangente de `x`, en radians, dans `[-π/2, π/2]`.
///
/// Retourne `NaN` si `x` est `NaN`.
pub fn atan(x: f32) -> f32 {
    if x.is_nan() {
        return x;
    }

    let bits = x.to_bits();
    let a = f32::from_bits(bits & !SIGN_BIT);

    let (base, t) = if a > TAN_3PI_8 {
        (FRAC_PI_2, -1.0 / a)
    } else if a > TAN_PI_8 {
        (FRAC_PI_4, (a - 1.0) / (a + 1.0))
    } else {
        (0.0, a)
    };

    let z = t * t;
    let p = (((P3 * z + P2) * z + P1) * z + P0) * z * t + t;
    let r = base + p;

    // Réapplique le signe (fonction impaire, conserve -0.0).
    f32::from_bits(r.to_bits() | (bits & SIGN_BIT))
}

/// `atan(x)` normalisé dans `[-1, 1]` (c'est-à-dire `atan(x) · 2/π`).
#[inline]
pub fn atan_norm(x: f32) -> f32 {
    atan(x) * FRAC_2_PI
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32::consts;

    /// Erreur absolue tolérée (ulp de f32 près de π/2 : ~1,2·10⁻⁷).
    const TOL: f32 = 5e-7;

    fn ref_f64(x: f32) -> f64 {
        (x as f64).atan()
    }

    #[test]
    fn valeurs_remarquables() {
        let s3 = 3.0f32.sqrt();
        let vecteurs: &[(f32, f32)] = &[
            (0.0, 0.0),
            (s3 / 3.0, consts::FRAC_PI_6),
            (1.0, consts::FRAC_PI_4),
            (s3, consts::FRAC_PI_3),
            (-s3 / 3.0, -consts::FRAC_PI_6),
            (-1.0, -consts::FRAC_PI_4),
            (-s3, -consts::FRAC_PI_3),
        ];
        for &(x, attendu) in vecteurs {
            let r = atan(x);
            assert!(
                (r - attendu).abs() <= TOL,
                "atan({x}) = {r}, attendu {attendu}"
            );
        }
    }

    #[test]
    fn zeros_signes() {
        assert_eq!(atan(0.0).to_bits(), 0.0f32.to_bits());
        assert_eq!(atan(-0.0).to_bits(), (-0.0f32).to_bits());
    }

    #[test]
    fn nan_reste_nan() {
        assert!(atan(f32::NAN).is_nan());
        assert!(atan(-f32::NAN).is_nan());
        assert!(atan_norm(f32::NAN).is_nan());
    }

    #[test]
    fn infinis() {
        assert_eq!(atan(f32::INFINITY), FRAC_PI_2);
        assert_eq!(atan(f32::NEG_INFINITY), -FRAC_PI_2);
    }

    #[test]
    fn grandes_valeurs() {
        for &x in &[1e3f32, 1e6, 1e12, 1e30, f32::MAX] {
            for &s in &[1.0f32, -1.0] {
                let x = s * x;
                let err = (atan(x) as f64 - ref_f64(x)).abs();
                assert!(err <= TOL as f64, "x = {x:e}, err = {err:e}");
            }
        }
    }

    #[test]
    fn petites_valeurs_precision_relative() {
        for &x in &[
            1e-30f32,
            -1e-30,
            1e-10,
            -1e-10,
            1e-5,
            f32::EPSILON,
            f32::MIN_POSITIVE,
            f32::from_bits(1),
            f32::from_bits(SIGN_BIT | 1),
        ] {
            let got = atan(x) as f64;
            let want = ref_f64(x);
            assert!(((got - want) / want).abs() <= 1e-6, "x = {x:e}");
        }
    }

    #[test]
    fn symetrie_impaire_exacte() {
        let mut x = 0.0f32;
        while x <= 50.0 {
            assert_eq!(atan(-x).to_bits(), (-atan(x)).to_bits(), "x = {x}");
            x += 1.0 / 211.0;
        }
    }

    #[test]
    fn raccords_des_branches() {
        // Balaie 128 flottants consécutifs autour de chaque seuil.
        for &seuil in &[TAN_PI_8, TAN_3PI_8] {
            let mut x = f32::from_bits(seuil.to_bits() - 64);
            for _ in 0..128 {
                let err = (atan(x) as f64 - ref_f64(x)).abs();
                assert!(err <= TOL as f64, "x = {x}, err = {err:e}");
                x = f32::from_bits(x.to_bits() + 1);
            }
        }
    }

    #[test]
    fn identite_inverse() {
        // atan(x) + atan(1/x) = π/2 pour x > 0
        let mut x = 0.01f32;
        while x < 100.0 {
            let somme = atan(x) + atan(1.0 / x);
            assert!(
                (somme - FRAC_PI_2).abs() <= 2.0 * TOL,
                "x = {x}, somme = {somme}"
            );
            x *= 1.013;
        }
    }

    #[test]
    fn dans_l_intervalle() {
        for i in -50_000..=50_000 {
            let x = (i as f32) / 100.0;
            let r = atan(x);
            assert!((-FRAC_PI_2..=FRAC_PI_2).contains(&r), "x = {x}, r = {r}");
        }
    }

    #[test]
    fn monotone_croissante() {
        let mut prec = atan(-100.0);
        for i in -99_999..=100_000 {
            let x = (i as f32) / 1000.0;
            let r = atan(x);
            // Tolérance de bruit d'arrondi aux raccords.
            assert!(r >= prec - 3e-7, "non monotone en x = {x}: {r} < {prec}");
            prec = r;
        }
    }

    #[test]
    fn norm_dans_moins_un_un() {
        assert!((atan_norm(1.0) - 0.5).abs() <= TOL);
        assert!((atan_norm(-1.0) + 0.5).abs() <= TOL);
        assert!((atan_norm(f32::INFINITY) - 1.0).abs() <= TOL);
        assert!((atan_norm(f32::NEG_INFINITY) + 1.0).abs() <= TOL);
    }

    #[test]
    fn balayage_vs_f64() {
        let mut pire = 0.0f64;
        // Échelle linéaire [-10, 10] puis logarithmique jusqu'à 1e30.
        for i in -200_000..=200_000 {
            let x = (i as f32) / 20_000.0;
            let err = (atan(x) as f64 - ref_f64(x)).abs();
            pire = pire.max(err);
            assert!(err <= TOL as f64, "x = {x}, err = {err:e}");
        }
        let mut x = 10.0f32;
        while x < 1e30 {
            for &s in &[1.0f32, -1.0] {
                let err = (atan(s * x) as f64 - ref_f64(s * x)).abs();
                pire = pire.max(err);
                assert!(err <= TOL as f64, "x = {}, err = {err:e}", s * x);
            }
            x *= 1.07;
        }
        std::println!("erreur max : {pire:e}");
    }
}
