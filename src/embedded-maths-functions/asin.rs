// SPDX-License-Identifier: GPL-2.0-or-later
//! Arc sinus de `f32` pour `no_std`, sans `libm`.
//!
//! # Méthode
//!
//! Soit `a = |x|`.
//!
//! * `a <= 0.25` : série de Taylor `x·(1 + x²/6 + 3x⁴/40 + ...)` (8 termes).
//!   Précision relative conservée jusqu'aux subnormaux.
//! * `0.25 < a <= 1` : `asin(a) ≈ π/2 − √(1−a)·P(a)` avec `P` le polynôme de
//!   degré 7 d'Abramowitz & Stegun, formule 4.4.45 (erreur ≤ 2·10⁻⁸).
//!
//! Le signe de `x` est réappliqué sur le résultat (fonction impaire).
//! La racine carrée utilise `embedded-f32-sqrt`, sans `micromath`.
//!
//! # Comportement
//!
//! | Entrée     | Sortie                     |
//! |------------|----------------------------|
//! | `±1.0`     | `±π/2` exactement          |
//! | `±0.0`     | `±0.0` (signe conservé)    |
//! | `|x| > 1`  | `NaN`                      |
//! | `NaN`, `±∞`| `NaN`                      |

use core::f32::consts::FRAC_PI_2;
use embedded_f32_sqrt::sqrt;

const SIGN_BIT: u32 = 1 << 31;

/// Seuil entre la série de Taylor et la formule A&S.
const TAYLOR_MAX: f32 = 0.25;

// Taylor : asin(x) = x · Σ c_n · x^(2n), c_n = C(2n,n) / (4^n (2n+1)).
const T1: f32 = 1.0 / 6.0;
const T2: f32 = 3.0 / 40.0;
const T3: f32 = 5.0 / 112.0;
const T4: f32 = 35.0 / 1152.0;
const T5: f32 = 63.0 / 2816.0;
const T6: f32 = 231.0 / 13312.0;
const T7: f32 = 143.0 / 10240.0;

// Coefficients A&S 4.4.45.
const C0: f32 = 1.570_796_3;
const C1: f32 = -0.214_598_8;
const C2: f32 = 0.088_978_99;
const C3: f32 = -0.050_174_304;
const C4: f32 = 0.030_891_88;
const C5: f32 = -0.017_088_126;
const C6: f32 = 0.006_670_09;
const C7: f32 = -0.001_262_491_1;

