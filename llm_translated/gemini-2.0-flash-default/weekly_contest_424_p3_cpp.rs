use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut d = vec![0; n + 1];

        d[0] = nums[0];
        for i in 1..n {
            d[i] = nums[i] - nums[i - 1];
        }

        let mut acc = 0;
        let mut cur = -1;
        let mut ans = 0;
        while (acc <= 0) && (cur < (n as i32 -1)) {
            cur += 1;
            acc += d[cur as usize];
        }
        if (cur as usize) == n {
            return 0;
        }

        let m = queries.len();
        for i in 0..m {
            d[(queries[i][1] + 1) as usize] += queries[i][2];
            d[queries[i][0] as usize] -= queries[i][2];
            if (cur >= queries[i][0]) && (cur <= queries[i][1]) {
                acc -= queries[i][2];
                while (acc <= 0) && (cur < (n as i32 -1) ) {
                    cur += 1;
                    acc += d[cur as usize];
                }
                if (cur as usize) == n {
                    return (i as i32) + 1;
                }
            }
        }
        return -1;
    }
}

fn main() {
    let sol = Solution {};

    // Read the size of the nums array
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    let n: i32 = n_str.trim().parse().unwrap();

    // Read the nums array
    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).unwrap();
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read the number of queries
    let mut m_str = String::new();
    io::stdin().read_line(&mut m_str).unwrap();
    let m: i32 = m_str.trim().parse().unwrap();

    // Read the queries
    let mut queries: Vec<Vec<i32>> = Vec::new();
    for _ in 0..m {
        let mut query_str = String::new();
        io::stdin().read_line(&mut query_str).unwrap();
        let query: Vec<i32> = query_str
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(query);
    }

    // Call the solution function and print the result
    println!("{}", sol.min_zero_array(nums, queries));
}