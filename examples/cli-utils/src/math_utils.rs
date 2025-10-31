//! A collection of mathematical utility functions.
//!
//! This module provides common math operations that can be reused across different applications.
//! # Examples
//! ```
//! use cli_utils::math_utils::{gcd, is_prime};
//!
//! assert_eq!(gcd(48, 18), 6);
//! assert!(is_prime(7));
//! ```

/// Computes the greatest common divisor (GCD) of two numbers using the Euclidean algorithm.
///
/// # Examples
///
/// ```
/// use cli_utils::math_utils::gcd;
/// assert_eq!(gcd(48, 18), 6);
/// assert_eq!(gcd(56, 98), 14);
/// assert_eq!(gcd(0, 5), 5);
/// assert_eq!(gcd(5, 0), 5);
/// ```
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Checks if a number is prime.
///
/// A prime number is a natural number greater than 1 that has no positive divisors other than 1 and itself.
///
/// # Examples
///
/// ```
/// use cli_utils::math_utils::is_prime;
/// assert!(is_prime(7));
/// assert!(!is_prime(10));
/// assert!(!is_prime(0));
/// assert!(!is_prime(1));
/// assert!(is_prime(2));
/// ```
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    // We only need to check for divisors up to the square root of n.
    let limit = (n as f64).sqrt() as u64;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_and_is_prime() {
        assert_eq!(gcd(56, 98), 14);
        assert!(is_prime(97));
        assert!(!is_prime(98));
    }
}