// Problem: Weekly Contest 425 Problem 1

use std::io;

struct Solution;

impl Solution {
    fn minimum_sum_subarray(nums: &[i32], l: i32, r: i32) -> i32 {
        let n = nums.len();
        let mut mini = i32::MAX;
        for i in 0..n {
            let mut currsum = 0;
            for j in i..n {
                currsum += nums[j];
                let length = j - i + 1;
                if length >= l as usize && length <= r as usize && currsum > 0 {
                    mini = mini.min(currsum);
                }
            }
        }
        if mini == i32::MAX { -1 } else { mini }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = (0..n)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();
    let l: i32 = tokens.next().unwrap().parse().unwrap();
    let r: i32 = tokens.next().unwrap().parse().unwrap();

    let result = Solution::minimum_sum_subarray(&nums, l, r);

    println!("{}", result);
}