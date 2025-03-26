use std::io;

struct Solution;

impl Solution {
    fn find_maximum_score(nums: &[i32]) -> i64 {
        let mut ans: i64 = 0;
        let mut mx = 0;
        for i in 0..(nums.len() - 1) {
            mx = mx.max(nums[i]);
            ans += mx as i64;
        }
        ans
    }
}

fn main() {
    // Read number of elements
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums_size: usize = input.trim().parse().expect("Failed to parse nums_size");
    
    // Read the array elements
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    
    // Ensure we have the correct number of elements
    assert_eq!(nums.len(), nums_size, "Input array size doesn't match the specified size");
    
    // Compute and output the result
    let sol = Solution;
    println!("{}", Solution::find_maximum_score(&nums));
}