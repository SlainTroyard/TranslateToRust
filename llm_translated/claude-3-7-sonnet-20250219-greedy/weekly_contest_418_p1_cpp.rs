use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_good_number(nums: &mut Vec<i32>) -> i32 {
        // Sort the numbers based on a custom comparator
        nums.sort_by(|&a, &b| {
            // Calculate the length (number of bits) for each number
            let len_a = (a as f64).log2().floor() as i32 + 1;
            let len_b = (b as f64).log2().floor() as i32 + 1;
            
            // Simulate concatenation by shifting and OR-ing
            // Compare which concatenation produces a larger number
            let a_then_b = (a << len_b) | b;
            let b_then_a = (b << len_a) | a;
            
            // Sort in descending order
            b_then_a.cmp(&a_then_b)
        });

        // Concatenate all numbers to form the result
        let mut ans = 0;
        for &x in nums.iter() {
            let len = (x as f64).log2().floor() as i32 + 1;
            ans = (ans << len) | x;
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the array
    let num_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the array elements
    let nums_line = lines.next().unwrap().unwrap();
    let mut nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Ensure we have the correct number of elements
    nums.truncate(num_size);
    
    // Call the solution function and print the result
    let result = Solution::max_good_number(&mut nums);
    println!("{}", result);
}