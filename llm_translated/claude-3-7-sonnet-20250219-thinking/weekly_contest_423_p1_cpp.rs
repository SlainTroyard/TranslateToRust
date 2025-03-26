use std::io::{self, Read};

struct Solution;

impl Solution {
    fn has_increasing_subarrays(nums: &Vec<i32>, k: usize) -> bool {
        // Closure to check if a subarray starting at idx is strictly increasing
        let mono = |idx: usize| -> bool {
            for i in idx..(idx + k - 1) {
                if nums[i] >= nums[i + 1] {
                    return false;
                }
            }
            true
        };

        // Check for two non-overlapping increasing subarrays
        for idx in 0..=(nums.len() - 2 * k) {
            if mono(idx) && mono(idx + k) {
                return true;
            }
        }
        false
    }
}

fn main() -> io::Result<()> {
    // Read all input into a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    // Create an iterator over whitespace-separated tokens
    let mut tokens = input.split_whitespace();
    
    // Parse the size of the array
    let n: usize = tokens.next().unwrap().parse().expect("Invalid size");
    
    // Parse the array elements
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num = tokens.next().unwrap().parse::<i32>().expect("Invalid number");
        nums.push(num);
    }
    
    // Parse the subarray length
    let k: usize = tokens.next().unwrap().parse().expect("Invalid k");
    
    // Create a Solution object and call the function
    let sol = Solution;
    if sol.has_increasing_subarrays(&nums, k) {
        println!("true");
    } else {
        println!("false");
    }
    
    Ok(())
}