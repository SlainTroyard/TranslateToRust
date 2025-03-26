use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn minimum_sum_subarray(nums: &Vec<i32>, l: i32, r: i32) -> i32 {
        let n = nums.len();
        let mut mini = i32::MAX;
        for i in 0..n {
            let mut currsum = 0;
            for j in i..n {
                currsum += nums[j];
                let length = (j - i + 1) as i32;
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input the size of the array
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Input the array elements
    let nums: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Input the range [l, r]
    let lr: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let l = lr[0];
    let r = lr[1];

    // Compute the minimum sum subarray
    let result = Solution::minimum_sum_subarray(&nums, l, r);

    // Output the result
    println!("{}", result);

    Ok(())
}