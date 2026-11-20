// SPDX-License-Identifier: GPL-2.0-or-later
//! Arc tangente à quatre quadrants de `f32` pour `no_std`, sans `libm`.
//!
//! # Méthode
//!
//! Soit `a = |y|` et `b = |x|`.
//!
//! * `a <= b` : `r = atan(a / b)`
//! * `a >  b` : `r = π/2 − atan(b / a)`
//!
//! Le rapport est toujours dans `[0, 1]` : pas de débordement. `atan` sur
//! `[0, 1]` utilise `atan(t) = π/4 + atan((t−1)/(t+1))` pour `t > tan(π/8)`,
//! puis `atan(u) ≈ u + u·z·Q(z)` avec `z = u²`, `Q` de degré 3 (coefficients
//! de l'`atanf` de Cephes, S. L. Moshier).
//!
//! Si `x` est négatif : `r = π − r`. Le signe de `y` est réappliqué en dernier.
//!
//! # Comportement (conforme à IEEE 754 `atan2`)
//!
//! | `y`, `x`            | Sortie                    |
//! |---------------------|---------------------------|
//! | `±0`, `+0`          | `±0`                      |
//! | `±0`, `-0`          | `±π`                      |
//! | `±0`, `x < 0`       | `±π`                      |
//! | `y ≠ 0`, `±0`       | `±π/2`                    |
//! | `±∞`, `+∞`          | `±π/4`                    |
//! | `±∞`, `-∞`          | `±3π/4`                   |
//! | `NaN` en entrée     | `NaN`                     |
//!
//! Résultat dans `[-π, π]`.

use core::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

const SIGN_BIT: u32 = 1 << 31;

/// tan(π/8) = √2 − 1
const TAN_PI_8: f32 = 0.414_213_57;

// atan(u) ≈ u + u·z·(P0 + z·(P1 + z·(P2 + z·P3))).
const P0: f32 = -0.333_329_5;
const P1: f32 = 0.199_777_1;
const P2: f32 = -0.138_776_86;
const P3: f32 = 0.080_537_445;

/// `atan(t)` pour `t ∈ [0, 1]` (usage interne).
#[inline(always)]
fn atan_unit(t: f32) -> f32 {
    let (base, u) = if t > TAN_PI_8 {
        (FRAC_PI_4, (t - 1.0) / (t + 1.0))
    } else {
        (0.0, t)
    };
    let z = u * u;
    base + ((((P3 * z + P2) * z + P1) * z + P0) * z * u + u)
}

