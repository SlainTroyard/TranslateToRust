use std::collections::HashMap;
use std::cmp::{max, min};
use std::io::{self, BufRead};

// Struct representing the solution
struct Solution;

impl Solution {
    // Function to calculate the maximum subarray sum with constraints
    fn max_subarray_sum(v: Vec<i32>, k: usize) -> i64 {
        let mut m: HashMap<usize, i64> = HashMap::new();
        let mut ans = i64::MIN; // Equivalent to LLONG_MIN in C++
        let mut sm: i64 = 0;

        for (i, &val) in v.iter().enumerate() {
            sm += val as i64; // Accumulate the sum
            let cur_sz = i + 1;

            // If the current size is divisible by k, update ans
            if cur_sz % k == 0 {
                ans = max(ans, sm);
            }

            let y = cur_sz % k;

            // If y exists in the map, check difference and update map
            if let Some(prev_sm) = m.get(&y) {
                ans = max(ans, sm - prev_sm);
                m.insert(y, min(*prev_sm, sm));
            } else {
                // Otherwise, insert the current sum
                m.insert(y, sm);
            }
        }

        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock().lines();

    // Read the array size (n) and the value of k
    let first_line = reader
        .next()
        .expect("Failed to read input")
        .expect("Failed to unwrap input");
    let mut numbers = first_line
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("Failed to parse number"));
    let n = numbers.next().expect("Failed to get n");
    let k = numbers.next().expect("Failed to get k");

    // Read the array elements
    let second_line = if let Some(line) = reader.next() {
        line.expect("Failed to unwrap array input")
    } else {
        panic!("Array input missing");
    };
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("Failed to parse array element"))
        .collect();

    // Ensure the array length matches the input size
    assert!(nums.len() == n, "Array length mismatch");

    // Call the function and print the result
    let solution = Solution;
    let result = solution.max_subarray_sum(nums, k);
    println!("{}", result);
}