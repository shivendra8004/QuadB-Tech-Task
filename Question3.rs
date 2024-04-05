/*
# Q. Given a string of words, implement a function that returns the shortest word in the string.
Code:
*/
fn shortest_word(s: &str) -> Option<&str> {
        s.split_whitespace().min_by_key(|&word| word.len())
    }
    
    fn main() {
        let words = "This is a test string";
        println!("Shortest word in '{}': {:?}", words, shortest_word(words));
    }
    