//! Hardware-friendly `f32` mathematics for Rust `no_std` environments.
//!
//! `embedded-maths` exposes scalar functions, vectors, matrices, quaternions,
//! statistics, and compile-time metadata without requiring `micromath`.
//! Implementations are delegated to focused crates when that provides better
//! accuracy, compatibility, or maintenance boundaries.
//!
//! # Quick example
//!
//! ```rust
//! use embedded_maths::{asin, cos, hypot, sin, sqrt};
//!
//! let angle = 0.5_f32;
//! assert!((sin(angle) - 0.479_425_55).abs() < 1e-6);
//! assert!((cos(angle) - 0.877_582_6).abs() < 1e-6);
//! assert!((asin(0.5) - core::f32::consts::FRAC_PI_6).abs() < 1e-6);
//! assert_eq!(hypot(3.0, 4.0), 5.0);
//! assert_eq!(sqrt(9.0).unwrap(), 3.0);
//! ```
//!
//! # API groups
//!
//! - [`sin()`], [`cos()`], [`tan()`], [`sin_cos()`]
//! - [`asin()`], [`acos()`], [`atan()`], [`atan2()`]
//! - [`sqrt()`], [`invsqrt()`], [`hypot()`]
//! - [`exp()`], [`ln()`], [`log()`], [`log2()`], [`log10()`]
//! - [`powf()`], [`powi()`], [`inv()`], [`recip()`]
//! - [`ceil()`], [`floor()`], [`round()`], [`trunc()`], [`fract()`]
//! - [`abs()`], [`signum()`], [`copysign()`]
//! - [`mean`], [`variance`], [`std_dev`], [`StreamingStats`]
//! - [`Vec2`], [`Vec3`], [`Vec4`], [`Quaternion`]
//! - [`Matrix2x2`], [`Matrix3x3`]
//!
//! # Error behavior
//!
//! Invalid-domain functions generally preserve the behavior documented in
//! their implementation crate: `sqrt` and `invsqrt` return `Result`; inverse
//! trigonometric functions return `NaN` outside their domain; infinities and
//! `NaN` values are handled explicitly where relevant.
//!
//! # License
//!
//! Licensed under `GPL-2.0-or-later`.

#![cfg_attr(not(test), no_std)]

#[path = "embedded-maths-functions/abs.rs"]
pub mod abs;

#[path = "embedded-maths-functions/acos.rs"]
pub mod acos;

#[path = "embedded-maths-functions/asin.rs"]
pub mod asin;

#[path = "embedded-maths-functions/sqrt.rs"]
pub mod sqrt;

#[path = "embedded-maths-functions/atan.rs"]
pub mod atan;
#[path = "embedded-maths-functions/atan2.rs"]
pub mod atan2;
#[path = "embedded-maths-functions/ceil.rs"]
pub mod ceil;
#[path = "embedded-maths-functions/copysign.rs"]
pub mod copysign;
#[path = "embedded-maths-functions/cos.rs"]
pub mod cos;
#[path = "embedded-maths-functions/cosh.rs"]
pub mod cosh;
#[path = "embedded-maths-functions/diveuclid.rs"]
pub mod div_euclid;
#[path = "embedded-maths-functions/diveuclidean.rs"]
pub mod div_euclidean;
#[path = "embedded-maths-functions/exp.rs"]
pub mod exp;
#[path = "embedded-maths-functions/floor.rs"]
pub mod floor;
#[path = "embedded-maths-functions/fract.rs"]
pub mod fract;
#[path = "embedded-maths-functions/hypot.rs"]
pub mod hypot;
#[path = "embedded-maths-functions/inv.rs"]
pub mod inv;
#[path = "embedded-maths-functions/invsqrt.rs"]
pub mod invsqrt;
#[path = "embedded-maths-functions/ln.rs"]
pub mod ln;
#[path = "embedded-maths-functions/log.rs"]
pub mod log;
#[path = "embedded-maths-functions/log10.rs"]
pub mod log10;
#[path = "embedded-maths-functions/log2.rs"]
pub mod log2;
#[path = "embedded-maths-functions/mul_add.rs"]
pub mod mul_add;
#[path = "embedded-maths-functions/powf.rs"]
pub mod powf;
#[path = "embedded-maths-functions/powi.rs"]
pub mod powi;
#[path = "embedded-maths-functions/recip.rs"]
pub mod recip;
#[path = "embedded-maths-functions/rem_euclid.rs"]
pub mod rem_euclid;
#[path = "embedded-maths-functions/round.rs"]
pub mod round;
#[path = "embedded-maths-functions/signum.rs"]
pub mod signum;
#[path = "embedded-maths-functions/sin.rs"]
pub mod sin;
#[path = "embedded-maths-functions/sin_cos.rs"]
pub mod sin_cos;
#[path = "embedded-maths-functions/tan.rs"]
pub mod tan;
#[path = "embedded-maths-functions/trunc.rs"]
pub mod trunc;

#[path = "matrix.rs"]
pub mod matrix;
#[path = "quaternion.rs"]
pub mod quaternion;
#[path = "statistics.rs"]
pub mod statistics;
#[path = "vector.rs"]
pub mod vector;

pub use embedded_stats_f32::{
    StatsError, StreamingStats, mean as kahan_mean, std_dev as kahan_std_dev,
    variance as kahan_variance,
};

pub use abs::abs;
pub use acos::acos;
pub use asin::asin;
pub use atan::atan;
pub use atan2::atan2;
pub use ceil::ceil;
pub use copysign::copysign;
pub use cos::cos;
pub use cosh::cosh;
pub use div_euclid::div_euclid;
pub use div_euclidean::div_euclid as div_euclidean;
pub use exp::exp;
pub use floor::floor;
pub use fract::fract;
pub use hypot::hypot;
pub use inv::inv;
pub use invsqrt::inv_sqrt as invsqrt;
pub use ln::ln;
pub use log::log;
pub use log2::log2;
pub use log10::log10;
pub use matrix::{Matrix2x2, Matrix3x3, MatrixError};
pub use mul_add::mul_add;
pub use powf::powf;
pub use powi::powi;
pub use quaternion::Quaternion;
pub use recip::recip;
pub use rem_euclid::rem_euclid;
pub use round::round;
pub use signum::signum;
pub use sin::sin;
pub use sin_cos::sin_cos;
pub use sqrt::sqrt;
pub use statistics::{mean, std_dev, variance};
pub use tan::tan;
pub use trunc::trunc;
pub use vector::{Vec2, Vec3, Vec4, VecError};
