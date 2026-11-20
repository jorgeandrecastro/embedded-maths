//! Signe d'un `f32`, sans dépendance externe.
//!
//! Même comportement que `f32::signum` de la bibliothèque standard :
//! `±1.0` selon le bit de signe (zéros signés et infinis inclus), NaN pour NaN.

const SIGN_MASK: u32 = 0x8000_0000;
const ONE_BITS: u32 = 0x3F80_0000; // 1.0f32

/// Renvoie `1.0` ou `-1.0` selon le bit de signe de `x`, ou NaN si `x` est NaN.
///
/// - `+0.0` et `+∞` → `1.0`
/// - `-0.0` et `-∞` → `-1.0`
pub fn signum(x: f32) -> f32 {
    if x.is_nan() {
        return f32::NAN;
    }
    // 1.0 avec le bit de signe de `x` : équivalent de `1.0.copysign(x)`.
    f32::from_bits(ONE_BITS | (x.to_bits() & SIGN_MASK))
}

#[cfg(test)]
mod tests {
    use super::signum;

    #[test]
    fn basic_values() {
        assert_eq!(signum(f32::INFINITY), 1.0);
        assert_eq!(signum(0.0), 1.0);
        assert_eq!(signum(1.0), 1.0);
        assert_eq!(signum(42.5), 1.0);
        assert_eq!(signum(f32::NEG_INFINITY), -1.0);
        assert_eq!(signum(-0.0), -1.0);
        assert_eq!(signum(-1.0), -1.0);
        assert_eq!(signum(-42.5), -1.0);
    }

    #[test]
    fn subnormals_and_extremes() {
        assert_eq!(signum(1e-40), 1.0);
        assert_eq!(signum(-1e-40), -1.0);
        assert_eq!(signum(f32::MAX), 1.0);
        assert_eq!(signum(f32::MIN), -1.0);
    }

    #[test]
    fn nan_stays_nan() {
        assert!(signum(f32::NAN).is_nan());
        assert!(signum(-f32::NAN).is_nan());
    }

    #[test]
    fn matches_std() {
        // `f32::signum` est disponible dans les tests (std) : référence indépendante.
        for i in -1000..=1000 {
            let x = i as f32 * 0.37;
            assert_eq!(signum(x).to_bits(), x.signum().to_bits(), "x={x}");
        }
    }
}