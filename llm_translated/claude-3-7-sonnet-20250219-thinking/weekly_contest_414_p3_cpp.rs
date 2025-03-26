use std::io::{self, Read};

struct Solution {}

impl Solution {
    fn find_maximum_score(nums: &Vec<i32>) -> i64 {
        let mut ans: i64 = 0;
        let mut mx: i32 = 0;
        // Iterate up to the second-to-last element
        for i in 0..nums.len() - 1 {
            // Update the maximum value seen so far
            mx = mx.max(nums[i]);
            // Add the current maximum to the answer
            ans += mx as i64;
        }
        ans
    }
}

fn main() {
    // Read all input into a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let mut iter = input.split_whitespace();
    
    // Read the array size
    let nums_size: usize = iter.next().unwrap().parse().unwrap();
    
    // Read the array elements
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let num: i32 = iter.next().unwrap().parse().unwrap();
        nums.push(num);
    }
    
    // Compute and print the result
    println!("{}", Solution::find_maximum_score(&nums));
}