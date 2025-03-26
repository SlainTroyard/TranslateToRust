use std::io::{self, BufRead};
use std::cmp;

struct Solution {}

impl Solution {
    pub fn max_frequency(nums: &Vec<i32>, k: i32) -> i32 {
        let mut f0 = 0;
        let mut f1 = [0; 51];
        let mut max_f1 = 0;
        let mut f2 = 0;
        
        for &x in nums {
            f2 = cmp::max(f2, max_f1) + (if x == k { 1 } else { 0 });
            f1[x as usize] = cmp::max(f1[x as usize], f0) + 1;
            f0 += if x == k { 1 } else { 0 };
            max_f1 = cmp::max(max_f1, f1[x as usize]);
        }
        
        cmp::max(max_f1, f2)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and k from the first line
    let first_line = lines.next().unwrap()?;
    let parts: Vec<i32> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = parts[0] as usize;
    let k = parts[1];
    
    // Read n integers from the second line
    let second_line = lines.next().unwrap()?;
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .take(n)  // Ensure we only take n numbers
        .collect();
    
    // Create solution instance and call the method
    let solution = Solution {};
    println!("{}", solution.max_frequency(&nums, k));
    
    Ok(())
}