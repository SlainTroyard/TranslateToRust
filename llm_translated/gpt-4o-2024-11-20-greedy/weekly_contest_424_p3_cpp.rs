use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut d = vec![0; n + 1];

        // Compute the difference array
        d[0] = nums[0];
        for i in 1..n {
            d[i] = nums[i] - nums[i - 1];
        }

        let mut acc = 0;
        let mut cur = -1;
        let mut ans = 0;

        // Find the first position where the prefix sum becomes positive
        while acc <= 0 && cur < n as i32 {
            cur += 1;
            acc += d[cur as usize];
        }
        if cur == n as i32 {
            return 0;
        }

        let m = queries.len();
        for i in 0..m {
            let l = queries[i][0] as usize;
            let r = queries[i][1] as usize;
            let x = queries[i][2];

            d[r + 1] += x;
            d[l] -= x;

            if cur >= l as i32 && cur <= r as i32 {
                acc -= x;
                while acc <= 0 && cur < n as i32 {
                    cur += 1;
                    acc += d[cur as usize];
                }
                if cur == n as i32 {
                    return (i + 1) as i32;
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
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse n");

    // Read the nums array
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse nums"))
        .collect();

    // Read the number of queries
    let m: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse m");

    // Read the queries
    let mut queries = Vec::new();
    for _ in 0..m {
        let query: Vec<i32> = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse query"))
            .collect();
        queries.push(query);
    }

    // Call the solution function and print the result
    let result = Solution::min_zero_array(nums, queries);
    println!("{}", result);
}