//! Partie entière (troncature vers zéro) pour `f32`, sans dépendance externe.
//!
//! Méthode : on lit l'exposant non biaisé `e` de `x`. Les 23 bits de mantisse
//! se partagent en `e` bits entiers et `23 - e` bits fractionnaires ; il suffit
//! de mettre ces derniers à zéro.

const SIGN_MASK: u32 = 0x8000_0000;
const MANTISSA_MASK: u32 = 0x007F_FFFF;

/// Partie entière de `x`, arrondie vers zéro.
///
/// - |x| < 1        → ±0 (le signe est conservé)
/// - |x| ≥ 2^23     → `x` inchangé (déjà entier), y compris ±∞ et NaN
pub fn trunc(x: f32) -> f32 {
    let bits = x.to_bits();
    let exp = ((bits >> 23) & 0xFF) as i32 - 127;

    if exp >= 23 {
        // Aucun bit fractionnaire (ou infini / NaN).
        return x;
    }
    if exp < 0 {
        // |x| < 1 (zéro et dénormalisés inclus) : zéro signé.
        return f32::from_bits(bits & SIGN_MASK);
    }

    // Les `23 - exp` bits de poids faible de la mantisse sont fractionnaires.
    let fractional_mask = MANTISSA_MASK >> exp;
    f32::from_bits(bits & !fractional_mask)
}

#[cfg(test)]
mod tests {
    use super::trunc;

    #[test]
    fn basic_values() {
        assert_eq!(trunc(-1.1), -1.0);
        assert_eq!(trunc(1.1), 1.0);
        assert_eq!(trunc(2.9), 2.0);
        assert_eq!(trunc(-2.9), -2.0);
        assert_eq!(trunc(1.0), 1.0);
        assert_eq!(trunc(0.0), 0.0);
        assert_eq!(trunc(0.999_999_9), 0.0);
    }

    #[test]
    fn sign_of_zero_is_kept() {
        assert_eq!(trunc(-0.1).to_bits(), (-0.0f32).to_bits());
        assert_eq!(trunc(0.1).to_bits(), 0.0f32.to_bits());
        assert_eq!(trunc(-0.0).to_bits(), (-0.0f32).to_bits());
        assert_eq!(trunc(-1e-40).to_bits(), (-0.0f32).to_bits()); // dénormalisé
    }

    #[test]
    fn large_values() {
        assert_eq!(trunc(100_000_000.0), 100_000_000.0);
        assert_eq!(trunc(-100_000_000.0), -100_000_000.0);
        assert_eq!(trunc(8_388_607.5), 8_388_607.0); // dernier cas avec fraction
        assert_eq!(trunc(-8_388_607.5), -8_388_607.0);
        assert_eq!(trunc(8_388_608.0), 8_388_608.0); // 2^23
        assert_eq!(trunc(f32::MAX), f32::MAX);
    }

    #[test]
    fn matches_integer_cast() {
        // `as i32` tronque vers zéro : référence indépendante pour |x| < 2^31.
        for i in -5000..=5000 {
            let x = i as f32 * 0.37;
            assert_eq!(trunc(x), (x as i32) as f32, "x={x}");
        }
    }

    #[test]
    fn idempotent() {
        for i in -1000..=1000 {
            let x = i as f32 * 0.731;
            assert_eq!(trunc(trunc(x)), trunc(x), "x={x}");
        }
    }

    #[test]
    fn special_values() {
        assert_eq!(trunc(f32::INFINITY), f32::INFINITY);
        assert_eq!(trunc(f32::NEG_INFINITY), f32::NEG_INFINITY);
        assert!(trunc(f32::NAN).is_nan());
    }
}
