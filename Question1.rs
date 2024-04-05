/*
# Q. Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.
Code:
*/
fn is_palindrome(s: &str) -> bool {
        s.chars().eq(s.chars().rev())
    }
    
    
    fn main() {
        let s = "radar";
        let r = "satellite";
        println!("Is '{}' a palindrome? {}", s, is_palindrome(s));
        println!("Is '{}' a palindrome? {}", r,is_palindrome(r));
    }
    