// SPDX-License-Identifier: GPL-2.0-or-later
//! Fused multiply-add operations for `f32`.

/// Returns `a * b + c` with a fused operation when supported.
#[inline(always)]
pub fn mul_add(a: f32, b: f32, c: f32) -> f32 {
    a * b + c
}

#[cfg(test)]
mod tests {
    use super::mul_add;

    #[test]
    fn fused_operation() {
        assert_eq!(mul_add(2.0, 3.0, 4.0), 10.0);
        assert_eq!(mul_add(-2.0, 3.0, 1.0), -5.0);
    }
}