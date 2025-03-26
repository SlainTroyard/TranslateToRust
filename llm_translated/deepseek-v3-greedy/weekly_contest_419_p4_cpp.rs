use std::collections::{HashMap, BTreeSet};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and x
    let kx_line = lines.next().unwrap().unwrap();
    let mut kx_iter = kx_line.split_whitespace();
    let k: usize = kx_iter.next().unwrap().parse().unwrap();
    let x: usize = kx_iter.next().unwrap().parse().unwrap();

    // Read numsSize
    let nums_size_line = lines.next().unwrap().unwrap();
    let nums_size: usize = nums_size_line.parse().unwrap();

    // Read nums
    let nums_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Compute the result
    let res = find_x_sum(nums, k, x);

    // Print the result
    for val in res {
        print!("{} ", val);
    }
    println!();
}

fn find_x_sum(nums: Vec<i32>, k: usize, x: usize) -> Vec<i64> {
    type Pair = (i32, i32);
    let mut l = BTreeSet::new();
    let mut r = BTreeSet::new();
    let mut sum_l: i64 = 0;
    let mut cnt = HashMap::new();

    let mut add = |x: i32| {
        let p = (cnt.get(&x).copied().unwrap_or(0), x);
        if p.0 == 0 {
            return;
        }
        if !l.is_empty() && p > *l.iter().next().unwrap() {
            sum_l += p.0 as i64 * p.1 as i64;
            l.insert(p);
        } else {
            r.insert(p);
        }
    };

    let mut del = |x: i32| {
        let p = (cnt.get(&x).copied().unwrap_or(0), x);
        if p.0 == 0 {
            return;
        }
        if l.contains(&p) {
            sum_l -= p.0 as i64 * p.1 as i64;
            l.remove(&p);
        } else {
            r.remove(&p);
        }
    };

    let mut l2r = || {
        let p = *l.iter().next().unwrap();
        sum_l -= p.0 as i64 * p.1 as i64;
        l.remove(&p);
        r.insert(p);
    };

    let mut r2l = || {
        let p = *r.iter().next_back().unwrap();
        sum_l += p.0 as i64 * p.1 as i64;
        r.remove(&p);
        l.insert(p);
    };

    let mut ans = vec![0; nums.len() - k + 1];
    for r in 0..nums.len() {
        let in_num = nums[r];
        del(in_num);
        *cnt.entry(in_num).or_insert(0) += 1;
        add(in_num);

        let l_idx = r + 1 - k;
        if l_idx < 0 {
            continue;
        }

        while !r.is_empty() && l.len() < x {
            r2l();
        }
        while l.len() > x {
            l2r();
        }
        ans[l_idx] = sum_l;

        let out_num = nums[l_idx];
        del(out_num);
        *cnt.entry(out_num).or_insert(0) -= 1;
        add(out_num);
    }
    ans
}