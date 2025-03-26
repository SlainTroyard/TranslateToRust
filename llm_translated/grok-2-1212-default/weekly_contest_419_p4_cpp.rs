use std::collections::{BTreeSet, HashMap};
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let k: usize = lines.next().unwrap()?.parse().unwrap();
    let x: usize = lines.next().unwrap()?.parse().unwrap();
    let nums_size: usize = lines.next().unwrap()?.parse().unwrap();

    let nums: Vec<i32> = lines
        .take(nums_size)
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let solution = Solution {};
    let res = solution.find_x_sum(nums, k, x);

    for num in res {
        print!("{} ", num);
    }
    println!();

    Ok(())
}

struct Solution;

impl Solution {
    pub fn find_x_sum(&self, nums: Vec<i32>, k: usize, x: usize) -> Vec<i64> {
        let mut l = BTreeSet::new();
        let mut r = BTreeSet::new();
        let mut sum_l = 0;
        let mut cnt = HashMap::new();

        let add = |x: i32, l: &mut BTreeSet<(i32, i32)>, r: &mut BTreeSet<(i32, i32)>, sum_l: &mut i64, cnt: &HashMap<i32, i32>| {
            let p = (*cnt.get(&x).unwrap_or(&0), x);
            if p.0 == 0 {
                return;
            }
            if !l.is_empty() && p > *l.iter().next().unwrap() {
                *sum_l += i64::from(p.0) * i64::from(p.1);
                l.insert(p);
            } else {
                r.insert(p);
            }
        };

        let del = |x: i32, l: &mut BTreeSet<(i32, i32)>, r: &mut BTreeSet<(i32, i32)>, sum_l: &mut i64, cnt: &HashMap<i32, i32>| {
            let p = (*cnt.get(&x).unwrap_or(&0), x);
            if p.0 == 0 {
                return;
            }
            if l.remove(&p) {
                *sum_l -= i64::from(p.0) * i64::from(p.1);
            } else {
                r.remove(&p);
            }
        };

        let l2r = |l: &mut BTreeSet<(i32, i32)>, r: &mut BTreeSet<(i32, i32)>, sum_l: &mut i64| {
            let p = *l.iter().next().unwrap();
            *sum_l -= i64::from(p.0) * i64::from(p.1);
            l.remove(&p);
            r.insert(p);
        };

        let r2l = |l: &mut BTreeSet<(i32, i32)>, r: &mut BTreeSet<(i32, i32)>, sum_l: &mut i64| {
            let p = *r.iter().next_back().unwrap();
            *sum_l += i64::from(p.0) * i64::from(p.1);
            r.remove(&p);
            l.insert(p);
        };

        let mut ans = vec![0; nums.len() - k + 1];
        for r in 0..nums.len() {
            let in_val = nums[r];
            del(in_val, &mut l, &mut r, &mut sum_l, &cnt);
            *cnt.entry(in_val).or_insert(0) += 1;
            add(in_val, &mut l, &mut r, &mut sum_l, &cnt);

            let l = r + 1 - k;
            if l < 0 {
                continue;
            }

            while !r.is_empty() && l.len() < x {
                r2l(&mut l, &mut r, &mut sum_l);
            }
            while l.len() > x {
                l2r(&mut l, &mut r, &mut sum_l);
            }
            ans[l] = sum_l;

            let out_val = nums[l];
            del(out_val, &mut l, &mut r, &mut sum_l, &cnt);
            *cnt.entry(out_val).or_insert(0) -= 1;
            add(out_val, &mut l, &mut r, &mut sum_l, &cnt);
        }
        ans
    }
}