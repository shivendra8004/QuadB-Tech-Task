/*
# Q. Implement a function that checks whether a given string is a palindrome or not.
Code:
*/
fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
        arr.iter().position(|&x| x == target)
    }
    
    fn main() {
        let sorted_arr = vec![1, 2, 3, 4, 5];
        let target = 2;
        println!("Index of first occurrence of {}: {:?}", target, first_occurrence(&sorted_arr, target));
    }
    