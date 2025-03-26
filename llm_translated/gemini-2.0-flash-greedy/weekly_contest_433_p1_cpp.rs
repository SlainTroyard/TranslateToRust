use std::io;
use std::io::Read;
use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn subarray_sum(&self, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut s: Vec<i32> = vec![0; n + 1];
        let mut sum = 0;
        for i in 0..n {
            sum += nums[i];
            s[i + 1] = sum;
        }

        let mut ans = 0;
        for i in 0..n {
            ans += s[i + 1] - s[max(i as i32 - nums[i] as i32, 0) as usize];
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();

    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let solution = Solution {};
    println!("{}", solution.subarray_sum(nums));
}