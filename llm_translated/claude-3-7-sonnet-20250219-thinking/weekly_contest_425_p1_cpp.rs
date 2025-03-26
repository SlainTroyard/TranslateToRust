use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn minimum_sum_subarray(nums: &Vec<i32>, l: usize, r: usize) -> i32 {
        let n = nums.len();
        let mut mini = i32::MAX;
        
        for i in 0..n {
            let mut curr_sum = 0;
            for j in i..n {
                curr_sum += nums[j];
                let length = j - i + 1;
                if length >= l && length <= r && curr_sum > 0 {
                    mini = mini.min(curr_sum);
                }
            }
        }
        
        if mini == i32::MAX { -1 } else { mini }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    
    // Input the size of the array
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().expect("Failed to parse array size");
    
    // Input the array elements
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let mut num_input = String::new();
        stdin.read_line(&mut num_input).unwrap();
        let num: i32 = num_input.trim().parse().expect("Failed to parse array element");
        nums.push(num);
    }
    
    // Input the range [l, r]
    let mut l_input = String::new();
    stdin.read_line(&mut l_input).unwrap();
    let l: usize = l_input.trim().parse().expect("Failed to parse l");
    
    let mut r_input = String::new();
    stdin.read_line(&mut r_input).unwrap();
    let r: usize = r_input.trim().parse().expect("Failed to parse r");
    
    // Compute the minimum sum subarray
    let result = Solution::minimum_sum_subarray(&nums, l, r);
    
    // Output the result
    println!("{}", result);
}