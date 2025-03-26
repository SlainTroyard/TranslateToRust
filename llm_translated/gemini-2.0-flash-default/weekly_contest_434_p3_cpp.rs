use std::cmp::max;
use std::io;

struct Solution {}

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut f0 = 0;
        let mut f1 = [0; 51];
        let mut max_f1 = 0;
        let mut f2 = 0;

        for &x in &nums {
            f2 = max(f2, max_f1) + if x == k { 1 } else { 0 };
            f1[x as usize] = max(f1[x as usize], f0) + 1;
            f0 += if x == k { 1 } else { 0 };
            max_f1 = max(max_f1, f1[x as usize]);
        }

        max(max_f1, f2)
    }
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let mut nk_iter = n.trim().split_whitespace();
    let n: i32 = nk_iter.next().unwrap().parse().unwrap();
    let k: i32 = nk_iter.next().unwrap().parse().unwrap();

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).unwrap();
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let solution = Solution {};
    println!("{}", solution.max_frequency(nums, k));
}