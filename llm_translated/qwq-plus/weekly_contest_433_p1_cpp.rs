use std::io;

pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut s = vec![0; n + 1];
        for i in 1..=n {
            s[i] = s[i - 1] + nums[i - 1];
        }

        let mut ans = 0;
        for i in 0..n {
            let start = (i as i32 - nums[i]).max(0) as usize;
            ans += s[i + 1] - s[start];
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let n: usize = tokens[0].parse().unwrap();
    let nums: Vec<i32> = tokens[1..1 + n]
        .iter()
        .map(|&s| s.parse().unwrap())
        .collect();

    let ans = Solution::subarray_sum(nums);
    println!("{}", ans);
}