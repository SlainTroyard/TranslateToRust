use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<usize>>) -> bool {
        let mut v = vec![0; nums.len() + 1];
        
        // Apply range updates using the queries
        for query in queries {
            v[query[0]] += 1;
            if query[1] + 1 < v.len() {
                v[query[1] + 1] -= 1;
            }
        }
        
        // Compute prefix sums to apply the range updates
        for i in 1..nums.len() {
            v[i] += v[i - 1];
        }
        
        // Check if the resulting array satisfies the condition
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
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    // Read the number of queries
    let m: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the queries
    let mut queries = Vec::new();
    for _ in 0..m {
        let query: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        queries.push(query);
    }
    
    // Call the solution function and print the result
    let sol = Solution;
    if sol.is_zero_array(nums, queries) {
        println!("true");
    } else {
        println!("false");
    }
}