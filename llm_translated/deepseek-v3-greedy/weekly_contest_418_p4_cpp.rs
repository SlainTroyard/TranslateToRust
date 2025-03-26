use std::io::{self, BufRead};
use std::cmp::max;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements in nums
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the nums vector
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read the number of queries
    let q: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the queries vector
    let queries: Vec<i64> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Compute the answers
    let ans = gcd_values(&nums, &queries);

    // Print the answers
    for x in ans {
        print!("{} ", x);
    }
    println!();
}

fn gcd_values(nums: &[i32], queries: &[i64]) -> Vec<i32> {
    let mx = *nums.iter().max().unwrap() as usize;
    let mut cnt_x = vec![0; mx + 1];

    // Count the occurrences of each number in nums
    for &x in nums {
        cnt_x[x as usize] += 1;
    }

    let mut cnt_gcd = vec![0i64; mx + 1];

    // Calculate the number of pairs with GCD equal to i
    for i in (1..=mx).rev() {
        let mut c = 0;
        for j in (i..=mx).step_by(i) {
            c += cnt_x[j];
            cnt_gcd[i] -= cnt_gcd[j];
        }
        cnt_gcd[i] += (c * (c - 1) / 2) as i64;
    }

    // Compute the prefix sum of cnt_gcd
    for i in 1..=mx {
        cnt_gcd[i] += cnt_gcd[i - 1];
    }

    // For each query, find the largest i such that cnt_gcd[i] <= query
    queries.iter()
        .map(|&query| {
            match cnt_gcd.binary_search(&query) {
                Ok(idx) => idx as i32 + 1,
                Err(idx) => idx as i32,
            }
        })
        .collect()
}