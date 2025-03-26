use std::io::{self, BufRead};
use std::cmp;

struct Solution;

impl Solution {
    fn max_frequency(nums: &Vec<i32>, k: i32) -> i32 {
        let mut f0 = 0;
        let mut f1 = [0; 51]; // Array to store frequencies, assuming constraints from problem
        let mut max_f1 = 0;
        let mut f2 = 0;
        
        for &x in nums {
            // Update f2 - the maximum frequency considering the special value k
            f2 = cmp::max(f2, max_f1) + if x == k { 1 } else { 0 };
            
            // Update f1[x] - the frequency of the current number
            f1[x as usize] = cmp::max(f1[x as usize], f0) + 1;
            
            // Update f0 - count of occurrences of k seen so far
            f0 += if x == k { 1 } else { 0 };
            
            // Update max_f1 - the maximum frequency seen so far
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
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    // Read the nums array from the second line
    let second_line = lines.next().unwrap()?;
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Call the solution function and print the result
    let solution = Solution;
    println!("{}", Solution::max_frequency(&nums, k));
    
    Ok(())
}