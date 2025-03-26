use std::io::{self, Read};

struct Solution;

impl Solution {
    pub fn max_frequency(nums: &[i32], k: i32) -> i32 {
        let mut f0 = 0;
        let mut f1 = [0; 51]; // Track frequencies for values 0-50
        let mut max_f1 = 0;
        let mut f2 = 0;
        
        for &x in nums {
            // Update f2 using previous max_f1 and check current element
            f2 = i32::max(f2, max_f1) + (x == k) as i32;
            let idx = x as usize; // Assumes x is within 0-50 based on problem constraints
            
            // Update frequency for current value x
            let new_f1_x = f1[idx].max(f0) + 1;
            f1[idx] = new_f1_x;
            
            // Update f0 if current element matches k
            f0 += (x == k) as i32;
            
            // Maintain maximum frequency in f1 array
            max_f1 = max_f1.max(new_f1_x);
        }
        
        // Return the maximum of the two frequency tracking mechanisms
        max_f1.max(f2)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    
    // Parse n and k from first two tokens
    let n: i32 = tokens.next().unwrap().parse().unwrap();
    let k: i32 = tokens.next().unwrap().parse().unwrap();
    
    // Collect exactly n numbers from remaining tokens
    let nums: Vec<i32> = tokens
        .take(n as usize)
        .map(|s| s.parse().unwrap())
        .collect();
    
    let solution = Solution;
    println!("{}", solution.max_frequency(&nums, k));
}