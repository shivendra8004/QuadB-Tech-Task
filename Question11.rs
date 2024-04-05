/*
# Q. Merge two sorted arrays in Rust.
Code:
*/
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
        let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
        let (mut i, mut j) = (0, 0);
        while i < arr1.len() && j < arr2.len() {
            if arr1[i] < arr2[j] {
                merged.push(arr1[i]);
                i += 1;
            } else {
                merged.push(arr2[j]);
                j += 1;
            }
        }
        merged.extend_from_slice(&arr1[i..]);
        merged.extend_from_slice(&arr2[j..]);
        merged
    }
    
    fn main() {
        let arr1 = vec![1, 3, 5, 7];
        let arr2 = vec![2, 4, 6, 8];
        println!("Merged sorted arrays: {:?}", merge_sorted_arrays(&arr1, &arr2));
    }
    