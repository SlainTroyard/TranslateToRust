use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn is_zero_array(nums: &Vec<i32>, q: &Vec<Vec<i32>>) -> bool {
        let mut v = vec![0; nums.len() + 1];
        
        // Apply the difference array technique for the queries
        for i in 0..q.len() {
            let start = q[i][0] as usize;
            let end = q[i][1] as usize + 1;
            v[start] += 1;
            v[end] -= 1;
        }
        
        // Calculate the prefix sum to get the actual effect at each position
        for i in 1..nums.len() {
            v[i] += v[i-1];
        }
        
        // Check if all elements in nums can be reduced to zero or below
        for i in 0..nums.len() {
            if nums[i] - v[i] > 0 {
                return false;
            }
        }
        
        true
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the nums array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the nums array
    let nums_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read the number of queries
    let m: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the queries
    let mut queries = Vec::with_capacity(m);
    for _ in 0..m {
        let query_line = lines.next().unwrap().unwrap();
        let query: Vec<i32> = query_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(query);
    }
    
    // Call the solution function and print the result
    if Solution::is_zero_array(&nums, &queries) {
        println!("true");
    } else {
        println!("false");
    }
}