/// Arc tangente de `y / x` en radians, quadrant déterminé par les signes
/// de `y` et `x`, dans `[-π, π]`.
///
/// Retourne `NaN` si `y` ou `x` est `NaN`.
pub fn atan2(y: f32, x: f32) -> f32 {
    if y != y || x != x {
        return f32::NAN;
    }

    let yb = y.to_bits();
    let xb = x.to_bits();
    let a = f32::from_bits(yb & !SIGN_BIT);
    let b = f32::from_bits(xb & !SIGN_BIT);

    let r = if a == 0.0 && b == 0.0 {
        0.0
    } else if a == b {
        // Égalité de modules (inclut ±∞ / ±∞, où a/b serait NaN).
        FRAC_PI_4
    } else if a > b {
        FRAC_PI_2 - atan_unit(b / a)
    } else {
        atan_unit(a / b)
    };

    // x négatif (y compris -0.0) : on passe dans le demi-plan gauche.
    let r = if xb & SIGN_BIT != 0 { PI - r } else { r };

    // Signe de y (conserve -0.0).
    f32::from_bits(r.to_bits() | (yb & SIGN_BIT))
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32::consts;

    /// Erreur absolue tolérée (ulp de f32 près de π : ~2,4·10⁻⁷).
    const TOL: f32 = 6e-7;

    fn ref_f64(y: f32, x: f32) -> f64 {
        (y as f64).atan2(x as f64)
    }

    fn proche(y: f32, x: f32) {
        let err = (atan2(y, x) as f64 - ref_f64(y, x)).abs();
        assert!(err <= TOL as f64, "atan2({y:e}, {x:e}) : err = {err:e}");
    }

    #[test]
    fn valeurs_remarquables() {
        let s3 = 3.0f32.sqrt();
        let vecteurs: &[(f32, f32, f32)] = &[
            (0.0, 1.0, 0.0),
            (1.0, 1.0, consts::FRAC_PI_4),
            (1.0, 0.0, consts::FRAC_PI_2),
            (1.0, s3, consts::FRAC_PI_6),
            (s3, 1.0, consts::FRAC_PI_3),
            (1.0, -1.0, 3.0 * consts::FRAC_PI_4),
            (0.0, -1.0, consts::PI),
            (-1.0, -1.0, -3.0 * consts::FRAC_PI_4),
            (-1.0, 0.0, -consts::FRAC_PI_2),
            (-1.0, 1.0, -consts::FRAC_PI_4),
        ];
        for &(y, x, attendu) in vecteurs {
            let r = atan2(y, x);
            assert!((r - attendu).abs() <= TOL, "atan2({y}, {x}) = {r}, attendu {attendu}");
        }
    }

    #[test]
    fn zeros_signes() {
        let pz = 0.0f32;
        let nz = -0.0f32;
        assert_eq!(atan2(pz, pz).to_bits(), pz.to_bits());
        assert_eq!(atan2(nz, pz).to_bits(), nz.to_bits());
        assert_eq!(atan2(pz, nz), PI);
        assert_eq!(atan2(nz, nz), -PI);
        assert_eq!(atan2(pz, -1.0), PI);
        assert_eq!(atan2(nz, -1.0), -PI);
        assert_eq!(atan2(pz, 1.0).to_bits(), pz.to_bits());
        assert_eq!(atan2(nz, 1.0).to_bits(), nz.to_bits());
    }

    #[test]
    fn axes() {
        for &s in &[1.0f32, 1e-30, 1e30, f32::MAX, f32::MIN_POSITIVE, f32::from_bits(1)] {
            assert_eq!(atan2(s, 0.0), FRAC_PI_2);
            assert_eq!(atan2(s, -0.0), FRAC_PI_2);
            assert_eq!(atan2(-s, 0.0), -FRAC_PI_2);
            assert_eq!(atan2(0.0, s).to_bits(), 0.0f32.to_bits());
            assert_eq!(atan2(0.0, -s), PI);
            assert_eq!(atan2(-0.0, -s), -PI);
        }
    }

    #[test]
    fn infinis() {
        let inf = f32::INFINITY;
        let tol = TOL;
        assert!((atan2(inf, inf) - consts::FRAC_PI_4).abs() <= tol);
        assert!((atan2(-inf, inf) + consts::FRAC_PI_4).abs() <= tol);
        assert!((atan2(inf, -inf) - 3.0 * consts::FRAC_PI_4).abs() <= tol);
        assert!((atan2(-inf, -inf) + 3.0 * consts::FRAC_PI_4).abs() <= tol);
        assert_eq!(atan2(inf, 1.0), FRAC_PI_2);
        assert_eq!(atan2(-inf, -1.0), -FRAC_PI_2);
        assert_eq!(atan2(inf, 0.0), FRAC_PI_2);
        assert_eq!(atan2(1.0, inf).to_bits(), 0.0f32.to_bits());
        assert_eq!(atan2(-1.0, inf).to_bits(), (-0.0f32).to_bits());
        assert_eq!(atan2(1.0, -inf), PI);
        assert_eq!(atan2(-1.0, -inf), -PI);
    }

    #[test]
    fn nan_reste_nan() {
        let n = f32::NAN;
        assert!(atan2(n, 1.0).is_nan());
        assert!(atan2(1.0, n).is_nan());
        assert!(atan2(n, n).is_nan());
        assert!(atan2(-n, 0.0).is_nan());
        assert!(atan2(f32::INFINITY, n).is_nan());
    }

    #[test]
    fn extremes_et_subnormaux() {
        let vals = [
            f32::MAX,
            f32::MIN_POSITIVE,
            f32::from_bits(1),
            f32::from_bits(0x007F_FFFF),
            1e-30,
            1e30,
            1.0,
        ];
        for &y in &vals {
            for &x in &vals {
                for &(sy, sx) in &[(1.0f32, 1.0f32), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)] {
                    proche(sy * y, sx * x);
                }
            }
        }
    }

    #[test]
    fn invariance_d_echelle() {
        // Multiplier x et y par une puissance de 2 laisse le rapport inchangé.
        let mut i = 0;
        while i < 2000 {
            let y = ((i * 37 % 1000) as f32 - 500.0) / 7.0;
            let x = ((i * 91 % 1000) as f32 - 500.0) / 11.0;
            if x != 0.0 || y != 0.0 {
                assert_eq!(
                    atan2(y * 1024.0, x * 1024.0).to_bits(),
                    atan2(y, x).to_bits(),
                    "y = {y}, x = {x}"
                );
            }
            i += 1;
        }
    }

    #[test]
    fn symetrie_en_y_exacte() {
        for i in 0..5000 {
            let t = (i as f32) * 0.0123;
            let (y, x) = (t.sin() * 3.0, t.cos() * 3.0);
            assert_eq!(atan2(-y, x).to_bits(), (-atan2(y, x)).to_bits(), "y = {y}, x = {x}");
        }
    }

    #[test]
    fn reflexion_en_x() {
        // atan2(y, -x) = π − atan2(y, x) pour y > 0
        for i in 1..5000 {
            let t = (i as f32) * 0.000_6;
            let (y, x) = (t.sin() + 1e-3, t.cos());
            let somme = atan2(y, x) + atan2(y, -x);
            assert!((somme - PI).abs() <= 2.0 * TOL, "y = {y}, x = {x}");
        }
    }

    #[test]
    fn dans_l_intervalle() {
        for i in -300..=300 {
            for j in -300..=300 {
                let (y, x) = (i as f32 / 17.0, j as f32 / 13.0);
                let r = atan2(y, x);
                assert!((-PI..=PI).contains(&r), "y = {y}, x = {x}, r = {r}");
            }
        }
    }

    #[test]
    fn raccord_diagonale() {
        // Autour de |y| = |x| : changement de branche.
        for &x in &[1.0f32, 3.7, 1e-20, 1e20] {
            let mut y = f32::from_bits(x.to_bits() - 64);
            for _ in 0..128 {
                proche(y, x);
                proche(-y, -x);
                y = f32::from_bits(y.to_bits() + 1);
            }
        }
    }

    #[test]
    fn raccord_au_seuil_de_reduction() {
        // Autour de |y|/|x| = tan(π/8) : changement de réduction interne.
        let mut y = f32::from_bits(TAN_PI_8.to_bits() - 64);
        for _ in 0..128 {
            proche(y, 1.0);
            proche(y, -1.0);
            proche(1.0, y);
            y = f32::from_bits(y.to_bits() + 1);
        }
    }

    #[test]
    fn aller_retour_angle() {
        let n = 100_000;
        for i in 0..=n {
            let theta = -(PI as f64) + 2.0 * (PI as f64) * (i as f64) / (n as f64);
            let (y, x) = (theta.sin() as f32, theta.cos() as f32);
            proche(y, x);
        }
    }

    #[test]
    fn balayage_vs_f64() {
        let mut pire = 0.0f64;
        for i in -400..=400 {
            for j in -400..=400 {
                let (y, x) = (i as f32 / 40.0, j as f32 / 40.0);
                if x == 0.0 && y == 0.0 {
                    continue;
                }
                let err = (atan2(y, x) as f64 - ref_f64(y, x)).abs();
                pire = pire.max(err);
                assert!(err <= TOL as f64, "y = {y}, x = {x}, err = {err:e}");
            }
        }
        std::println!("erreur max : {pire:e}");
    }
}