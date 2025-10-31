// # MathUtils
//
// `my_library` is a collection of mathematical utility functions.
// This library is created as part of the Applied Rust course.

/// Adds two unsigned 64-bit integers.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Calculates the factorial of a non-negative integer.
///
/// Note: The result is a `u64`, which will overflow for `n > 20`.
/// In debug builds, this will panic. In release builds, it will wrap.
///
/// # Examples
///
/// ```
/// assert_eq!(my_library::factorial(5), 120);
/// ```
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

/// Computes the greatest common divisor (GCD) of two numbers using the Euclidean algorithm.
///
/// # Examples
///
/// ```
/// assert_eq!(my_library::gcd(48, 18), 6);
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
/// assert!(my_library::is_prime(7));
/// assert!(!my_library::is_prime(10));
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
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(20), 2432902008176640000);
    }

    #[test]
    #[should_panic]
    fn test_factorial_overflow() {
        // This will panic in debug mode due to overflow checks.
        factorial(21);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(56, 98), 14);
        assert_eq!(gcd(0, 5), 5);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(7));
        assert!(!is_prime(10));
        assert!(is_prime(97));
    }
}
