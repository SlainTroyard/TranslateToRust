use std::io::{self, Read};

struct Solution;

impl Solution {
    fn count_partitions(nums: &Vec<i32>) -> i32 {
        // Calculate the sum of all elements in nums
        let sum: i32 = nums.iter().sum();
        
        // Return 0 if sum is odd, otherwise return size-1
        if sum % 2 != 0 {
            0
        } else {
            (nums.len() - 1) as i32
        }
    }
}

fn main() {
    // Read the entire input into a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    
    // Create an iterator over space-separated tokens
    let mut iter = input.split_whitespace();
    
    // Read n (number of elements)
    let n: usize = iter.next().expect("Missing input for n")
        .parse().expect("Failed to parse n");
    
    // Read the nums vector
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = iter.next().expect("Not enough integers in input")
            .parse().expect("Failed to parse integer");
        nums.push(num);
    }
    
    // Create solution and get result
    let sol = Solution;
    println!("{}", sol.count_partitions(&nums));
}