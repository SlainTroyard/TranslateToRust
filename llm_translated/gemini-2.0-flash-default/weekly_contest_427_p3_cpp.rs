use std::collections::HashMap;
use std::io;
use std::cmp::{max, min};

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    let n: i32 = n_str.trim().parse().unwrap();

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).unwrap();
    let k: i32 = k_str.trim().parse().unwrap();

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).unwrap();
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let solution = Solution {};
    let result = solution.max_subarray_sum(nums, k);
    println!("{}", result);
}

struct Solution {}

impl Solution {
    pub fn max_subarray_sum(&self, v: Vec<i32>, k: i32) -> i64 {
        let mut m: HashMap<i32, i64> = HashMap::new();
        let mut ans: i64 = i64::min_value();
        let mut sm: i64 = 0;
        for i in 0..v.len() {
            sm += v[i] as i64;
            let cur_sz = (i as i32) + 1;
            if cur_sz % k == 0 {
                ans = max(ans, sm);
            }
            let y = (cur_sz % k);
            if m.contains_key(&y) {
                ans = max(ans, sm - m.get(&y).unwrap());
                let current_min = *m.get(&y).unwrap();
                m.insert(y, min(current_min, sm));
            } else {
                m.insert(y, sm);
            }
        }
        ans
    }
}