/// Arc sinus de `x`, en radians, dans `[-π/2, π/2]`.
///
/// Retourne `NaN` si `|x| > 1` ou si `x` est `NaN`.
pub fn asin(x: f32) -> f32 {
    let bits = x.to_bits();
    let a = f32::from_bits(bits & !SIGN_BIT);

    // Capte NaN, ±∞ et |x| > 1 en une comparaison.
    if !(a <= 1.0) {
        return f32::NAN;
    }

    let r = if a <= TAYLOR_MAX {
        let x2 = a * a;
        a * (1.0
            + x2 * (T1 + x2 * (T2 + x2 * (T3 + x2 * (T4 + x2 * (T5 + x2 * (T6 + x2 * T7)))))))
    } else {
        let p = C0
            + a * (C1 + a * (C2 + a * (C3 + a * (C4 + a * (C5 + a * (C6 + a * C7))))));
        FRAC_PI_2 - sqrt(1.0 - a).unwrap_or(0.0) * p
    };

    // Réapplique le signe (fonction impaire, conserve -0.0).
    f32::from_bits(r.to_bits() | (bits & SIGN_BIT))
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32::consts;

    /// Erreur absolue tolérée (ulp de f32 près de π/2 : ~1,2·10⁻⁷).
    const TOL: f32 = 1e-6;

    fn ref_f64(x: f32) -> f64 {
        (x as f64).asin()
    }

    #[test]
    fn valeurs_remarquables() {
        let vecteurs: &[(f32, f32)] = &[
            (0.0, 0.0),
            (0.5, consts::FRAC_PI_6),
            (0.707_106_8, consts::FRAC_PI_4),
            (0.866_025_4, consts::FRAC_PI_3),
            (1.0, consts::FRAC_PI_2),
            (-0.5, -consts::FRAC_PI_6),
            (-0.707_106_8, -consts::FRAC_PI_4),
            (-0.866_025_4, -consts::FRAC_PI_3),
            (-1.0, -consts::FRAC_PI_2),
        ];
        for &(x, attendu) in vecteurs {
            let r = asin(x);
            assert!((r - attendu).abs() <= TOL, "asin({x}) = {r}, attendu {attendu}");
        }
    }

    #[test]
    fn bornes_exactes() {
        assert_eq!(asin(1.0), FRAC_PI_2);
        assert_eq!(asin(-1.0), -FRAC_PI_2);
    }

    #[test]
    fn zeros_signes() {
        assert_eq!(asin(0.0).to_bits(), 0.0f32.to_bits());
        assert_eq!(asin(-0.0).to_bits(), (-0.0f32).to_bits());
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
            assert!(asin(x).is_nan(), "asin({x:e}) devrait être NaN");
        }
    }

    #[test]
    fn petites_valeurs_precision_relative() {
        // asin(x) ≈ x près de 0 : la précision relative doit tenir.
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
            let got = asin(x) as f64;
            let want = ref_f64(x);
            assert!(((got - want) / want).abs() <= 1e-6, "x = {x:e}");
        }
    }

    #[test]
    fn symetrie_impaire_exacte() {
        let mut x = 0.0f32;
        while x <= 1.0 {
            assert_eq!(asin(-x).to_bits(), (-asin(x)).to_bits(), "x = {x}");
            x += 1.0 / 1009.0;
        }
    }

    #[test]
    fn raccord_au_seuil() {
        // Les deux branches doivent coïncider autour de 0.25.
        let mut x = f32::from_bits(TAYLOR_MAX.to_bits() - 64);
        for _ in 0..128 {
            let err = (asin(x) as f64 - ref_f64(x)).abs();
            assert!(err <= TOL as f64, "x = {x}, err = {err:e}");
            x = f32::from_bits(x.to_bits() + 1);
        }
    }

    #[test]
    fn voisinage_de_un() {
        let sous_un = f32::from_bits(1.0f32.to_bits() - 1);
        for &x in &[sous_un, -sous_un, 0.999_9, -0.999_9, 0.999, -0.999] {
            let err = (asin(x) as f64 - ref_f64(x)).abs();
            assert!(err <= TOL as f64, "x = {x}, err = {err:e}");
        }
    }

    #[test]
    fn dans_l_intervalle() {
        for i in 0..=20_000 {
            let x = (-1.0 + 2.0 * (i as f32) / 20_000.0).clamp(-1.0, 1.0);
            let r = asin(x);
            assert!((-FRAC_PI_2..=FRAC_PI_2).contains(&r), "x = {x}, r = {r}");
        }
    }

    #[test]
    fn monotone_croissante() {
        let mut prec = asin(-1.0);
        for i in 1..=100_000 {
            let x = (-1.0 + 2.0 * (i as f32) / 100_000.0).clamp(-1.0, 1.0);
            let r = asin(x);
            // Tolérance de bruit d'arrondi (raccord Taylor / A&S).
            assert!(r >= prec - 3e-7, "non monotone en x = {x}: {r} < {prec}");
            prec = r;
        }
    }

    #[test]
    fn aller_retour_sinus() {
        for i in 0..=10_000 {
            let x = -1.0 + 2.0 * (i as f32) / 10_000.0;
            let x = x.clamp(-1.0, 1.0);
            let retour = (asin(x) as f64).sin();
            assert!((retour - x as f64).abs() <= 1e-6, "x = {x}, retour = {retour}");
        }
    }

    #[test]
    fn balayage_vs_f64() {
        let n = 200_000;
        let mut pire = 0.0f64;
        for i in 0..=n {
            let x = (-1.0 + 2.0 * (i as f64) / (n as f64)) as f32;
            let err = (asin(x) as f64 - ref_f64(x)).abs();
            pire = pire.max(err);
            assert!(err <= TOL as f64, "x = {x}, err = {err:e}");
        }
        std::println!("erreur max sur [-1, 1] : {pire:e}");
    }

    #[test]
    fn sqrt_interne() {
        for i in 0..=10_000 {
            let x = (i as f32) / 10_000.0;
            let got = sqrt(x).expect("racine positive attendue") as f64;
            assert!((got - (x as f64).sqrt()).abs() <= 1e-6, "x = {x}");
        }
        assert_eq!(sqrt(0.0), Ok(0.0));
    }
}