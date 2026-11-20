// SPDX-License-Identifier: GPL-2.0-or-later
//! Valeur absolue de `f32` pour `no_std`, sans `libm`.
//!
//! L'opération se réduit à effacer le bit de signe (bit 31) : une seule
//! instruction `AND` sur Cortex-M, RISC-V (ESP32-C3) et RP2350, sans
//! branchement et sans FPU requise.
//!
//! # Comportement
//!
//! | Entrée      | Sortie                                  |
//! |-------------|-----------------------------------------|
//! | `-0.0`      | `+0.0`                                  |
//! | `±∞`        | `+∞`                                    |
//! | `NaN`       | `NaN` (payload conservé, signe effacé)  |
//! | subnormaux  | magnitude inchangée, signe effacé       |

const SIGN_BIT: u32 = 1 << 31;
const MAGNITUDE_MASK: u32 = !SIGN_BIT;

/// Retourne `|x|`.
///
/// Utilisable dans un contexte `const` (Rust ≥ 1.83).
#[inline(always)]
pub const fn abs(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & MAGNITUDE_MASK)
}

/// Remplace chaque élément de `values` par sa valeur absolue.
#[inline]
pub fn abs_in_place(values: &mut [f32]) {
    for v in values {
        *v = abs(*v);
    }
}

/// Écrit `|src[i]|` dans `dst[i]`.
///
/// # Panics
///
/// Panique si `src.len() != dst.len()`.
#[inline]
pub fn abs_into(src: &[f32], dst: &mut [f32]) {
    assert_eq!(src.len(), dst.len(), "abs_into: longueurs différentes");
    for (d, s) in dst.iter_mut().zip(src) {
        *d = abs(*s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Référence indépendante du masquage de bits (comparaisons flottantes).
    fn reference(x: f32) -> f32 {
        if x < 0.0 {
            -x
        } else if x == 0.0 {
            0.0 // couvre -0.0 → +0.0
        } else {
            x
        }
    }

    fn check(bits: u32) {
        let x = f32::from_bits(bits);
        let got = abs(x);
        if x.is_nan() {
            assert!(got.is_nan(), "bits = {bits:#010x}");
            assert_eq!(got.to_bits(), bits & MAGNITUDE_MASK, "bits = {bits:#010x}");
        } else {
            assert_eq!(
                got.to_bits(),
                reference(x).to_bits(),
                "bits = {bits:#010x}"
            );
        }
        assert_eq!(got.to_bits() & SIGN_BIT, 0, "signe non effacé, bits = {bits:#010x}");
    }

    #[test]
    fn valeurs_usuelles() {
        assert_eq!(abs(1.0), 1.0);
        assert_eq!(abs(-1.0), 1.0);
        assert_eq!(abs(0.0), 0.0);
        assert_eq!(abs(-123.456), 123.456);
        assert_eq!(abs(7.25), 7.25);
    }

    #[test]
    fn zero_signe() {
        assert_eq!(abs(-0.0).to_bits(), 0.0f32.to_bits());
        assert_eq!(abs(0.0).to_bits(), 0.0f32.to_bits());
    }

    #[test]
    fn infinis() {
        assert_eq!(abs(f32::INFINITY).to_bits(), f32::INFINITY.to_bits());
        assert_eq!(abs(f32::NEG_INFINITY).to_bits(), f32::INFINITY.to_bits());
    }

    #[test]
    fn nan_reste_nan() {
        assert!(abs(f32::NAN).is_nan());
        // NaN négatif avec payload personnalisé
        let bits = SIGN_BIT | 0x7FC0_1234;
        let r = abs(f32::from_bits(bits));
        assert!(r.is_nan());
        assert_eq!(r.to_bits(), 0x7FC0_1234);
    }

    #[test]
    fn nan_signalant() {
        // sNaN : exposant plein, bit de quiet à 0, payload non nul
        let bits = SIGN_BIT | 0x7F80_0001;
        let r = abs(f32::from_bits(bits));
        assert_eq!(r.to_bits(), 0x7F80_0001);
    }

    #[test]
    fn extremes_et_subnormaux() {
        for &x in &[
            f32::MAX,
            f32::MIN,
            f32::MIN_POSITIVE,
            -f32::MIN_POSITIVE,
            f32::from_bits(1),            // plus petit subnormal
            f32::from_bits(SIGN_BIT | 1), // idem, négatif
            f32::from_bits(0x007F_FFFF),  // plus grand subnormal
            f32::from_bits(SIGN_BIT | 0x007F_FFFF),
        ] {
            assert_eq!(abs(x).to_bits(), reference(x).to_bits(), "x = {x:e}");
        }
    }

    #[test]
    fn idempotence() {
        for &x in &[-3.5f32, -0.0, 0.0, 7.25, f32::NEG_INFINITY, f32::MIN] {
            let a = abs(x);
            assert_eq!(abs(a).to_bits(), a.to_bits());
        }
    }

    #[test]
    fn symetrie() {
        for &x in &[1.5f32, 1e-30, 1e30, f32::MAX, f32::MIN_POSITIVE] {
            assert_eq!(abs(x).to_bits(), abs(-x).to_bits());
        }
    }

    #[test]
    fn contexte_const() {
        const A: f32 = abs(-2.0);
        const B: f32 = abs(f32::NEG_INFINITY);
        assert_eq!(A, 2.0);
        assert_eq!(B, f32::INFINITY);
    }

    #[test]
    fn tranches() {
        let mut v = [-1.0f32, 2.0, -0.0, f32::NEG_INFINITY, -5.5];
        abs_in_place(&mut v);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2].to_bits(), 0.0f32.to_bits());
        assert_eq!(v[3], f32::INFINITY);
        assert_eq!(v[4], 5.5);

        let src = [-3.0f32, 4.0, -0.5];
        let mut dst = [0.0f32; 3];
        abs_into(&src, &mut dst);
        assert_eq!(dst, [3.0, 4.0, 0.5]);
    }

    #[test]
    fn tranche_vide() {
        abs_in_place(&mut []);
        abs_into(&[], &mut []);
    }

    #[test]
    #[should_panic(expected = "longueurs différentes")]
    fn abs_into_longueurs_differentes() {
        abs_into(&[1.0, 2.0], &mut [0.0]);
    }

    #[test]
    fn balayage_espace_des_bits() {
        // Pas premier avec le modulo 2^32 : visite ~1M motifs répartis
        // sur tous les exposants, subnormaux, infinis et NaN.
        let mut bits: u32 = 0;
        loop {
            check(bits);
            match bits.checked_add(4099) {
                Some(next) => bits = next,
                None => break,
            }
        }
    }

    /// Exhaustif sur les 2^32 motifs. Lent : `cargo test --release -- --ignored`.
    #[test]
    #[ignore]
    fn exhaustif() {
        let mut bits: u32 = 0;
        loop {
            check(bits);
            if bits == u32::MAX {
                break;
            }
            bits += 1;
        }
    }
}