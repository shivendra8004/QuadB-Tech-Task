/*
# Q. Check if a number is prime in Rust.
Code: Same as Question4.rs
*/
fn is_prime(n: u32) -> bool {
        if n <= 1 {
            return false;
        }
        for i in 2..=n / 2 {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
    
    fn main() {
        let num = 7;
        println!("Is {} prime? {}", num, is_prime(num));
    }
    