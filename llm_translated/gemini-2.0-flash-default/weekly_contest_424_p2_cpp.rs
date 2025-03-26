use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn is_zero_array(nums: &mut Vec<i32>, q: &mut Vec<Vec<i32>>) -> bool {
        let mut v = vec![0; nums.len() + 1];
        for i in 0..q.len() {
            let start = q[i][0] as usize;
            let end = q[i][1] as usize;
            v[start] += 1;
            v[end + 1] -= 1;
        }
        for i in 1..nums.len() {
            v[i] += v[i - 1];
        }
        for i in 0..nums.len() {
            if nums[i] - v[i] > 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let sol = Solution {};

    // Read the size of the nums array
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: i32 = n_str.trim().parse().expect("Invalid input");

    // Read the nums array
    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Read the number of queries
    let mut m_str = String::new();
    io::stdin().read_line(&mut m_str).expect("Failed to read line");
    let m: i32 = m_str.trim().parse().expect("Invalid input");

    // Read the queries
    let mut queries: Vec<Vec<i32>> = Vec::new();
    for _ in 0..m {
        let mut query_str = String::new();
        io::stdin().read_line(&mut query_str).expect("Failed to read line");
        let query: Vec<i32> = query_str
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        queries.push(query);
    }

    // Call the solution function and print the result
    if sol.is_zero_array(&mut nums.clone(), &mut queries.clone()) {
        println!("true");
    } else {
        println!("false");
    }
}