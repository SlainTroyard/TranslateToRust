use std::io;

struct Solution;

impl Solution {
    fn is_zero_array(nums: &Vec<i32>, q: &Vec<Vec<i32>>) -> bool {
        let mut v = vec![0; nums.len() + 1];
        
        // Apply each query's effect on the difference array
        for i in 0..q.len() {
            let start = q[i][0] as usize;
            let end = (q[i][1] + 1) as usize;
            v[start] += 1;
            if end < v.len() {
                v[end] -= 1;
            }
        }
        
        // Calculate the prefix sum
        for i in 1..nums.len() {
            v[i] += v[i - 1];
        }
        
        // Check if any element in nums is greater than the corresponding element in v
        for i in 0..nums.len() {
            if nums[i] - v[i] > 0 {
                return false;
            }
        }
        
        true
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    
    // Read the size of the nums array
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse().expect("Invalid input for n");
    
    // Read the nums array
    input.clear();
    io::stdin().read_line(&mut input)?;
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input for nums"))
        .collect();
    
    if nums.len() != n {
        panic!("Expected {} numbers in nums array, got {}", n, nums.len());
    }
    
    // Read the number of queries
    input.clear();
    io::stdin().read_line(&mut input)?;
    let m: usize = input.trim().parse().expect("Invalid input for m");
    
    // Read the queries
    let mut queries = Vec::with_capacity(m);
    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let query: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input for query"))
            .collect();
        
        if query.len() != 2 {
            panic!("Expected 2 numbers per query, got {}", query.len());
        }
        
        queries.push(query);
    }
    
    // Call the solution function and print the result
    if Solution::is_zero_array(&nums, &queries) {
        println!("true");
    } else {
        println!("false");
    }
    
    Ok(())
}