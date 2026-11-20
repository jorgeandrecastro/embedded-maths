// SPDX-License-Identifier: GPL-2.0-or-later
//! Euclidean division alias.

pub use super::div_euclid::div_euclid;

#[cfg(test)]
mod tests {
    use super::div_euclid;

    #[test]
    fn alias_matches_div_euclid() {
        assert_eq!(div_euclid(-5.0, 2.0), -3.0);
    }
}