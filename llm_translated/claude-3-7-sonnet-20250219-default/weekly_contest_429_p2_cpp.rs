use std::collections::HashSet;
use std::io;

struct Solution {}

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
    let mut input = String::new();
    
    // Input array size
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse().expect("Expected a number for array size");
    
    // Input difference
    input.clear();
    io::stdin().read_line(&mut input)?;
    let diff: i32 = input.trim().parse().expect("Expected a number for difference");
    
    // Input array elements
    input.clear();
    io::stdin().read_line(&mut input)?;
    let arr_input = input.trim();
    let mut arr: Vec<i32> = arr_input
        .split_whitespace()
        .map(|s| s.parse().expect("Expected a number for array element"))
        .collect();
    
    // Make sure we have exactly n elements
    if arr.len() != n {
        arr.resize(n, 0);
    }
    
    // Compute the result
    let result = Solution::max_distinct_elements(&mut arr, diff);
    
    // Output the result
    println!("{}", result);
    
    Ok(())
}