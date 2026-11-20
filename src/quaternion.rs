// SPDX-License-Identifier: GPL-2.0-or-later
//! 4D quaternions and perspective projection (`f32`, `no_std`),
//! powered by `embedded-quaternion-f32`.
//!
//! Provides Hamilton multiplication, 4D rotations in the `X-W` and `Y-Z` planes,
//! norm/normalization helpers, and 4D -> 3D -> 2D perspective projection without
//! heap allocation or `libm`.
//!
//! # Behavior & Notes
//!
//! | Operation / Input | Behavior |
//! |-------------------|----------|
//! | `Quaternion::new(w, x, y, z)` | Builds a quaternion from `(w, x, y, z)` |
//! | `Quaternion::ZERO` / `Quaternion::ONE` | Additive identity / Hamilton unit identity |
//! | `rotate_xw(angle)` / `rotate_yz(angle)` | 4D rotations in the `X-W` and `Y-Z` planes |
//! | `norm_squared()` | Squared norm without `sqrt` |
//! | `normalize()` | Returns `Some(unit)` or `None` for `Quaternion::ZERO` |
//! | `project_2d(cx, cy, dist_4d, dist_3d)` | Returns `(screen_x: i16, screen_y: i16, depth: f32)` |
//!
//! Because `embedded-quaternion-f32` relies on fast trigonometric approximations,
//! periodically call [`normalize`] during long chains of composed rotations to
//! limit numerical drift.

pub use embedded_quaternion_f32::Quaternion;

/// Normalizes a quaternion, returning `None` if it is zero (`Quaternion::ZERO`).
pub fn normalize(q: Quaternion) -> Option<Quaternion> {
    q.normalize()
}

/// Rotates a quaternion in the `X-W` plane by `angle` (in radians).
pub fn rotate_xw(q: Quaternion, angle: f32) -> Quaternion {
    q.rotate_xw(angle)
}

/// Rotates a quaternion in the `Y-Z` plane by `angle` (in radians).
pub fn rotate_yz(q: Quaternion, angle: f32) -> Quaternion {
    q.rotate_yz(angle)
}

/// Projects a 4D quaternion point through 3D to 2D screen coordinates.
pub fn project_2d(
    q: Quaternion,
    cx: f32,
    cy: f32,
    dist_4d: f32,
    dist_3d: f32,
) -> (i16, i16, f32) {
    q.project_2d(cx, cy, dist_4d, dist_3d)
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32::consts::FRAC_PI_2;

    #[test]
    fn identities_and_arithmetic() {
        let q = Quaternion::new(1.0, 0.0, 0.0, 0.0);
        assert_eq!(q, Quaternion::ONE);
        assert_eq!(Quaternion::ZERO.norm_squared(), 0.0);
        assert_eq!(normalize(Quaternion::ZERO), None);

        let a = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let b = Quaternion::new(0.5, 0.5, 0.5, 0.5);
        assert_eq!(a + b, Quaternion::new(1.5, 2.5, 3.5, 4.5));
        assert_eq!(a - b, Quaternion::new(0.5, 1.5, 2.5, 3.5));
        assert_eq!(a * 2.0, Quaternion::new(2.0, 4.0, 6.0, 8.0));
        assert_eq!(a.conjugate(), Quaternion::new(1.0, -2.0, -3.0, -4.0));
    }

    #[test]
    fn normalization() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let unit = normalize(q).expect("non-zero quaternion should normalize");
        assert!((unit.norm_squared() - 1.0).abs() < 1e-3);
    }

    #[test]
    fn rotation_and_projection() {
        let p = Quaternion::new(0.0, 1.0, 0.0, 0.0);
        let rotated = rotate_xw(p, FRAC_PI_2);
        let (sx, sy, depth) = project_2d(rotated, 160.0, 80.0, 5.0, 5.0);
        assert!((i16::MIN..=i16::MAX).contains(&sx));
        assert!((i16::MIN..=i16::MAX).contains(&sy));
        assert!(depth.is_finite());
    }
}