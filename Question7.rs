/*
# Q. Implement a function that returns the kth smallest element in a given array.
Code:
*/
fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
        if k > arr.len() {
            return None;
        }
        let mut sorted_arr = arr.to_vec();
        sorted_arr.sort();
        Some(sorted_arr[k - 1])
    }
    
    fn main() {
        let arr = vec![1, 2, 3, 4, 5];
        let k = 3;
        println!("{}th smallest element in {:?}: {:?}", k, arr, kth_smallest(&arr, k));
    }
    