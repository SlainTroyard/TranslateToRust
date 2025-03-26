use std::collections::HashMap;
use std::io::{self, BufRead};

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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and k
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let k: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the array
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Ensure the array has the expected size
    assert_eq!(nums.len(), n);
    
    // Call the max_subarray_sum function and print the result
    let result = Solution::max_subarray_sum(nums, k);
    println!("{}", result);
}