use std::cmp::max;
use std::io;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_frequency(nums: Vec<int>, k: int) -> int {
        let mut f0: int = 0;
        let mut f1: [int; 51] = [0; 51];
        let mut max_f1: int = 0;
        let mut f2: int = 0;

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
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let mut nk = n_str.split_whitespace();
    let n: usize = nk.next().unwrap().parse().unwrap();
    let k: int = nk.next().unwrap().parse().unwrap();

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<int> = nums_str
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let solution = Solution {};
    println!("{}", solution.max_frequency(nums, k));
}