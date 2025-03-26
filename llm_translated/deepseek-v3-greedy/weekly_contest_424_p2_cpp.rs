use std::io::{self, BufRead};

fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
    let mut v = vec![0; nums.len() + 1];
    
    // Apply the queries to the difference array
    for q in queries {
        v[q[0] as usize] += 1;
        if q[1] + 1 < v.len() as i32 {
            v[(q[1] + 1) as usize] -= 1;
        }
    }
    
    // Compute the prefix sum to get the actual counts
    for i in 1..nums.len() {
        v[i] += v[i - 1];
    }
    
    // Check if all elements in nums can be reduced to zero
    for i in 0..nums.len() {
        if nums[i] - v[i] > 0 {
            return false;
        }
    }
    
    true
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the nums array
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Read the nums array
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        nums.push(num);
    }
    
    // Read the number of queries
    let m: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Read the queries
    let mut queries = Vec::with_capacity(m);
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        queries.push(parts);
    }
    
    // Call the solution function and print the result
    if is_zero_array(nums, queries) {
        println!("true");
    } else {
        println!("false");
    }
}