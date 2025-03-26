use std::collections::{BTreeSet, HashMap};
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        // Using BTreeSet instead of set since Rust doesn't have a direct equivalent to C++'s set
        let mut l: BTreeSet<(i32, i32)> = BTreeSet::new();
        let mut r: BTreeSet<(i32, i32)> = BTreeSet::new();
        let mut sum_l: i64 = 0;
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        
        // Define add function
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
        
        // Define del function
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
        
        // Define l2r function
        let mut l2r = || {
            if let Some(&p) = l.iter().next() {
                sum_l -= (p.0 as i64) * (p.1 as i64);
                l.remove(&p);
                r.insert(p);
            }
        };
        
        // Define r2l function
        let mut r2l = || {
            if let Some(&p) = r.iter().last() {
                sum_l += (p.0 as i64) * (p.1 as i64);
                r.remove(&p);
                l.insert(p);
            }
        };
        
        let mut ans = vec![0i64; nums.len() - k as usize + 1];
        
        for r_idx in 0..nums.len() {
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
    
    // Read k and x
    let line = lines.next().unwrap()?;
    let mut iter = line.split_whitespace();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let x: i32 = iter.next().unwrap().parse().unwrap();
    
    // Read nums size
    let nums_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read nums
    let line = lines.next().unwrap()?;
    let nums: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Ensure nums has the expected size
    assert_eq!(nums.len(), nums_size);
    
    // Compute result
    let res = Solution::find_x_sum(nums, k, x);
    
    // Print result
    for (i, val) in res.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
    
    Ok(())
}