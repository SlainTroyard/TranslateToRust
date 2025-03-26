// Problem: Weekly Contest 424 Problem 3
use std::io::{self, BufRead};
use std::collections::VecDeque;

struct Solution;

impl Solution {
    fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut d = vec![0; n + 1];

        // Compute the difference array d
        d[0] = nums[0];
        for i in 1..n {
            d[i] = nums[i] - nums[i - 1];
        }

        let mut acc = 0;
        let mut cur = -1;
        let mut ans = 0;

        // Find the initial position `cur` where the sum becomes positive in `d`
        while acc <= 0 && (cur as usize) < n {
            cur += 1;
            acc += d[cur as usize];
        }

        // If no positive prefix sum exists, return 0
        if cur as usize == n {
            return 0;
        }

        let m = queries.len();
        for (i, query) in queries.iter().enumerate() {
            let l = query[0] as usize;
            let r = query[1] as usize;
            let v = query[2];

            // Update the difference array based on the query
            d[r + 1] += v;
            d[l] -= v;

            if cur >= l as i32 && cur <= r as i32 {
                acc -= v;
                while acc <= 0 && (cur as usize) < n {
                    cur += 1;
                    acc += d[cur as usize];
                }
                if cur as usize == n {
                    return i as i32 + 1;
                }
            }
        }

        -1
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the nums array
    let n: usize = lines
        .next()
        .expect("Expected input")
        .unwrap()
        .trim()
        .parse()
        .expect("Expected integer");

    // Read the nums array
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected input")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Expected integer"))
        .collect();

    // Read the number of queries
    let m: usize = lines
        .next()
        .expect("Expected input")
        .unwrap()
        .trim()
        .parse()
        .expect("Expected integer");

    // Read the queries
    let mut queries = Vec::new();
    for _ in 0..m {
        let query: Vec<i32> = lines
            .next()
            .expect("Expected input")
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Expected integer"))
            .collect();
        queries.push(query);
    }

    let solution = Solution;
    let result = solution.min_zero_array(nums, queries);

    // Print the result
    println!("{}", result);
}