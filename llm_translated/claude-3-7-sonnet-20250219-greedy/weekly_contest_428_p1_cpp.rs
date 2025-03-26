use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn button_with_longest_time(events: &Vec<Vec<i32>>) -> i32 {
        let mut idx = events[0][0];
        let mut max_diff = events[0][1];
        
        for i in 1..events.len() {
            let p = &events[i - 1];
            let q = &events[i];
            let d = q[1] - p[1];
            
            if d > max_diff || (d == max_diff && q[0] < idx) {
                idx = q[0];
                max_diff = d;
            }
        }
        
        idx
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read number of events
    let n: usize = lines.next().unwrap()?.trim().parse().expect("Failed to parse number of events");
    
    // Initialize events vector
    let mut events: Vec<Vec<i32>> = Vec::with_capacity(n);
    
    // Read events
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse event value"))
            .collect();
        
        events.push(vec![values[0], values[1]]);
    }
    
    // Compute and output result
    let solution = Solution;
    let result = Solution::button_with_longest_time(&events);
    println!("{}", result);
    
    Ok(())
}