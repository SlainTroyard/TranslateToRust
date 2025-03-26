use std::io::{self, Read};

struct Solution;

impl Solution {
    fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let n = nums.len();
        let mut mini = i32::MAX;
        
        for i in 0..n {
            let mut currsum = 0;
            for j in i..n {
                currsum += nums[j];
                let length = (j - i + 1) as i32;
                // Check if the subarray length is within [l, r] and sum is positive
                if length >= l && length <= r && currsum > 0 {
                    mini = mini.min(currsum);
                }
            }
        }
        
        if mini == i32::MAX {
            -1
        } else {
            mini
        }
    }
}

fn main() {
    // Read all input at once and split into whitespace-separated tokens
    let input = {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input
    };
    let mut tokens = input.split_whitespace();
    
    // Parse the size of the array
    let n: usize = tokens.next().unwrap().parse().unwrap();
    // Parse the array elements
    let nums: Vec<i32> = (0..n)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();
    // Parse the range [l, r]
    let l: i32 = tokens.next().unwrap().parse().unwrap();
    let r: i32 = tokens.next().unwrap().parse().unwrap();
    
    // Compute and print the result
    let result = Solution::minimum_sum_subarray(nums, l, r);
    println!("{}", result);
}