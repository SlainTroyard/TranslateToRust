use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn get_largest_outlier(nums: Vec<i32>) -> i32 {
        let mut ctr = HashMap::new();
        let mut sm = 0;

        // Count occurrences and calculate the total sum of all numbers
        for &num in &nums {
            *ctr.entry(num).or_insert(0) += 1;
            sm += num;
        }

        // Collect unique numbers into a vector
        let mut cands: Vec<i32> = ctr.keys().cloned().collect();
        
        // Sort candidates in descending order
        cands.sort_by(|a, b| b.cmp(a));

        // Iterate over candidates and find the largest valid outlier
        for n in cands {
            let d = (sm - n) / 2;
            let m = (sm - n) % 2;
            if m == 0 && ctr.contains_key(&d) && (d != n || ctr[&d] > 1) {
                return n;
            }
        }

        // Return -1 if no outlier is found
        -1
    }
}

fn main() {
    // Read input from the user
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read the elements of the array
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if nums.len() != n {
        eprintln!("Error: Expected {} numbers but got {}", n, nums.len());
        return;
    }

    // Call the method and output the result
    let result = Solution::get_largest_outlier(nums);
    println!("{}", result);
}