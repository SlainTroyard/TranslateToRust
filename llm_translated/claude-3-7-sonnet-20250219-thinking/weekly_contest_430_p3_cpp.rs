use std::collections::HashMap;
use std::io::{self, Read};

struct Solution;

impl Solution {
    fn number_of_subsequences(nums: &[i32]) -> i64 {
        let n = nums.len();
        let mut suf = HashMap::new();
        
        // Build suffix map
        for i in 4..n-2 {
            let c = nums[i];
            for j in i+2..n {
                let d = nums[j];
                let g = Self::gcd(c, d);
                let key = ((d / g) << 16) | (c / g);
                *suf.entry(key).or_insert(0) += 1;
            }
        }
        
        let mut ans: i64 = 0;
        for i in 2..n-4 {
            let b = nums[i];
            // Match prefix pairs with suffix pairs
            for j in 0..i-1 {
                let a = nums[j];
                let g = Self::gcd(a, b);
                let key = ((a / g) << 16) | (b / g);
                ans += *suf.get(&key).unwrap_or(&0) as i64;
            }
            
            // Remove pairs from suffix map
            let c = nums[i+2];
            for j in i+4..n {
                let d = nums[j];
                let g = Self::gcd(c, d);
                let key = ((d / g) << 16) | (c / g);
                *suf.entry(key).or_insert(0) -= 1;
            }
        }
        
        ans
    }
    
    // Euclidean algorithm to find Greatest Common Divisor
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for n");
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();
    
    // Call the solution function
    let result = Solution::number_of_subsequences(&nums);
    
    // Output the result
    println!("{}", result);
}