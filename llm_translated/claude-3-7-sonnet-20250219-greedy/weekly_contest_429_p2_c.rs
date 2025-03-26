use std::io::{self, BufRead};
use std::cmp::{min, max};

/// Finds the maximum number of distinct elements that can be created
/// by modifying array elements within the given difference constraint.
fn max_distinct_elements(arr: &mut [i32], diff: i32) -> i32 {
    let mut prev = i32::MIN;
    let mut distinct_count = 0;

    // Sort the array in ascending order
    arr.sort();

    for &num in arr.iter() {
        // Calculate the next distinct value
        // Take the maximum of (prev + 1) and (num - diff)
        let x = max(prev + 1, num - diff);

        // If the value is within the allowed range
        if x <= num + diff {
            distinct_count += 1;
            prev = x;
        }
    }

    distinct_count
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read array size
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read difference value
    let diff: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read array elements
    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        arr.push(num);
    }
    
    // Compute the result
    let result = max_distinct_elements(&mut arr, diff);
    
    // Output the result
    println!("{}", result);
}