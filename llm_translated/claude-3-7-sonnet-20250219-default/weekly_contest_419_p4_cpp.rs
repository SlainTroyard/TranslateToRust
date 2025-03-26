use std::collections::{BTreeSet, HashMap};
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        // Using BTreeSet instead of set in C++ for ordered collections
        let mut l: BTreeSet<(i32, i32)> = BTreeSet::new();
        let mut r: BTreeSet<(i32, i32)> = BTreeSet::new();
        let mut sum_l: i64 = 0;
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        
        // Closure to add an element
        let mut add = |val: i32| {
            let count = *cnt.get(&val).unwrap_or(&0);
            let p = (count, val);
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
        
        // Closure to delete an element
        let mut del = |val: i32| {
            let count = *cnt.get(&val).unwrap_or(&0);
            let p = (count, val);
            if p.0 == 0 {
                return;
            }
            if l.contains(&p) {
                sum_l -= (p.0 as i64) * (p.1 as i64);
                l.remove(&p);
            } else {
                r.remove(&p);
            }
        };
        
        // Closure to move an element from left to right
        let mut l2r = || {
            let p = *l.iter().next().unwrap();
            sum_l -= (p.0 as i64) * (p.1 as i64);
            l.remove(&p);
            r.insert(p);
        };
        
        // Closure to move an element from right to left
        let mut r2l = || {
            let p = *r.iter().next_back().unwrap();
            sum_l += (p.0 as i64) * (p.1 as i64);
            r.remove(&p);
            l.insert(p);
        };
        
        let n = nums.len();
        let mut ans = vec![0i64; n - k as usize + 1];
        
        for r_idx in 0..n {
            let in_val = nums[r_idx];
            del(in_val);
            *cnt.entry(in_val).or_insert(0) += 1;
            add(in_val);
            
            let l_idx = r_idx + 1 - k as usize;
            if l_idx < 0 {
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Reading k and x
    let input = lines.next().unwrap()?;
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let k = nums[0];
    let x = nums[1];
    
    // Reading nums size
    let numsSize: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Reading nums array
    let input = lines.next().unwrap()?;
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let solution = Solution;
    let res = Solution::find_x_sum(nums, k, x);
    
    // Output results
    for val in res {
        print!("{} ", val);
    }
    println!();
    
    Ok(())
}