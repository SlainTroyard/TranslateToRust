use std::collections::BTreeSet;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_distinct_elements(arr: &mut Vec<i32>, diff: i32) -> i32 {
        let mut prev = i32::MIN;
        let mut c = BTreeSet::new();
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
    
    // Input array size
    let n: usize = lines.next().unwrap()?.trim().parse()
        .expect("Failed to parse array size");
    
    // Input difference
    let diff: i32 = lines.next().unwrap()?.trim().parse()
        .expect("Failed to parse difference");
    
    // Input array elements
    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        let val: i32 = lines.next().unwrap()?.trim().parse()
            .expect("Failed to parse array element");
        arr.push(val);
    }
    
    // Compute the result
    let result = Solution::max_distinct_elements(&mut arr, diff);
    
    // Output the result
    println!("{}", result);
    
    Ok(())
}