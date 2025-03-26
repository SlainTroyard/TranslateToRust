use std::collections::{BTreeSet, HashMap};
use std::io;

struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        let mut l: BTreeSet<(i32, i32)> = BTreeSet::new();
        let mut r: BTreeSet<(i32, i32)> = BTreeSet::new();
        let mut sum_l: i64 = 0;
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        
        // Add a value to either L or R set
        let mut add = |val: i32| {
            let count = *cnt.get(&val).unwrap_or(&0);
            let p = (count, val);
            if count == 0 {
                return;
            }
            if !l.is_empty() && p > *l.iter().next().unwrap() {
                sum_l += (p.0 as i64) * (p.1 as i64);
                l.insert(p);
            } else {
                r.insert(p);
            }
        };
        
        // Delete a value from either L or R set
        let mut del = |val: i32| {
            let count = *cnt.get(&val).unwrap_or(&0);
            let p = (count, val);
            if count == 0 {
                return;
            }
            if l.contains(&p) {
                sum_l -= (p.0 as i64) * (p.1 as i64);
                l.remove(&p);
            } else {
                r.remove(&p);
            }
        };
        
        // Move smallest element from L to R
        let mut l2r = || {
            if let Some(&p) = l.iter().next() {
                sum_l -= (p.0 as i64) * (p.1 as i64);
                l.remove(&p);
                r.insert(p);
            }
        };
        
        // Move largest element from R to L
        let mut r2l = || {
            if let Some(&p) = r.iter().rev().next() {
                sum_l += (p.0 as i64) * (p.1 as i64);
                r.remove(&p);
                l.insert(p);
            }
        };
        
        let mut ans = vec![0; nums.len() - k as usize + 1];
        for r_idx in 0..nums.len() {
            let in_val = nums[r_idx];
            del(in_val);
            *cnt.entry(in_val).or_insert(0) += 1;
            add(in_val);
            
            let l_idx = r_idx + 1 - k as usize;
            if l_idx >= nums.len() || r_idx + 1 < k as usize {
                continue;
            }
            
            while !r.is_empty() && l.len() < x as usize {
                r2l();
            }
            while l.len() > x as usize {
                l2r();
            }
            ans[l_idx] = sum_l;
            
            let out_val = nums[l_idx];
            del(out_val);
            *cnt.entry(out_val).or_insert(0) -= 1;
            add(out_val);
        }
        
        ans
    }
}

fn main() {
    // Read k and x
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid input")).collect();
    let k = parts[0];
    let x = parts[1];
    
    // Read nums size
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums_size: usize = input.trim().parse().expect("Invalid input");
    
    // Read nums
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();
    
    // Check if nums has expected size
    assert_eq!(nums.len(), nums_size, "Input array size doesn't match specified size");
    
    // Calculate and print result
    let res = Solution::find_x_sum(nums, k, x);
    for (i, val) in res.iter().enumerate() {
        print!("{}", val);
        if i < res.len() - 1 {
            print!(" ");
        }
    }
    println!();
}