use std::collections::{BTreeSet, HashMap};
use std::io;

fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
    let mut counts = HashMap::new();
    let mut L = BTreeSet::new();
    let mut R = BTreeSet::new();
    let mut sum_l = 0i64;
    let mut ans = vec![0i64; nums.len() - k as usize + 1];

    for r in 0..nums.len() {
        let in_val = nums[r];
        // Remove the current value from the sets using its current count
        let current_in_count = counts.get(&in_val).cloned().unwrap_or(0);
        if current_in_count > 0 {
            let p = (current_in_count, in_val);
            if L.contains(&p) {
                sum_l -= p.0 as i64 * p.1 as i64;
                L.remove(&p);
            } else if R.contains(&p) {
                R.remove(&p);
            }
        }
        // Increment the count and add the new value
        counts.insert(in_val, current_in_count + 1);
        let new_in_count = current_in_count + 1;
        if new_in_count > 0 {
            let p = (new_in_count, in_val);
            if !L.is_empty() && p > *L.first().unwrap() {
                sum_l += p.0 as i64 * p.1 as i64;
                L.insert(p);
            } else {
                R.insert(p);
            }
        }

        let l = r as i32 + 1 - k;
        if l >= 0 {
            // Balance the sets to ensure L has exactly x elements
            while !R.is_empty() && L.len() < x as usize {
                let p = R.last().cloned().unwrap();
                R.remove(&p);
                L.insert(p);
                sum_l += p.0 as i64 * p.1 as i64;
            }
            while L.len() > x as usize {
                let p = L.first().cloned().unwrap();
                L.remove(&p);
                R.insert(p);
                sum_l -= p.0 as i64 * p.1 as i64;
            }
            ans[l as usize] = sum_l;

            let out_val = nums[l as usize];
            // Remove the outgoing value using its current count
            let current_out_count = counts.get(&out_val).cloned().unwrap_or(0);
            if current_out_count > 0 {
                let p = (current_out_count, out_val);
                if L.contains(&p) {
                    sum_l -= p.0 as i64 * p.1 as i64;
                    L.remove(&p);
                } else if R.contains(&p) {
                    R.remove(&p);
                }
            }
            // Decrement the count and add the new value
            counts.insert(out_val, current_out_count - 1);
            let new_out_count = current_out_count - 1;
            if new_out_count > 0 {
                let p = (new_out_count, out_val);
                if !L.is_empty() && p > *L.first().unwrap() {
                    sum_l += p.0 as i64 * p.1 as i64;
                    L.insert(p);
                } else {
                    R.insert(p);
                }
            }
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let kx: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse k or x"))
        .collect();
    let k = kx[0];
    let x = kx[1];

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let nums_size: usize = input
        .trim()
        .parse()
        .expect("Failed to parse nums size");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse nums"))
        .collect();

    let res = find_x_sum(nums, k, x);

    for num in res {
        print!("{} ", num);
    }
    println!();
}