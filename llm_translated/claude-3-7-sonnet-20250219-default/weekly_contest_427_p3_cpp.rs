// Problem: Weekly Contest 427 Problem 3
use std::collections::HashMap;
use std::io;

struct Solution;

impl Solution {
    pub fn max_subarray_sum(v: Vec<i32>, k: i32) -> i64 {
        let mut m: HashMap<i32, i64> = HashMap::new();
        let mut ans = i64::MIN;
        let mut sm: i64 = 0;
        
        for i in 0..v.len() {
            sm += v[i] as i64;
            let cur_sz = (i + 1) as i32;
            
            if cur_sz % k == 0 {
                ans = ans.max(sm);
            }
            
            let y = cur_sz % k;
            
            if m.contains_key(&y) {
                ans = ans.max(sm - m[&y]);
                let entry = m.entry(y).or_insert(sm);
                *entry = (*entry).min(sm);
            } else {
                m.insert(y, sm);
            }
        }
        
        ans
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    
    // Input array size
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse().expect("Failed to parse n");
    
    // Input k value
    input.clear();
    io::stdin().read_line(&mut input)?;
    let k: i32 = input.trim().parse().expect("Failed to parse k");
    
    // Input array elements
    input.clear();
    io::stdin().read_line(&mut input)?;
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    
    // Call the max_subarray_sum function and print the result
    let result = Solution::max_subarray_sum(nums, k);
    println!("{}", result);
    
    Ok(())
}