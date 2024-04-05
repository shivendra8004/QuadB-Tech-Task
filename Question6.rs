/*
# Q. Implement a function that finds the longest common prefix of a given set of strings.
Code:
*/
fn longest_common_prefix(strings: &[&str]) -> String {
        if strings.is_empty() {
            return String::new();
        }
        let mut prefix = strings[0].to_string();
        for &s in &strings[1..] {
            while !s.starts_with(&prefix) {
                prefix.pop();
            }
        }
        prefix
    }
    
    fn main() {
        let strings = vec!["flower", "flow", "flight"];
        println!("Longest common prefix of {:?}: {}", strings, longest_common_prefix(&strings));
    }
    