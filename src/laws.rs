// SPDX-License-Identifier: GPL-2.0-or-later
//! Probability distributions for `f32` on `no_std`, powered by `embedded-laws-f32`.
//!
//! Provides PDF/PMF, CDF, quantile (inverse CDF) and sampling helpers for more
//! than a dozen continuous and discrete laws, with zero heap allocation and zero
//! `libm` (`exp`, `sqrt` and trigonometry come from `embedded-exp-f32`,
//! `embedded-f32-sqrt` and `embedded-trig-f32`).
//!
//! # Conventions
//!
//! `*_pdf` is a density, `*_pmf` a probability mass function, `*_cdf` the
//! cumulative distribution function and `*_quantile` its inverse. Quantile
//! functions return `NaN` for a probability outside `[0, 1]`.
//!
//! # Continuous laws
//!
//! | Law | Functions |
//! |-----|-----------|
//! | Normal `N(0, 1)` | [`normal_pdf()`], [`normal_cdf()`], [`normal_quantile()`] |
//! | Gaussian `N(mu, sigma²)` | [`gaussian_pdf()`], [`gaussian_cdf()`], [`gaussian_quantile()`] |
//! | Uniform `U(a, b)` | [`uniform_pdf()`], [`uniform_cdf()`], [`uniform_quantile()`] |
//! | Exponential `Exp(rate)` | [`exponential_pdf()`], [`exponential_cdf()`], [`exponential_quantile()`], [`exponential_sample()`] |
//! | Laplace `(mu, b)` | [`laplace_pdf()`], [`laplace_cdf()`], [`laplace_quantile()`] |
//! | Logistic `(mu, s)` | [`logistic_pdf()`], [`logistic_cdf()`], [`logistic_quantile()`] |
//! | Cauchy `(x0, gamma)` | [`cauchy_pdf()`], [`cauchy_quantile()`] |
//! | Rayleigh `(sigma)` | [`rayleigh_pdf()`], [`rayleigh_cdf()`], [`rayleigh_quantile()`] |
//! | Weibull `(k, lambda)` | [`weibull_pdf()`], [`weibull_cdf()`], [`weibull_quantile()`] |
//! | Log-normal `(mu, sigma)` | [`lognormal_pdf()`], [`lognormal_cdf()`], [`lognormal_quantile()`] |
//! | Gamma `(k, theta)` | [`gamma_pdf()`] |
//! | Beta `(alpha, beta)` | [`beta_pdf()`] |
//! | Student's t `(nu)` | [`student_t_pdf()`] |
//!
//! # Discrete laws
//!
//! | Law | Functions |
//! |-----|-----------|
//! | Bernoulli `(p)` | [`bernoulli_pmf()`] |
//! | Geometric `(p)` | [`geometric_pmf()`], [`geometric_cdf()`] |
//! | Poisson `(lambda)` | [`poisson_pmf()`], [`poisson_cdf()`] |
//! | Binomial `(n, p)` | [`binomial_pmf()`], [`binomial_cdf()`] |
//!
//! # Helpers
//!
//! | Item | Description |
//! |------|-------------|
//! | [`box_muller()`] | Two independent `N(0, 1)` samples from two uniforms on `(0, 1)` |
//! | [`ln_approx()`] | Natural logarithm accurate over the whole `f32` range |
//! | [`erf_approx()`], [`erfc_approx()`] | Error function and its complement |
//! | [`ln_gamma()`] | `ln Γ(x)` for `x > 0` |
//! | [`Xorshift32`] | Tiny `no_std` PRNG (not cryptographic) with built-in samplers |
//!
//! # Behavior & Notes
//!
//! | Operation / Input | Behavior |
//! |-------------------|----------|
//! | Invalid parameter (e.g. `sigma <= 0`) | Densities return `0.0`; quantiles return `NaN` |
//! | `normal_quantile(p)` | `-inf` at `p = 0`, `+inf` at `p = 1`, `NaN` outside `[0, 1]` |
//! | `poisson` sampler | Knuth's method, suited to modest `lambda` (about 30 or less) |
//! | Targets | Pure `no_std`, no allocation, suited to MCUs (e.g. ESP32-C3, RP2040) |

