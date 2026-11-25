// SPDX-License-Identifier: GPL-2.0-or-later
//! Probability distributions for `f32` on `no_std`, powered by `embedded-laws-f32`.
//!
//! Provides the Gaussian density, the standard normal CDF and quantile, Box-Muller
//! normal sampling and the exponential density, with zero heap allocation and zero
//! `libm` (`exp` and `sqrt` come from `embedded-exp-f32` and `embedded-f32-sqrt`).
//!
//! # Behavior & Notes
//!
//! | Operation / Input | Behavior |
//! |-------------------|----------|
//! | `gaussian_pdf(x, mu, sigma)` | Density of `N(mu, sigma²)` at `x` |
//! | `normal_cdf(x)` | `P(X <= x)` for `X ~ N(0, 1)` |
//! | `normal_quantile(p)` | Inverse CDF of `N(0, 1)`, `p` in `(0, 1)` |
//! | `box_muller(u1, u2)` | Two independent `N(0, 1)` samples from two uniforms on `(0, 1)` |
//! | `exponential_pdf(x, rate)` | Density of the exponential distribution of parameter `rate` |
//! | Targets | Pure `no_std`, no allocation, suited to MCUs (e.g. ESP32-C3, RP2040) |

pub use embedded_laws_f32::{
    box_muller as law_box_muller, exponential_pdf as law_exponential_pdf,
    gaussian_pdf as law_gaussian_pdf, normal_cdf as law_normal_cdf,
    normal_quantile as law_normal_quantile,
};

/// Density of the normal distribution `N(mu, sigma²)` at `x`.
pub fn gaussian_pdf(x: f32, mu: f32, sigma: f32) -> f32 {
    law_gaussian_pdf(x, mu, sigma)
}

/// Cumulative distribution function of the standard normal `N(0, 1)`.
pub fn normal_cdf(x: f32) -> f32 {
    law_normal_cdf(x)
}

/// Quantile (inverse CDF) of the standard normal `N(0, 1)`.
pub fn normal_quantile(p: f32) -> f32 {
    law_normal_quantile(p)
}

/// Generates two independent `N(0, 1)` samples from two uniforms `u1`, `u2` on `(0, 1)`.
pub fn box_muller(u1: f32, u2: f32) -> (f32, f32) {
    law_box_muller(u1, u2)
}

/// Density of the exponential distribution of parameter `rate` at `x`.
pub fn exponential_pdf(x: f32, rate: f32) -> f32 {
    law_exponential_pdf(x, rate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gaussian_and_cdf() {
        // N(0, 1) density at 0: 1 / sqrt(2π)
        assert!((gaussian_pdf(0.0, 0.0, 1.0) - 0.398_942_3).abs() < 1e-4);
        // Symmetry around mu
        let a = gaussian_pdf(3.0, 2.0, 1.5);
        let b = gaussian_pdf(1.0, 2.0, 1.5);
        assert!((a - b).abs() < 1e-5);

        assert!((normal_cdf(0.0) - 0.5).abs() < 1e-4);
        assert!((normal_cdf(1.0) - 0.841_344_7).abs() < 1e-3);
    }

    #[test]
    fn quantile_roundtrip() {
        assert!((normal_quantile(0.975) - 1.96).abs() < 1e-2);
        assert!(normal_quantile(0.5).abs() < 1e-3);

        let x = normal_quantile(0.9);
        assert!((normal_cdf(x) - 0.9).abs() < 1e-3);
    }

    #[test]
    fn box_muller_and_exponential() {
        // z1² + z2² = -2 ln(u1) for the standard Box-Muller transform
        let (z1, z2) = box_muller(0.42, 0.13);
        let r2 = z1 * z1 + z2 * z2;
        assert!((r2 - 1.735_1).abs() < 1e-2);

        // rate * exp(-rate * x) = 2 * exp(-1)
        assert!((exponential_pdf(0.5, 2.0) - 0.735_758_9).abs() < 1e-3);
    }
}
