use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn find_maximum_score(nums: Vec<i32>) -> i64 {
        let mut ans: i64 = 0;
        let mut mx = 0;
        for i in 0..nums.len() - 1 {
            mx = mx.max(nums[i]);
            ans += mx as i64;
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the nums vector
    let nums_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the nums vector
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Ensure the size matches the number of elements read
    assert_eq!(nums.len(), nums_size);

    // Create an instance of Solution and compute the result
    let sol = Solution;
    let result = sol.find_maximum_score(nums);

    // Print the result
    println!("{}", result);
}