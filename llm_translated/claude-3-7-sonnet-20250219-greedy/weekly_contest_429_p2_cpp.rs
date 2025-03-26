use std::collections::HashSet;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_distinct_elements(arr: &mut Vec<i32>, diff: i32) -> i32 {
        let mut prev = i32::MIN;
        let mut c = HashSet::new();
        
        arr.sort();
        
        for &val in arr.iter() {
            let x = std::cmp::max(prev + 1, val - diff);
            
            if x <= val + diff {
                c.insert(x);
                prev = x;
            }
        }
        
        c.len() as i32
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Input array size and difference
    let n: usize = lines.next().unwrap()?.trim().parse().expect("Failed to parse array size");
    let diff: i32 = lines.next().unwrap()?.trim().parse().expect("Failed to parse difference");
    
    // Input array elements
    let arr_input = lines.next().unwrap()?;
    let mut arr: Vec<i32> = arr_input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse array element"))
        .collect();
    
    // Compute the result
    let result = Solution::max_distinct_elements(&mut arr, diff);
    
    // Output the result
    println!("{}", result);
    
    Ok(())
}