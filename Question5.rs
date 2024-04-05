/*
# Q. Given a sorted array of integers, implement a function that returns the median of the array.
Code:
*/
fn median(arr: &[i32]) -> f64 {
        let len = arr.len();
        if len % 2 == 0 {
            let mid = len / 2;
            (arr[mid - 1] + arr[mid]) as f64 / 2.0
        } else {
            arr[len / 2] as f64
        }
    }
    
    fn main() {
        let arr = vec![1, 2, 3, 4, 5];
        println!("Median of {:?}: {}", arr, median(&arr));
    }
    