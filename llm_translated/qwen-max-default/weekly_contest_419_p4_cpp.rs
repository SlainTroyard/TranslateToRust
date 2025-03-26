use std::collections::{BTreeSet, HashMap};
use std::io::{self, BufRead};

type Pii = (i32, i32);

struct Solution;

impl Solution {
    fn find_x_sum(nums: &Vec<i32>, k: usize, x: usize) -> Vec<i64> {
        let mut l: BTreeSet<Pii> = BTreeSet::new();
        let mut r: BTreeSet<Pii> = BTreeSet::new();
        let mut sum_l: i64 = 0;
        let mut cnt: HashMap<i32, i32> = HashMap::new();

        let add = |&x: &i32| {
            let p = (*cnt.entry(x).or_insert(0), x);
            if p.0 == 0 {
                return;
            }
            if !l.is_empty() && p > *l.iter().next().unwrap() {
                sum_l += (p.0 as i64) * (p.1 as i64);
                l.insert(p);
            } else {
                r.insert(p);
            }
        };

        let del = |&x: &i32| {
            let p = (*cnt.entry(x).or_insert(0), x);
            if p.0 == 0 {
                return;
            }
            if let Some(_) = l.take(&p) {
                sum_l -= (p.0 as i64) * (p.1 as i64);
            } else {
                r.remove(&p);
            }
        };

        let l2r = || {
            let p = *l.iter().next().unwrap();
            sum_l -= (p.0 as i64) * (p.1 as i64);
            l.remove(&p);
            r.insert(p);
        };

        let r2l = || {
            let p = *r.iter().rev().next().unwrap();
            sum_l += (p.0 as i64) * (p.1 as i64);
            r.remove(&p);
            l.insert(p);
        };

        let mut ans: Vec<i64> = vec![0; nums.len() - k + 1];
        for r in 0..nums.len() {
            let in_ = nums[r];
            del(&in_);
            *cnt.entry(in_).or_insert(0) += 1;
            add(&in_);

            let l = r + 1 - k;
            if l < 0 {
                continue;
            }

            while !r.is_empty() && l.len() < x {
                r2l();
            }
            while l.len() > x {
                l2r();
            }
            ans[l] = sum_l;

            let out = nums[l];
            del(&out);
            *cnt.get_mut(&out).unwrap() -= 1;
            add(&out);
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let line1: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let k = line1[0] as usize;
    let x = line1[1] as usize;

    let line2: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let nums_size = line2[0] as usize;

    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let res = Solution::find_x_sum(&nums, k, x);
    for &val in &res {
        print!("{} ", val);
    }
    println!();
}