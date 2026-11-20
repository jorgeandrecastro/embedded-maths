# embedded-maths

[![Crates.io](https://img.shields.io/crates/v/embedded-maths.svg)](https://crates.io/crates/embedded-maths)
[![Documentation](https://docs.rs/embedded-maths/badge.svg)](https://docs.rs/embedded-maths)
[![License: GPL v2](https://img.shields.io/badge/License-GPL_v2-blue.svg)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.html)
[![GitHub](https://img.shields.io/badge/GitHub-jorgeandrecastro-181717?logo=github)](https://github.com/jorgeandrecastro/embedded-maths)

`embedded-maths` is a collection of mathematical functions for embedded Rust
and `no_std` environments.

## Square root

The `sqrt` function is implemented by `embedded-f32-sqrt`, a dependency based
on IEEE 754 bit manipulation and Newton-Raphson iterations. It does not use
`micromath`.

```rust
use embedded_maths::sqrt;

let result = sqrt(9.0).unwrap();
assert_eq!(result, 3.0);
```

Negative values return `Err(NegativeInput)`. Zero, `NaN`, and infinity are
handled as described by the `embedded-f32-sqrt` implementation.

## Status

This crate is under active development. The available modules include:

- Basic operations: absolute value, sign, reciprocal, fused multiply-add
- Rounding: ceiling, floor, truncation, rounding, and fractional part
- Root operations: square root and inverse square root
- Exponentials and logarithms: exponential, natural log, log base, log10, log2
- Powers: floating-point power and integer power
- Trigonometry: sine, cosine, tangent, combined sine/cosine
- Inverse trigonometry: arcsine, arccosine, arctangent, four-quadrant arctangent
- Geometry: hypotenuse and hyperbolic cosine
- Euclidean operations: Euclidean division and remainder
- Statistics and vectors: statistics and vector/quaternion/matrix integrations

Additional functions will be added incrementally.

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
