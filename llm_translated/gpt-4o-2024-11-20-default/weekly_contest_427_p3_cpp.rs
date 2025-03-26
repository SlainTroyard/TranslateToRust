use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    /// Calculates the maximum subarray sum for subarrays of size multiples of `k`.
    pub fn max_subarray_sum(v: Vec<i32>, k: usize) -> i64 {
        let mut m: HashMap<usize, i64> = HashMap::new();
        let mut ans = i64::MIN;
        let mut sm: i64 = 0;

        for (i, &val) in v.iter().enumerate() {
            sm += val as i64;
            let cur_sz = i + 1;

            // Check if the current subarray size is a multiple of k
            if cur_sz % k == 0 {
                ans = ans.max(sm);
            }

            let y = cur_sz % k;

            if let Some(&prev_sm) = m.get(&y) {
                // Update the answer if a better result is found
                ans = ans.max(sm - prev_sm);
                // Store the smallest prefix sum for the current modulus
                m.insert(y, prev_sm.min(sm));
            } else {
                // If modulus y is not already in the map, insert current sum
                m.insert(y, sm);
            }
        }

        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read array size (n) and k value
    let mut first_line = lines.next().unwrap().unwrap();
    let n_k: Vec<usize> = first_line
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let n = n_k[0];
    let k = n_k[1];

    // Read the array as a vector of integers
    let mut second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    assert!(nums.len() == n, "Input array size does not match the specified size!");

    // Call the function and print the result
    let result = Solution::max_subarray_sum(nums, k);
    println!("{}", result);
}