// SPDX-License-Identifier: GPL-2.0-or-later
//! 2D, 3D, and 4D `f32` vectors for `no_std` embedded systems,
//! powered by `embedded-vector`.
//!
//! Provides `Vec2`, `Vec3`, and `Vec4` with operator overloads, dot/cross/triple
//! products, projections/reflections, homogeneous `perspective_divide`, and
//! numerically robust `norm` and `normalize` without heap allocation or `std`.
//!
//! # Behavior & Notes
//!
//! | Operation / Input | Behavior |
//! |-------------------|----------|
//! | `Vec2::new` / `Vec3::new` / `Vec4::new` | Builds 2D, 3D, or 4D `f32` vectors (`ZERO`, `ONE`, `Default`) |
//! | `norm()` | Euclidean norm (`Result<f32, VecError>`); `Ok(0.0)` for zero vector |
//! | `normalize()` | Unit vector (`Result<Self, VecError>`); returns `Err(VecError::ZeroNorm)` on zero vector |
//! | `normalize_or_zero()` | Returns unit vector or `Self::ZERO` without error |
//! | `checked_div(s)` | Scalar division returning `Err(VecError::DivisionByZero)` or `Err(VecError::NonFiniteValue)` |
//! | `perspective_divide()` | Converts homogeneous `Vec4` to `Vec3` (`Err(VecError::DivisionByZero)` if `w == 0.0`) |
//! | Extreme values (`1e30` / `1e-30`) | `norm`, `normalize`, `distance`, and `project_onto` handle scaling robustly |

pub use embedded_vector::{Vec2, Vec3, Vec4, VecError};

/// Computes the Euclidean norm of a `Vec3`.
pub fn norm_vec3(v: Vec3) -> Result<f32, VecError> {
    v.norm()
}

/// Normalizes a `Vec3` into a unit vector, or returns `Err(VecError::ZeroNorm)` if zero.
pub fn normalize_vec3(v: Vec3) -> Result<Vec3, VecError> {
    v.normalize()
}

/// Computes the cross product of two `Vec3` values (`a × b`).
pub fn cross_vec3(a: Vec3, b: Vec3) -> Vec3 {
    a.cross(&b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_ops_and_products() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(a + b, Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(a.dot(&b), 32.0);
        assert_eq!(cross_vec3(a, b), Vec3::new(-3.0, 6.0, -3.0));
        assert_eq!(2.0 * a, Vec3::new(2.0, 4.0, 6.0));

        let n = norm_vec3(a).unwrap();
        assert!((n - 14.0_f32.sqrt()).abs() < 1e-5);

        let u = normalize_vec3(a).unwrap();
        assert!((u.norm().unwrap() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn error_cases() {
        assert_eq!(normalize_vec3(Vec3::ZERO), Err(VecError::ZeroNorm));
        assert_eq!(Vec3::ZERO.normalize_or_zero(), Vec3::ZERO);
        assert_eq!(Vec3::ONE.checked_div(0.0), Err(VecError::DivisionByZero));
        assert_eq!(
            Vec4::new(1.0, 2.0, 3.0, 0.0).perspective_divide(),
            Err(VecError::DivisionByZero)
        );
    }
}