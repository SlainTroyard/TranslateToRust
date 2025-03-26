use std::collections::{BTreeSet, HashMap};
use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        // pii is represented as (count, value)
        let mut l: BTreeSet<(i32, i32)> = BTreeSet::new();
        let mut r: BTreeSet<(i32, i32)> = BTreeSet::new();
        let mut sum_l: i64 = 0;
        let mut cnt: HashMap<i32, i32> = HashMap::new();

        let mut add = |x: i32| {
            let p = (*cnt.get(&x).unwrap_or(&0), x);
            if p.0 == 0 {
                return;
            }
            if !l.is_empty() && p > *l.iter().next().unwrap() {
                sum_l += i64::from(p.0) * i64::from(p.1);
                l.insert(p);
            } else {
                r.insert(p);
            }
        };

        let mut del = |x: i32| {
            let p = (*cnt.get(&x).unwrap_or(&0), x);
            if p.0 == 0 {
                return;
            }
            if l.contains(&p) {
                sum_l -= i64::from(p.0) * i64::from(p.1);
                l.remove(&p);
            } else {
                r.remove(&p);
            }
        };

        let mut l2r = || {
            let p = *l.iter().next().unwrap();
            sum_l -= i64::from(p.0) * i64::from(p.1);
            l.remove(&p);
            r.insert(p);
        };

        let mut r2l = || {
            let p = *r.iter().next_back().unwrap();
            sum_l += i64::from(p.0) * i64::from(p.1);
            r.remove(&p);
            l.insert(p);
        };

        let mut ans: Vec<i64> = Vec::new();
        for _ in 0..(nums.len() as i32 - k + 1) {
            ans.push(0);
        }

        for r_idx in 0..nums.len() {
            let in = nums[r_idx];
            del(in);
            *cnt.entry(in).or_insert(0) += 1;
            add(in);

            let l_idx = r_idx as i32 + 1 - k;
            if l_idx < 0 {
                continue;
            }

            while !r.is_empty() && l.len() < x as usize {
                r2l();
            }
            while l.len() > x as usize {
                l2r();
            }
            ans[l_idx as usize] = sum_l;

            let out = nums[l_idx as usize];
            del(out);
            *cnt.entry(out).or_insert(0) -= 1;
            add(out);
        }
        return ans;
    }
}

fn main() {
    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).unwrap();
    let mut iter = k_str.trim().split_whitespace();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let x: i32 = iter.next().unwrap().parse().unwrap();

    let mut nums_size_str = String::new();
    io::stdin().read_line(&mut nums_size_str).unwrap();
    let nums_size: usize = nums_size_str.trim().parse().unwrap();

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).unwrap();
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let s = Solution {};
    let res = s.find_x_sum(nums, k, x);

    for i in 0..res.len() {
        print!("{} ", res[i]);
    }
    println!();
}