use std::collections::{BTreeSet, HashMap};
use std::io::{self, BufRead};

type Pii = (i32, i32);

struct Solution;

impl Solution {
    pub fn find_x_sum(nums: &Vec<i32>, k: usize, x: usize) -> Vec<i64> {
        let mut l: BTreeSet<Pii> = BTreeSet::new();
        let mut r: BTreeSet<Pii> = BTreeSet::new();
        let mut sum_l: i64 = 0;
        let mut cnt: HashMap<i32, i32> = HashMap::new();

        let add = |num: i32| {
            let p = (*cnt.entry(num).or_insert(0), num);
            if p.0 == 0 {
                return;
            }
            if l.is_empty() || p > *l.iter().next().unwrap() {
                sum_l += (p.0 as i64) * (p.1 as i64);
                l.insert(p);
            } else {
                r.insert(p);
            }
        };

        let del = |num: i32| {
            let p = (*cnt.entry(num).or_insert(0), num);
            if p.0 == 0 {
                return;
            }
            if let Some(&p) = l.iter().find(|&q| q == &p) {
                sum_l -= (p.0 as i64) * (p.1 as i64);
                l.remove(&p);
            } else {
                r.remove(&p);
            }
        };

        let l2r = || {
            if let Some(p) = l.iter().next().cloned() {
                sum_l -= (p.0 as i64) * (p.1 as i64);
                l.remove(&p);
                r.insert(p);
            }
        };

        let r2l = || {
            if let Some(p) = r.iter().rev().next().cloned() {
                sum_l += (p.0 as i64) * (p.1 as i64);
                r.remove(&p);
                l.insert(p);
            }
        };

        let mut ans: Vec<i64> = vec![0; nums.len() - k + 1];
        for r in 0..nums.len() {
            let in_num = nums[r];
            del(in_num);
            *cnt.entry(in_num).or_insert(0) += 1;
            add(in_num);

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

            let out_num = nums[l];
            del(out_num);
            *cnt.entry(out_num).or_insert(0) -= 1;
            add(out_num);
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let line1 = lines.next().unwrap().unwrap();
    let line2 = lines.next().unwrap().unwrap();

    let mut iter = line1.split_whitespace();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap();

    let nums_size: usize = line2.parse().unwrap();
    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        nums.push(lines.next().unwrap().unwrap().parse().unwrap());
    }

    let s = Solution;
    let res = s.find_x_sum(&nums, k, x);

    for num in res {
        print!("{} ", num);
    }
    println!();
}