use std::io;
use std::vec;

struct Solution {}

impl Solution {
    fn subarray_sum(&self, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut s: Vec<i32> = vec![0; n + 1];
        // Calculate prefix sum
        for i in 0..n {
            s[i + 1] = s[i] + nums[i];
        }

        let mut ans = 0;
        for i in 0..n {
            let max_index = std::cmp::max(i as i32 - nums[i], 0) as usize;
            ans += s[i + 1] - s[max_index];
        }
        ans
    }
}

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Invalid input for n");

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input for nums"))
        .collect();

    let solution = Solution {};
    println!("{}", solution.subarray_sum(nums));
}