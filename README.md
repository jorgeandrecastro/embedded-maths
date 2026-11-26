# embedded-maths

[![Crates.io](https://img.shields.io/crates/v/embedded-maths.svg)](https://crates.io/crates/embedded-maths)
[![Documentation](https://docs.rs/embedded-maths/badge.svg)](https://docs.rs/embedded-maths)
[![CI](https://github.com/jorgeandrecastro/embedded-maths/actions/workflows/ci.yml/badge.svg)](https://github.com/jorgeandrecastro/embedded-maths/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20or%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

[![GitHub](https://img.shields.io/badge/GitHub-jorgeandrecastro-181717?logo=github)](https://github.com/jorgeandrecastro/embedded-maths)

`embedded-maths` is a collection of mathematical functions for embedded Rust
and `no_std` environments. It provides an extensible scalar API and
dependency-backed implementations for trigonometry, square roots,
exponentials, statistics, probability distributions, vectors, matrices, and
quaternions.

## Installation

```toml
[dependencies]
embedded-maths = "0.6"
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

## Probability distributions

More than a dozen continuous and discrete laws with density or mass function,
CDF, quantile (inverse CDF) and sampling, in `f32`, with no heap allocation
and no `libm`.

Convention: `*_pdf` is a density, `*_pmf` a probability mass function,
`*_cdf` the cumulative distribution function and `*_quantile` its inverse.

```rust
use embedded_maths::{
    Xorshift32, box_muller, exponential_cdf, exponential_pdf, gaussian_cdf,
    gaussian_pdf, gaussian_quantile, normal_cdf, normal_quantile, poisson_pmf,
    weibull_cdf,
};

// Density of N(0, 1) at x = 0: ≈ 0.39894228
assert!((gaussian_pdf(0.0, 0.0, 1.0) - 0.398_942_3).abs() < 1e-4);

// P(X <= 1) for X ~ N(0, 1): ≈ 0.8413447
assert!((normal_cdf(1.0) - 0.841_344_7).abs() < 1e-3);

// P(X <= 25) for a sensor reading X ~ N(20, 3²): ≈ 0.9522
assert!((gaussian_cdf(25.0, 20.0, 3.0) - 0.952_2).abs() < 1e-3);

// Value x such that P(X <= x) = 0.975: ≈ 1.96
assert!((normal_quantile(0.975) - 1.96).abs() < 1e-2);

// 95 % upper bound for X ~ N(20, 3²): ≈ 24.93
assert!((gaussian_quantile(0.975, 20.0, 3.0) - 24.88).abs() < 1e-1);

// Density at x = 0.5 for a rate (lambda) of 2.0: 2 * exp(-1)
assert!((exponential_pdf(0.5, 2.0) - 0.735_758_9).abs() < 1e-3);

// Probability that a component fails before t = 1000 h
let p_weibull = weibull_cdf(1000.0, 1.5, 2000.0);
let p_exp = exponential_cdf(1000.0, 1.0 / 2000.0);
assert!(p_weibull > 0.0 && p_exp > 0.0);

// P(X = 2) for X ~ Poisson(3): ≈ 0.2240
assert!((poisson_pmf(2, 3.0) - 0.224_042).abs() < 1e-3);

// Two independent N(0, 1) samples from two uniforms on (0, 1),
// e.g. coming from a hardware RNG
let (_z1, _z2) = box_muller(0.42, 0.13);

// Or use the built-in PRNG (not cryptographic)
let mut rng = Xorshift32::new(0xC0FFEE);
let _noise = rng.normal(0.0, 0.5);
let _wait = rng.exponential(2.0);
let _hits = rng.poisson(4.0);
```

### Continuous laws

| Law | Functions |
|-----|-----------|
| Normal `N(0, 1)` | `normal_pdf`, `normal_cdf`, `normal_quantile` |
| Gaussian `N(mu, sigma²)` | `gaussian_pdf`, `gaussian_cdf`, `gaussian_quantile` |
| Uniform `U(a, b)` | `uniform_pdf`, `uniform_cdf`, `uniform_quantile` |
| Exponential `Exp(rate)` | `exponential_pdf`, `exponential_cdf`, `exponential_quantile`, `exponential_sample` |
| Laplace `(mu, b)` | `laplace_pdf`, `laplace_cdf`, `laplace_quantile` |
| Logistic `(mu, s)` | `logistic_pdf`, `logistic_cdf`, `logistic_quantile` |
| Cauchy `(x0, gamma)` | `cauchy_pdf`, `cauchy_quantile` |
| Rayleigh `(sigma)` | `rayleigh_pdf`, `rayleigh_cdf`, `rayleigh_quantile` |
| Weibull `(k, lambda)` | `weibull_pdf`, `weibull_cdf`, `weibull_quantile` |
| Log-normal `(mu, sigma)` | `lognormal_pdf`, `lognormal_cdf`, `lognormal_quantile` |
| Gamma `(k, theta)` | `gamma_pdf` |
| Beta `(alpha, beta)` | `beta_pdf` |
| Student's t `(nu)` | `student_t_pdf` |

### Discrete laws

| Law | Functions |
|-----|-----------|
| Bernoulli `(p)` | `bernoulli_pmf` |
| Geometric `(p)` | `geometric_pmf`, `geometric_cdf` |
| Poisson `(lambda)` | `poisson_pmf`, `poisson_cdf` |
| Binomial `(n, p)` | `binomial_pmf`, `binomial_cdf` |

### Helpers

| Item | Description |
|------|-------------|
| `box_muller(u1, u2)` | Two independent `N(0, 1)` samples from two uniforms on `(0, 1)` |
| `ln_approx(x)` | Natural logarithm, accurate over the whole `f32` range |
| `erf_approx(x)`, `erfc_approx(x)` | Error function and its complement |
| `ln_gamma(x)` | `ln Γ(x)` for `x > 0` |
| `Xorshift32` | Tiny `no_std` PRNG with built-in samplers (`normal`, `exponential`, `poisson`, ...) |

All computations are done in single precision. `erfc` has an absolute error
below about `1.5e-7`, and CDF/quantile round trips agree to about `1e-4`. The
`Xorshift32::poisson` sampler uses Knuth's method and is only suitable for
modest `lambda` (about 30 or less).

## Dependency-backed implementations

Specialized Jorge Andre Castro crates provide the core implementations when
they are available:

- [`embedded-f32-sqrt`](https://crates.io/crates/embedded-f32-sqrt) for square root
- [`embedded-invsqrt-f32`](https://crates.io/crates/embedded-invsqrt-f32) for inverse square root
- [`embedded-exp-f32`](https://crates.io/crates/embedded-exp-f32) for exponential calculations
- [`embedded-trig-f32`](https://crates.io/crates/embedded-trig-f32) for trigonometric calculations
- [`embedded-stats-f32`](https://crates.io/crates/embedded-stats-f32) for statistics
- [`embedded-laws-f32`](https://crates.io/crates/embedded-laws-f32) for probability distributions
- [`embedded-vector`](https://github.com/jorgeandrecastro/embedded-vector), [`embedded-matrix`](https://github.com/jorgeandrecastro/embedded-matrix), and [`embedded-quaternion-f32`](https://github.com/jorgeandrecastro/embedded-quaternion-f32) for linear algebra


## 🛡️ License

Licensed under either of:

    MIT license (http://opensource.org/licenses/MIT)

    Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## 🦅 About

Developed by Jorge Andre Castro.