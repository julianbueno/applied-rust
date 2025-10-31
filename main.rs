use my_library::{add, factorial, gcd, is_prime};

fn main() {
    println!("--- Testing my_library from my_app ---");

    // 1. Test add function
    let sum = add(15, 30);
    println!("15 + 30 = {}", sum);

    // 2. Test factorial function
    let fact_6 = factorial(6);
    println!("Factorial of 6 is {}", fact_6);

    // 3. Test gcd function
    let gcd_54_24 = gcd(54, 24);
    println!("GCD of 54 and 24 is {}", gcd_54_24);

    // 4. Test is_prime function
    println!("Is 13 prime? {}", is_prime(13));
    println!("Is 12 prime? {}", is_prime(12));
}