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

- Absolute value
- Arc cosine
- Arc sine
- Square root

Additional functions will be added incrementally.

## License

Licensed under the GNU General Public License, version 2 or later
(`GPL-2.0-or-later`).
