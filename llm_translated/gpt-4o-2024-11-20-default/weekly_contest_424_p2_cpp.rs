use std::io;

struct Solution;

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<usize>>) -> bool {
        let mut v = vec![0; nums.len() + 1];
        
        // Apply range updates using the difference array technique
        for query in queries.iter() {
            v[query[0]] += 1;
            v[query[1] + 1] -= 1;
        }
        
        // Compute prefix sums to transform the difference array
        for i in 1..nums.len() {
            v[i] += v[i - 1];
        }
        
        // Check if nums[i] - v[i] > 0 for any i
        for i in 0..nums.len() {
            if nums[i] - v[i] > 0 {
                return false;
            }
        }
        
        true
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let mut lines = input.lines();
    
    // Parse the size of nums array
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    
    // Parse the nums array
    let nums: Vec<i32> = lines.next().unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    // Parse the number of queries
    let m: usize = lines.next().unwrap().trim().parse().unwrap();
    
    // Parse the queries
    let mut queries: Vec<Vec<usize>> = Vec::with_capacity(m);
    for _ in 0..m {
        let query: Vec<usize> = lines.next().unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        queries.push(query);
    }
    
    // Create an instance of solution and compute the result
    let sol = Solution;
    let result = sol.is_zero_array(nums, queries);
    
    // Print the result
    println!("{}", if result { "true" } else { "false" });
}