use std::collections::{HashMap, BTreeSet};
use std::io::{self, BufRead};

type Pair = (i32, i32);

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and x
    let kx_line = lines.next().unwrap().unwrap();
    let mut kx_iter = kx_line.split_whitespace();
    let k: usize = kx_iter.next().unwrap().parse().unwrap();
    let x: usize = kx_iter.next().unwrap().parse().unwrap();

    // Read numsSize
    let nums_size: usize = lines.next().unwrap().unwrap().parse().unwrap();

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
    let mut l = BTreeSet::new();
    let mut r = BTreeSet::new();
    let mut sum_l: i64 = 0;
    let mut cnt = HashMap::new();

    let mut ans = vec![0; nums.len() - k + 1];

    for r_idx in 0..nums.len() {
        let in_num = nums[r_idx];

        // Remove the old count of in_num from the sets
        let old_pair = (cnt.get(&in_num).copied().unwrap_or(0), in_num);
        if old_pair.0 != 0 {
            if l.contains(&old_pair) {
                sum_l -= old_pair.0 as i64 * old_pair.1 as i64;
                l.remove(&old_pair);
            } else {
                r.remove(&old_pair);
            }
        }

        // Update the count of in_num
        *cnt.entry(in_num).or_insert(0) += 1;

        // Add the new count of in_num to the sets
        let new_pair = (cnt[&in_num], in_num);
        if !l.is_empty() && new_pair > *l.iter().next().unwrap() {
            sum_l += new_pair.0 as i64 * new_pair.1 as i64;
            l.insert(new_pair);
        } else {
            r.insert(new_pair);
        }

        let l_idx = r_idx + 1 - k;
        if l_idx >= nums.len() {
            continue;
        }

        // Ensure L has exactly x elements
        while !r.is_empty() && l.len() < x {
            let max_r = *r.iter().next_back().unwrap();
            sum_l += max_r.0 as i64 * max_r.1 as i64;
            r.remove(&max_r);
            l.insert(max_r);
        }
        while l.len() > x {
            let min_l = *l.iter().next().unwrap();
            sum_l -= min_l.0 as i64 * min_l.1 as i64;
            l.remove(&min_l);
            r.insert(min_l);
        }

        ans[l_idx] = sum_l;

        let out_num = nums[l_idx];

        // Remove the old count of out_num from the sets
        let old_pair = (cnt.get(&out_num).copied().unwrap_or(0), out_num);
        if old_pair.0 != 0 {
            if l.contains(&old_pair) {
                sum_l -= old_pair.0 as i64 * old_pair.1 as i64;
                l.remove(&old_pair);
            } else {
                r.remove(&old_pair);
            }
        }

        // Update the count of out_num
        *cnt.entry(out_num).or_insert(0) -= 1;

        // Add the new count of out_num to the sets
        let new_pair = (cnt[&out_num], out_num);
        if new_pair.0 != 0 {
            if !l.is_empty() && new_pair > *l.iter().next().unwrap() {
                sum_l += new_pair.0 as i64 * new_pair.1 as i64;
                l.insert(new_pair);
            } else {
                r.insert(new_pair);
            }
        }
    }

    ans
}