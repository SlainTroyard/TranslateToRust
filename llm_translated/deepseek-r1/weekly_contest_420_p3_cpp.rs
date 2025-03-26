use lazy_static::lazy_static;
use std::io::{self, Read};

lazy_static! {
    // Initialize the least prime factor (LPF) array using sieve of Eratosthenes
    static ref LPF: Vec<usize> = {
        const MX: usize = 1_000_001;
        let mut lpf = vec![0; MX];
        
        for i in 2..MX {
            // If i is prime (LPF not yet set)
            if lpf[i] == 0 {
                // Mark all multiples of i with LPF as i if not already marked
                for j in (i..MX).step_by(i) {
                    if lpf[j] == 0 {
                        lpf[j] = i;
                }
            }
        }
        lpf
    };
}

struct Solution;

impl Solution {
    pub fn min_operations(nums: &mut [i32]) -> i32 {
        let mut ans = 0;
        // Iterate from second last element to first
        for i in (0..nums.len().saturating_sub(1)).rev() {
            if nums[i] > nums[i + 1] {
                // Replace current number with its smallest prime factor
                let current = nums[i] as usize;
                nums[i] = LPF[current] as i32;
                ans += 1;
                
                // If still larger than next element, return -1
                if nums[i] > nums[i + 1] {
                    return -1;
                }
            }
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    
    // Read input size and numbers
    let n: usize = tokens.next().map(|s| s.parse().unwrap()).unwrap_or(0);
    let mut nums: Vec<i32> = tokens
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Compute and print result
    let result = Solution::min_operations(&mut nums);
    println!("{}", result);
}