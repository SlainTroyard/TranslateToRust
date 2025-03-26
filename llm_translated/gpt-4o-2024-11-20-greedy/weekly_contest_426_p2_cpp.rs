use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn get_largest_outlier(nums: Vec<i32>) -> i32 {
        let mut ctr = HashMap::new();
        let mut sm = 0;

        // Count occurrences and calculate the sum
        for &num in &nums {
            *ctr.entry(num).or_insert(0) += 1;
            sm += num;
        }

        // Collect unique numbers
        let mut cands: Vec<i32> = ctr.keys().cloned().collect();

        // Sort in descending order
        cands.sort_by(|a, b| b.cmp(a));

        // Check for the largest outlier
        for &n in &cands {
            let d = (sm - n) / 2;
            let m = (sm - n) % 2;
            if m == 0 && ctr.contains_key(&d) && (d != n || ctr[&d] > 1) {
                return n;
            }
        }

        -1
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements
    let n: usize = lines
        .next()
        .expect("Expected input for number of elements")
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse number of elements");

    // Read the elements of the array
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected input for array elements")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse array element"))
        .collect();

    assert_eq!(nums.len(), n, "Input array size does not match the specified size");

    // Call the method and output the result
    let result = Solution::get_largest_outlier(nums);
    println!("{}", result);
}