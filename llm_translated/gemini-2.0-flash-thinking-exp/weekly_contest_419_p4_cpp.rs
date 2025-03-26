use std::collections::{BTreeSet, HashMap};
use std::io;

struct Solution {}

impl Solution {
    pub fn find_x_sum(&self, nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        type Pii = (i32, i32);
        let mut l: BTreeSet<Pii> = BTreeSet::new();
        let mut r: BTreeSet<Pii> = BTreeSet::new();
        let mut sum_l: i64 = 0;
        let mut cnt: HashMap<i32, i32> = HashMap::new();

        let mut add = |val: i32, l: &mut BTreeSet<Pii>, r: &mut BTreeSet<Pii>, sum_l: &mut i64, cnt: &mut HashMap<i32, i32>| {
            let p = (*cnt.get(&val).unwrap_or(&0), val);
            if p.0 == 0 {
                return;
            }
            if !l.is_empty() && p > *l.iter().next().unwrap() {
                *sum_l += p.0 as i64 * p.1 as i64;
                l.insert(p);
            } else {
                r.insert(p);
            }
        };

        let mut del = |val: i32, l: &mut BTreeSet<Pii>, r: &mut BTreeSet<Pii>, sum_l: &mut i64, cnt: &mut HashMap<i32, i32>| {
            let p = (*cnt.get(&val).unwrap_or(&0), val);
            if p.0 == 0 {
                return;
            }
            if l.contains(&p) {
                *sum_l -= p.0 as i64 * p.1 as i64;
                l.remove(&p);
            } else {
                r.remove(&p);
            }
        };

        let mut l2r = |l: &mut BTreeSet<Pii>, r: &mut BTreeSet<Pii>, sum_l: &mut i64| {
            if let Some(&p) = l.iter().next() {
                *sum_l -= p.0 as i64 * p.1 as i64;
                l.remove(&p);
                r.insert(p);
            }
        };

        let mut r2l = |l: &mut BTreeSet<Pii>, r: &mut BTreeSet<Pii>, sum_l: &mut i64| {
            if let Some(&p) = r.iter().next_back() {
                *sum_l += p.0 as i64 * p.1 as i64;
                r.remove(&p);
                l.insert(p);
            }
        };

        let mut ans: Vec<i64> = Vec::with_capacity(nums.len().saturating_sub(k as usize) + 1);
        for r_idx in 0..nums.len() {
            let in_val = nums[r_idx];
            del(in_val, &mut l, &mut r, &mut sum_l, &mut cnt);
            *cnt.entry(in_val).or_insert(0) += 1;
            add(in_val, &mut l, &mut r, &mut sum_l, &mut cnt);

            let l_idx = r_idx as i32 + 1 - k;
            if l_idx < 0 {
                continue;
            }

            while !r.is_empty() && l.len() < x as usize {
                r2l(&mut l, &mut r, &mut sum_l);
            }
            while l.len() > x as usize {
                l2r(&mut l, &mut r, &mut sum_l);
            }
            ans.push(sum_l);

            let out_val = nums[l_idx as usize];
            del(out_val, &mut l, &mut r, &mut sum_l, &mut cnt);
            *cnt.entry(out_val).and_modify(|count| *count -= 1);
            add(out_val, &mut l, &mut r, &mut sum_l, &mut cnt);
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.trim().split_whitespace();
    let k: i32 = iter.next().unwrap().parse().expect("Parse error");
    let x: i32 = iter.next().unwrap().parse().expect("Parse error");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums_size: usize = input.trim().parse().expect("Parse error");

    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums_str = input.trim().split_whitespace();
    for s in nums_str {
        nums.push(s.parse().expect("Parse error"));
    }

    let s = Solution {};
    let res = s.find_x_sum(nums, k, x);
    for val in res {
        print!("{} ", val);
    }
    println!();
}