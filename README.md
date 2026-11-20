# embedded-maths

[![Crates.io](https://img.shields.io/crates/v/embedded-maths.svg)](https://crates.io/crates/embedded-maths)
[![Documentation](https://docs.rs/embedded-maths/badge.svg)](https://docs.rs/embedded-maths)
[![License: GPL v2](https://img.shields.io/badge/License-GPL_v2-blue.svg)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.html)
[![GitHub](https://img.shields.io/badge/GitHub-jorgeandrecastro-181717?logo=github)](https://github.com/jorgeandrecastro/embedded-maths)

`embedded-maths` is a collection of mathematical functions for embedded Rust
and `no_std` environments. It provides an extensible scalar API and
dependency-backed implementations for trigonometry, square roots,
exponentials, statistics, vectors, matrices, and quaternions.

## Installation

```toml
[dependencies]
embedded-maths = "0.1"
```

## Function calls

```rust
use embedded_maths::{
    abs, acos, asin, atan, atan2, ceil, copysign, cos, cosh, div_euclid,
    exp, floor, fract, hypot, inv, invsqrt, ln, log, log10, log2, mul_add,
    powf, powi, recip, rem_euclid, round, signum, sin, sin_cos, sqrt, tan,
    trunc,
};
```

### Roots and basic operations

```rust
assert_eq!(abs(-3.25), 3.25);
assert_eq!(sqrt(9.0).unwrap(), 3.0);
assert!((invsqrt(4.0).unwrap() - 0.5).abs() < 1e-3);
assert_eq!(hypot(3.0, 4.0), 5.0);
assert_eq!(inv(4.0), 0.25);
assert_eq!(recip(4.0), 0.25);
assert_eq!(mul_add(2.0, 3.0, 4.0), 10.0);
```

Negative `sqrt` inputs return `Err(NegativeInput)`. Negative or zero
`invsqrt` inputs return `Err(NegativeOrZeroInput)`.

### Trigonometry

```rust
use core::f32::consts::{FRAC_PI_2, FRAC_PI_6};

assert!((sin(FRAC_PI_2) - 1.0).abs() < 1e-6);
assert!((cos(0.0) - 1.0).abs() < 1e-6);
assert!((atan(1.0) - core::f32::consts::FRAC_PI_4).abs() < 1e-6);
assert!((asin(0.5) - FRAC_PI_6).abs() < 1e-6);
assert!((acos(0.5) - 2.0 * FRAC_PI_6).abs() < 1e-6);
assert!((atan2(1.0, 1.0) - core::f32::consts::FRAC_PI_4).abs() < 1e-6);

let (s, c) = sin_cos(core::f32::consts::FRAC_PI_4);
assert!((s - c).abs() < 1e-6);
```

### Exponentials, logarithms, and powers

```rust
assert_eq!(exp(0.0), 1.0);
assert_eq!(ln(1.0), 0.0);
assert_eq!(log2(8.0) as i32, 3);
assert_eq!(log10(100.0) as i32, 2);
assert_eq!(powi(2.0, 10), 1024.0);
assert_eq!(powf(9.0, 0.5) as i32, 3);
assert_eq!(cosh(0.0), 1.0);
```

### Rounding and Euclidean operations

```rust
assert_eq!(ceil(1.2), 2.0);
assert_eq!(floor(-1.2), -2.0);
assert_eq!(round(1.5), 2.0);
assert_eq!(trunc(-1.8), -1.0);
assert_eq!(fract(-3.25), -0.25);
assert_eq!(copysign(3.0, -1.0), -3.0);
assert_eq!(signum(-10.0), -1.0);
assert_eq!(div_euclid(-5.0, 2.0), -3.0);
assert_eq!(rem_euclid(-5.0, 2.0), 1.0);
```

## Algebra and statistics

```rust
use embedded_maths::{Matrix2x2, Quaternion, Vec3, mean, variance, std_dev};

let v = Vec3::new(1.0, 2.0, 3.0);
assert_eq!(v.x(), 1.0);

let q = Quaternion::new(1.0, 0.0, 0.0, 0.0);
assert_eq!(q, Quaternion::ONE);

let m = Matrix2x2::identity();
assert_eq!(m.det(), 1.0);

let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
assert_eq!(mean(&values).unwrap(), 5.0);
assert_eq!(variance(&values).unwrap(), 4.0);
assert_eq!(std_dev(&values).unwrap(), 2.0);
```

## Dependency-backed implementations

Specialized Jorge Andre Castro crates provide the core implementations when
they are available:

- [`embedded-f32-sqrt`](https://crates.io/crates/embedded-f32-sqrt) for square root
- [`embedded-invsqrt-f32`](https://crates.io/crates/embedded-invsqrt-f32) for inverse square root
- [`embedded-exp-f32`](https://crates.io/crates/embedded-exp-f32) for exponential calculations
- [`embedded-trig-f32`](https://crates.io/crates/embedded-trig-f32) for trigonometric calculations
- [`embedded-stats-f32`](https://crates.io/crates/embedded-stats-f32) for statistics
- [`embedded-vector`](https://github.com/jorgeandrecastro/embedded-vector), [`embedded-matrix`](https://github.com/jorgeandrecastro/embedded-matrix), and [`embedded-quaternion-f32`](https://github.com/jorgeandrecastro/embedded-quaternion-f32) for linear algebra

These implementations avoid `micromath` and are designed for `no_std`
embedded targets.

## License

Licensed under the GNU General Public License, version 2 or later
(`GPL-2.0-or-later`).