// Backward-compatible aliases (kept from earlier releases).
pub use embedded_laws_f32::{
    box_muller as law_box_muller, exponential_pdf as law_exponential_pdf,
    gaussian_pdf as law_gaussian_pdf, normal_cdf as law_normal_cdf,
    normal_quantile as law_normal_quantile,
};

pub use embedded_laws_f32::Xorshift32;

/// Generates thin, documented wrappers around `embedded_laws_f32` functions.
macro_rules! wrap {
    ($( $(#[$meta:meta])* $name:ident($($arg:ident: $ty:ty),*) -> $ret:ty; )*) => {
        $(
            $(#[$meta])*
            #[inline]
            pub fn $name($($arg: $ty),*) -> $ret {
                embedded_laws_f32::$name($($arg),*)
            }
        )*
    };
}

wrap! {
    // -- Special functions ---------------------------------------------------

    /// Natural logarithm of `x`, accurate over the whole `f32` range.
    ///
    /// Returns `NaN` for negative input and `-inf` for `0.0`.
    ln_approx(x: f32) -> f32;

    /// Error function `erf(x)` (absolute error below about `1.5e-7`).
    erf_approx(x: f32) -> f32;

    /// Complementary error function `erfc(x)`, accurate in the tails.
    erfc_approx(x: f32) -> f32;

    /// `ln Γ(x)` for `x > 0`; returns `NaN` otherwise.
    ln_gamma(x: f32) -> f32;

    // -- Normal / Gaussian ---------------------------------------------------

    /// Density of the standard normal `N(0, 1)` at `x`.
    normal_pdf(x: f32) -> f32;

    /// Cumulative distribution function of the standard normal `N(0, 1)`.
    normal_cdf(x: f32) -> f32;

    /// Quantile (inverse CDF) of the standard normal `N(0, 1)`.
    normal_quantile(p: f32) -> f32;

    /// Density of the normal distribution `N(mu, sigma²)` at `x`.
    gaussian_pdf(x: f32, mu: f32, sigma: f32) -> f32;

    /// Cumulative distribution function of `N(mu, sigma²)`.
    gaussian_cdf(x: f32, mu: f32, sigma: f32) -> f32;

    /// Quantile (inverse CDF) of `N(mu, sigma²)`.
    gaussian_quantile(p: f32, mu: f32, sigma: f32) -> f32;

    /// Generates two independent `N(0, 1)` samples from two uniforms `u1`, `u2` on `(0, 1)`.
    box_muller(u1: f32, u2: f32) -> (f32, f32);

    // -- Uniform -------------------------------------------------------------

    /// Density of the continuous uniform distribution `U(a, b)` at `x`.
    uniform_pdf(x: f32, a: f32, b: f32) -> f32;

    /// Cumulative distribution function of `U(a, b)`.
    uniform_cdf(x: f32, a: f32, b: f32) -> f32;

    /// Quantile (inverse CDF) of `U(a, b)`.
    uniform_quantile(p: f32, a: f32, b: f32) -> f32;

    // -- Exponential ---------------------------------------------------------

    /// Density of the exponential distribution of parameter `rate` at `x`.
    exponential_pdf(x: f32, rate: f32) -> f32;

    /// Cumulative distribution function of the exponential distribution.
    exponential_cdf(x: f32, rate: f32) -> f32;

    /// Quantile (inverse CDF) of the exponential distribution.
    exponential_quantile(p: f32, rate: f32) -> f32;

    /// Exponential sample from a uniform `u` on `(0, 1)`.
    exponential_sample(u: f32, rate: f32) -> f32;

    // -- Laplace -------------------------------------------------------------

    /// Density of the Laplace distribution with location `mu` and scale `b`.
    laplace_pdf(x: f32, mu: f32, b: f32) -> f32;

    /// Cumulative distribution function of the Laplace distribution.
    laplace_cdf(x: f32, mu: f32, b: f32) -> f32;

    /// Quantile (inverse CDF) of the Laplace distribution.
    laplace_quantile(p: f32, mu: f32, b: f32) -> f32;

    // -- Logistic ------------------------------------------------------------

    /// Density of the logistic distribution with location `mu` and scale `s`.
    logistic_pdf(x: f32, mu: f32, s: f32) -> f32;

    /// Cumulative distribution function of the logistic distribution.
    logistic_cdf(x: f32, mu: f32, s: f32) -> f32;

    /// Quantile (inverse CDF) of the logistic distribution.
    logistic_quantile(p: f32, mu: f32, s: f32) -> f32;

    // -- Cauchy --------------------------------------------------------------

    /// Density of the Cauchy distribution with location `x0` and scale `gamma`.
    cauchy_pdf(x: f32, x0: f32, gamma: f32) -> f32;

    /// Quantile (inverse CDF) of the Cauchy distribution.
    cauchy_quantile(p: f32, x0: f32, gamma: f32) -> f32;

    // -- Rayleigh ------------------------------------------------------------

    /// Density of the Rayleigh distribution of scale `sigma`.
    rayleigh_pdf(x: f32, sigma: f32) -> f32;

    /// Cumulative distribution function of the Rayleigh distribution.
    rayleigh_cdf(x: f32, sigma: f32) -> f32;

    /// Quantile (inverse CDF) of the Rayleigh distribution.
    rayleigh_quantile(p: f32, sigma: f32) -> f32;

    // -- Weibull -------------------------------------------------------------

    /// Density of the Weibull distribution with shape `k` and scale `lambda`.
    weibull_pdf(x: f32, k: f32, lambda: f32) -> f32;

    /// Cumulative distribution function of the Weibull distribution.
    weibull_cdf(x: f32, k: f32, lambda: f32) -> f32;

    /// Quantile (inverse CDF) of the Weibull distribution.
    weibull_quantile(p: f32, k: f32, lambda: f32) -> f32;

    // -- Log-normal ----------------------------------------------------------

    /// Density of the log-normal distribution with log-mean `mu` and log-deviation `sigma`.
    lognormal_pdf(x: f32, mu: f32, sigma: f32) -> f32;

    /// Cumulative distribution function of the log-normal distribution.
    lognormal_cdf(x: f32, mu: f32, sigma: f32) -> f32;

    /// Quantile (inverse CDF) of the log-normal distribution.
    lognormal_quantile(p: f32, mu: f32, sigma: f32) -> f32;

    // -- Gamma / Beta / Student ----------------------------------------------

    /// Density of the Gamma distribution with shape `k` and scale `theta`.
    gamma_pdf(x: f32, k: f32, theta: f32) -> f32;

    /// Density of the Beta distribution `Beta(alpha, beta)` on `[0, 1]`.
    beta_pdf(x: f32, alpha: f32, beta: f32) -> f32;

    /// Density of Student's t distribution with `nu` degrees of freedom.
    student_t_pdf(x: f32, nu: f32) -> f32;

    // -- Discrete laws -------------------------------------------------------

    /// `P(X = k)` for a Bernoulli variable of parameter `p` (`k` is 0 or 1).
    bernoulli_pmf(k: u32, p: f32) -> f32;

    /// `P(X = k)` for a geometric variable counting trials until the first success (`k >= 1`).
    geometric_pmf(k: u32, p: f32) -> f32;

    /// `P(X <= k)` for a geometric variable counting trials until the first success.
    geometric_cdf(k: u32, p: f32) -> f32;

    /// `P(X = k)` for a Poisson variable of mean `lambda`.
    poisson_pmf(k: u32, lambda: f32) -> f32;

    /// `P(X <= k)` for a Poisson variable of mean `lambda`.
    poisson_cdf(k: u32, lambda: f32) -> f32;

    /// `P(X = k)` for a binomial variable `Binomial(n, p)`.
    binomial_pmf(k: u32, n: u32, p: f32) -> f32;

    /// `P(X <= k)` for a binomial variable `Binomial(n, p)`.
    binomial_cdf(k: u32, n: u32, p: f32) -> f32;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f32, b: f32, tol: f32) -> bool {
        (a - b).abs() <= tol
    }

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
        assert!(close(normal_pdf(0.0), 0.398_942_3, 1e-4));
        assert!(close(gaussian_cdf(1.0, 0.0, 1.0), 0.841_344_7, 1e-3));
    }

    #[test]
    fn quantile_roundtrip() {
        assert!((normal_quantile(0.975) - 1.96).abs() < 1e-2);
        assert!(normal_quantile(0.5).abs() < 1e-3);

        let x = normal_quantile(0.9);
        assert!((normal_cdf(x) - 0.9).abs() < 1e-3);

        assert!(close(gaussian_quantile(0.975, 10.0, 2.0), 13.92, 2e-2));
        assert!(normal_quantile(1.5).is_nan());
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

    #[test]
    fn legacy_aliases_match_wrappers() {
        assert_eq!(law_gaussian_pdf(0.3, 0.0, 1.0), gaussian_pdf(0.3, 0.0, 1.0));
        assert_eq!(law_normal_cdf(0.3), normal_cdf(0.3));
        assert_eq!(law_normal_quantile(0.3), normal_quantile(0.3));
        assert_eq!(law_exponential_pdf(0.3, 2.0), exponential_pdf(0.3, 2.0));
        assert_eq!(law_box_muller(0.4, 0.2), box_muller(0.4, 0.2));
    }

    #[test]
    fn special_functions() {
        assert!(close(ln_approx(10.0), 2.302_585, 1e-5));
        assert!(ln_approx(0.0) == f32::NEG_INFINITY);
        assert!(ln_approx(-1.0).is_nan());
        assert!(close(erf_approx(1.0), 0.842_700_8, 1e-5));
        assert!(close(erfc_approx(1.0), 0.157_299_2, 1e-5));
        assert!(close(erf_approx(0.5) + erfc_approx(0.5), 1.0, 1e-6));
        assert!(close(ln_gamma(5.0), 3.178_054, 1e-3)); // ln 24
        assert!(ln_gamma(-1.0).is_nan());
    }

    #[test]
    fn uniform_laws() {
        assert!(close(uniform_pdf(0.5, 0.0, 2.0), 0.5, 1e-6));
        assert_eq!(uniform_pdf(3.0, 0.0, 2.0), 0.0);
        assert!(close(uniform_cdf(0.5, 0.0, 2.0), 0.25, 1e-6));
        assert!(close(uniform_quantile(0.25, 0.0, 2.0), 0.5, 1e-6));
    }

    #[test]
    fn exponential_laws() {
        assert!(close(exponential_cdf(1.0, 1.0), 0.632_120_6, 1e-3));
        assert!(close(exponential_quantile(0.5, 1.0), 0.693_147, 1e-4));
        let x = exponential_quantile(0.63, 2.0);
        assert!(close(exponential_cdf(x, 2.0), 0.63, 1e-4));
        assert!(close(exponential_sample(0.5, 1.0), 0.693_147, 1e-4));
    }

    #[test]
    fn laplace_logistic_cauchy() {
        assert!(close(laplace_pdf(0.0, 0.0, 1.0), 0.5, 1e-4));
        assert!(close(laplace_cdf(0.0, 0.0, 1.0), 0.5, 1e-4));
        assert!(close(laplace_cdf(laplace_quantile(0.3, 1.0, 2.0), 1.0, 2.0), 0.3, 1e-4));

        assert!(close(logistic_pdf(0.0, 0.0, 1.0), 0.25, 1e-4));
        assert!(close(logistic_cdf(0.0, 0.0, 1.0), 0.5, 1e-4));
        assert!(close(logistic_cdf(logistic_quantile(0.8, 1.0, 2.0), 1.0, 2.0), 0.8, 1e-4));

        assert!(close(cauchy_pdf(0.0, 0.0, 1.0), 0.318_309_9, 1e-4));
        assert!(close(cauchy_quantile(0.75, 0.0, 1.0), 1.0, 1e-3));
        assert!(close(cauchy_quantile(0.5, 2.0, 1.0), 2.0, 1e-3));
    }

    #[test]
    fn rayleigh_weibull_lognormal() {
        assert!(close(rayleigh_pdf(1.0, 1.0), 0.606_531, 1e-3));
        assert!(close(rayleigh_cdf(1.0, 1.0), 0.393_469, 1e-3));
        assert!(close(rayleigh_cdf(rayleigh_quantile(0.7, 1.5), 1.5), 0.7, 1e-4));

        // Weibull with k = 1 reduces to an exponential of rate 1 / lambda
        assert!(close(weibull_cdf(1.0, 1.0, 2.0), 0.393_469, 1e-3));
        assert!(close(weibull_pdf(1.0, 1.0, 2.0), exponential_pdf(1.0, 0.5), 1e-3));
        assert!(close(weibull_cdf(weibull_quantile(0.4, 1.5, 2.0), 1.5, 2.0), 0.4, 1e-3));

        assert!(close(lognormal_pdf(1.0, 0.0, 1.0), 0.398_942_3, 1e-3));
        assert!(close(lognormal_cdf(1.0, 0.0, 1.0), 0.5, 1e-3));
        assert!(close(lognormal_quantile(0.5, 0.0, 1.0), 1.0, 1e-3));
    }

    #[test]
    fn gamma_beta_student() {
        // Gamma(2, 1): x * exp(-x) at x = 2
        assert!(close(gamma_pdf(2.0, 2.0, 1.0), 0.270_671, 1e-3));
        // Beta(2, 2) at 0.5: 1.5
        assert!(close(beta_pdf(0.5, 2.0, 2.0), 1.5, 1e-3));
        // Student with 1 degree of freedom is Cauchy
        assert!(close(student_t_pdf(0.0, 1.0), 0.318_309_9, 1e-3));
    }

    #[test]
    fn discrete_laws() {
        assert_eq!(bernoulli_pmf(1, 0.3), 0.3);
        assert!(close(bernoulli_pmf(0, 0.3), 0.7, 1e-6));
        assert_eq!(bernoulli_pmf(2, 0.3), 0.0);

        assert!(close(geometric_pmf(3, 0.5), 0.125, 1e-4));
        assert!(close(geometric_cdf(3, 0.5), 0.875, 1e-4));

        assert!(close(poisson_pmf(2, 3.0), 0.224_042, 1e-3));
        assert!(close(poisson_cdf(40, 5.0), 1.0, 1e-4));

        assert!(close(binomial_pmf(3, 10, 0.5), 0.117_187_5, 1e-3));
        assert!(close(binomial_cdf(10, 10, 0.3), 1.0, 1e-4));
    }

    #[test]
    fn xorshift_rng() {
        let mut a = Xorshift32::new(0);
        let mut b = Xorshift32::new(0x9E37_79B9);
        // A zero seed is remapped to a fixed non-zero constant.
        assert_eq!(a.next_u32(), b.next_u32());

        let mut rng = Xorshift32::new(42);
        let n = 2_000;
        let mut sum = 0.0;
        for _ in 0..n {
            let u = rng.next_f32();
            assert!(u > 0.0 && u < 1.0);
            sum += u;
        }
        assert!(close(sum / n as f32, 0.5, 0.05));

        assert!(rng.exponential(2.0) >= 0.0);
        assert!(rng.geometric(0.5) >= 1);
    }
}