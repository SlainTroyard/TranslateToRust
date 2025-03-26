use std::io;

struct Solution;

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut prev = 0;
        let mut curr = 1;
        let mut ans = 0;

        for i in 1..nums.len() {
            if nums[i - 1] < nums[i] {
                curr += 1;
            } else {
                prev = curr;
                curr = 1;
            }
            ans = ans.max((curr / 2).max(prev.min(curr)));
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");

    let mut iter = input.split_whitespace();
    let n_str = iter.next().expect("No value for n");
    let n = n_str.parse::<i32>().expect("n must be an integer");

    let nums: Vec<_> = iter
        .map(|s| s.parse().expect("Invalid number"))
        .take(n as usize)
        .collect();

    let result = Solution::max_increasing_subarrays(nums);
    println!("{}", result);
}