/*
# Q. Reverse a string in Rust.
Code:
*/
fn reverse_string(s: &str) -> String {
        s.chars().rev().collect()
    }
    
    fn main() {
        let s = "Hello, world!";
        println!("Reversed string of '{}': {}", s, reverse_string(s));
    }
    