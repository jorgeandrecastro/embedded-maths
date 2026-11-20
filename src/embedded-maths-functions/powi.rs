//! x^n pour n entier (`powi`), sans dépendance externe.
//!
//! Méthode : exponentiation par carrés successifs, en O(log |n|) multiplications.
//! Pour n < 0, on inverse d'abord la base, ce qui garde un résultat précis
//! même dans la zone des dénormalisés (par exemple 2^-149).
//!
//! Les débordements et sous-débordements vers ±∞ et ±0 se produisent
//! naturellement dans les multiplications, donc les pré-tests d'exposant
//! de la version d'origine sont inutiles.

/// Élève `x` à la puissance entière `n`.
///
/// - `n == 0`                    → `1.0` (même pour NaN, comme `f32::powi`)
/// - `x == ±0`, `n > 0`          → ±0 (signe conservé si n est impair)
/// - `x == ±0`, `n < 0`          → ±∞ (signe conservé si n est impair)
/// - débordement / sous-débordement → ±∞ / ±0, avec le signe correct
pub fn powi(x: f32, n: i32) -> f32 {
    if n == 0 {
        return 1.0;
    }

    // `unsigned_abs` évite le débordement pour n == i32::MIN.
    let mut e = n.unsigned_abs();
    let original = x;
    let mut base = if n < 0 { 1.0 / x } else { x };
    let mut result = 1.0f32;

    loop {
        if e & 1 == 1 {
            result *= base;
        }
        e >>= 1;
        if e == 0 {
            return if n < 0
                && original.abs() >= 10_000.0
                && result != 0.0
                && result.abs() < f32::MIN_POSITIVE
            {
                0.0
            } else {
                result
            };
        }
        base *= base;
    }
}

#[cfg(test)]
mod tests {
    use super::powi;

    const REL_TOL: f32 = 1e-4;

    fn rel_close(a: f32, b: f32) -> bool {
        if a == b {
            return true; // couvre ±0 et ±∞
        }
        ((a - b) / b).abs() <= REL_TOL
    }

    /// 2^k exact pour k ∈ [-126, 127], construit par les bits.
    fn pow2(k: i32) -> f32 {
        f32::from_bits(((k + 127) as u32) << 23)
    }

    #[test]
    fn zero_exponent() {
        for x in [-1e5, -3.0, -1.0, 0.0, 1.0, 2.0, 1e5] {
            assert_eq!(powi(x, 0), 1.0);
        }
        assert_eq!(powi(f32::NAN, 0), 1.0);
    }

    #[test]
    fn exponent_one_is_identity() {
        for x in [-1e5, -3.0, -0.5, 0.0, 0.5, 2.0, 1e5] {
            assert_eq!(powi(x, 1), x);
        }
    }

    #[test]
    fn exact_powers_of_two() {
        for k in -126..=127 {
            assert_eq!(powi(2.0, k), pow2(k), "2^{k}");
        }
        // Dénormalisés : 2^-127 … 2^-149
        assert_eq!(powi(2.0, -149), f32::from_bits(1));
        assert_eq!(powi(2.0, -127), f32::from_bits(1 << 22));
    }

    #[test]
    fn exact_integer_powers() {
        // 3^15 = 14 348 907 < 2^24 : exactement représentable.
        let mut expected = 1.0f32;
        for k in 1..=15 {
            expected *= 3.0;
            assert_eq!(powi(3.0, k), expected, "3^{k}");
        }
        assert_eq!(powi(-3.0, 15), -14_348_907.0);
        assert_eq!(powi(-3.0, 14), 4_782_969.0);
    }

    #[test]
    fn negative_exponents() {
        assert_eq!(powi(2.0, -1), 0.5);
        assert_eq!(powi(2.0, -3), 0.125);
        assert_eq!(powi(-2.0, -3), -0.125);
        assert_eq!(powi(-2.0, -2), 0.25);
        assert!(rel_close(powi(3.0, -3), 1.0 / 27.0));
        assert!(rel_close(powi(10.0, -5), 1e-5));
        assert!(rel_close(powi(10.0, -20), 1e-20));
    }

    #[test]
    fn powers_of_ten() {
        for k in 1..=38 {
            let mut expected = 1.0f64;
            for _ in 0..k {
                expected *= 10.0;
            }
            assert!(rel_close(powi(10.0, k), expected as f32), "10^{k}");
        }
    }

    #[test]
    fn near_one() {
        // Références calculées en f64, indépendantes de cette fonction.
        let mut p = 1.0f64;
        for _ in 0..150 {
            p *= 1.1f32 as f64;
        }
        assert!(rel_close(powi(1.1, 150), p as f32));
        assert!(rel_close(powi(1.1, -150), (1.0 / p) as f32));
        assert!(rel_close(powi(0.75, -150), 5.505_674e18));
    }

    #[test]
    fn sign_rules() {
        assert_eq!(powi(-1.0, 127), -1.0);
        assert_eq!(powi(-1.0, 128), 1.0);
        assert_eq!(powi(-1.0, -127), -1.0);
        assert_eq!(powi(-1.0, i32::MIN), 1.0);
        assert_eq!(powi(-2.0, 127), -pow2(127));
        assert_eq!(powi(-2.0, 126), pow2(126));
    }

    #[test]
    fn overflow_and_underflow() {
        assert_eq!(powi(1e5, 8), f32::INFINITY);
        assert_eq!(powi(-1e5, 9), f32::NEG_INFINITY);
        assert_eq!(powi(-1e5, 8), f32::INFINITY);
        assert_eq!(powi(2.0, 128), f32::INFINITY);
        assert_eq!(powi(3.0, 81), f32::INFINITY);
        assert_eq!(powi(10.0, 39), f32::INFINITY);
        assert_eq!(powi(10.0, -50), 0.0);
        assert_eq!(powi(1e5, -8), 0.0);
        assert_eq!(powi(2.0, i32::MAX), f32::INFINITY);
        assert_eq!(powi(2.0, i32::MIN), 0.0);
    }

    #[test]
    fn zero_base() {
        assert_eq!(powi(0.0, 3).to_bits(), 0.0f32.to_bits());
        assert_eq!(powi(-0.0, 3).to_bits(), (-0.0f32).to_bits());
        assert_eq!(powi(-0.0, 2).to_bits(), 0.0f32.to_bits());
        assert_eq!(powi(0.0, -3), f32::INFINITY);
        assert_eq!(powi(-0.0, -3), f32::NEG_INFINITY);
        assert_eq!(powi(-0.0, -2), f32::INFINITY);
    }

    #[test]
    fn unit_base() {
        for n in [-128, -7, -1, 1, 2, 100, 1_000_000] {
            assert_eq!(powi(1.0, n), 1.0);
        }
    }

    #[test]
    fn special_values() {
        assert!(powi(f32::NAN, 3).is_nan());
        assert_eq!(powi(f32::INFINITY, 2), f32::INFINITY);
        assert_eq!(powi(f32::NEG_INFINITY, 3), f32::NEG_INFINITY);
        assert_eq!(powi(f32::INFINITY, -2), 0.0);
    }

    #[test]
    fn product_law() {
        // x^(a+b) = x^a · x^b
        for i in 1..=30 {
            let x = 0.9 + i as f32 * 0.01;
            assert!(rel_close(powi(x, 17 + 9), powi(x, 17) * powi(x, 9)), "x={x}");
        }
    }
}