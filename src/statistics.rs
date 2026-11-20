// SPDX-License-Identifier: GPL-2.0-or-later
//! Statistical analysis support for `f32` on `no_std`, powered by `embedded-stats-f32`.
//!
//! Provides batch statistics with compensated Kahan summation (`mean`, `variance`,
//! `std_dev`) and constant-memory streaming statistics with Welford's algorithm
//! (`StreamingStats`), with zero heap allocation, zero `libm`, and fail-fast rejection
//! of `NaN` and `±inf` values.
//!
//! # Behavior & Notes
//!
//! | Operation / Input | Behavior |
//! |-------------------|----------|
//! | `mean(&[f32])` | Arithmetic mean via Kahan summation (`Result<f32, StatsError>`) |
//! | `variance(&[f32])` | Population variance (divisor `N`) via Kahan (`Result<f32, StatsError>`) |
//! | `std_dev(&[f32])` | Standard deviation (`sqrt(variance)`) (`Result<f32, StatsError>`) |
//! | `StreamingStats::new()` | Initializes `O(1)` memory streaming accumulator |
//! | `acc.update(x)` | Updates state if finite; preserves state on `Err(StatsError::NonFiniteValue)` |
//! | Empty slice / uninitialized `StreamingStats` | Returns `Err(StatsError::EmptySlice)` |
//! | `NaN` or `±Infinity` input/output | Returns `Err(StatsError::NonFiniteValue)` |

pub use embedded_stats_f32::{
    mean as kahan_mean, std_dev as kahan_std_dev, variance as kahan_variance, StatsError,
    StreamingStats,
};

/// Computes the arithmetic mean of `slice` using Kahan compensated summation.
pub fn mean(slice: &[f32]) -> Result<f32, StatsError> {
    kahan_mean(slice)
}

/// Computes the population variance (divisor `N`) of `slice` using Kahan summation.
pub fn variance(slice: &[f32]) -> Result<f32, StatsError> {
    kahan_variance(slice)
}

/// Computes the population standard deviation of `slice`.
pub fn std_dev(slice: &[f32]) -> Result<f32, StatsError> {
    kahan_std_dev(slice)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batch_and_streaming_stats() {
        let data = [2.0_f32, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        assert!((mean(&data).unwrap() - 5.0).abs() < 1e-6);
        assert!((variance(&data).unwrap() - 4.0).abs() < 1e-5);
        assert!((std_dev(&data).unwrap() - 2.0).abs() < 1e-5);

        let mut acc = StreamingStats::new();
        for &x in &data {
            acc.update(x).unwrap();
        }
        assert!((acc.mean().unwrap() - 5.0).abs() < 1e-6);
        assert!((acc.running_variance().unwrap() - 4.0).abs() < 1e-5);
        assert!((acc.running_std_dev().unwrap() - 2.0).abs() < 1e-5);
    }

    #[test]
    fn error_handling_preserves_state() {
        assert_eq!(mean(&[]), Err(StatsError::EmptySlice));
        assert_eq!(mean(&[f32::INFINITY]), Err(StatsError::NonFiniteValue));

        let mut acc = StreamingStats::new();
        acc.update(10.0).unwrap();
        assert_eq!(acc.update(f32::NAN), Err(StatsError::NonFiniteValue));
        assert_eq!(acc.count(), 1);
        assert!((acc.mean().unwrap() - 10.0).abs() < 1e-6);
    }
}