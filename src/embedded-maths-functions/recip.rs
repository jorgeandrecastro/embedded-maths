// SPDX-License-Identifier: GPL-2.0-or-later
//! Reciprocal alias.

pub use super::inv::inv as recip;

#[cfg(test)]
mod tests {
    use super::recip;

    #[test]
    fn reciprocal_alias() {
        assert_eq!(recip(4.0), 0.25);
        assert_eq!(recip(-5.0), -0.2);
    }
}
