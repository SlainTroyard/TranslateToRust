use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_good_number(nums: &mut Vec<i32>) -> i32 {
        // Sort numbers based on which concatenation produces the larger number
        nums.sort_by(|&a, &b| {
            let len_a = (a as f64).log2().floor() as i32 + 1;
            let len_b = (b as f64).log2().floor() as i32 + 1;
            
            // Compare a|b vs b|a (where | is concatenation)
            // We shift a by len_b bits and add b to it, then compare with b shifted by len_a plus a
            let a_then_b = (a << len_b) | b;
            let b_then_a = (b << len_a) | a;
            
            // Sort in descending order
            b_then_a.cmp(&a_then_b)
        });

        // Concatenate all numbers in the sorted order
        let mut ans = 0;
        for x in nums.iter() {
            let len = (*x as f64).log2().floor() as i32 + 1;
            ans = (ans << len) | x;
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the array
    let num_size: usize = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input")
        .trim()
        .parse()
        .expect("Failed to parse number size");
    
    // Read the array elements
    let mut nums: Vec<i32> = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input")
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    
    // Ensure we have the right number of elements
    nums.truncate(num_size);
    
    // Calculate and print the result
    let result = Solution::max_good_number(&mut nums);
    println!("{}", result);
}