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
#[path = "embedded-maths-functions/cosh.rs"]
pub mod cosh;
#[path = "embedded-maths-functions/cos.rs"]
pub mod cos;
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

#[path = "statistics.rs"]
pub mod statistics;
#[path = "vector.rs"]
pub mod vector;
#[path = "quaternion.rs"]
pub mod quaternion;
#[path = "matrix.rs"]
pub mod matrix;

pub use embedded_stats_f32::{
    mean as kahan_mean, std_dev as kahan_std_dev, variance as kahan_variance, StatsError,
    StreamingStats,
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
pub use log10::log10;
pub use log2::log2;
pub use mul_add::mul_add;
pub use powf::powf;
pub use powi::powi;
pub use recip::recip;
pub use rem_euclid::rem_euclid;
pub use round::round;
pub use signum::signum;
pub use sin::sin;
pub use sin_cos::sin_cos;
pub use sqrt::sqrt;
pub use tan::tan;
pub use trunc::trunc;
pub use statistics::{mean, variance, std_dev};
pub use vector::{Vec2, Vec3, Vec4, VecError};
pub use quaternion::Quaternion;
pub use matrix::{Matrix2x2, Matrix3x3, MatrixError};
