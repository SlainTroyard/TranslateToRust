use std::cmp::{min, max};
use std::io::{self, Read};

// Represents an interval (equivalent to C++'s pair<int, int>)
#[derive(Clone, Copy, Debug)]
struct Interval {
    right: usize,  // Right endpoint (used for sorting)
    left: usize,   // Left endpoint
}

// Lower bound (find the first element greater than or equal to val)
fn lower_bound(arr: &[usize], val: usize) -> usize {
    arr.partition_point(|&x| x < val)
}

// Upper bound (find the first element greater than val)
fn upper_bound(arr: &[usize], val: usize) -> usize {
    arr.partition_point(|&x| x <= val)
}

// Main function implementation
fn max_substring_length(s: &str, k: usize) -> bool {
    let n = s.len();
    
    // Store positions for each character
    let mut pos: Vec<Vec<usize>> = vec![Vec::with_capacity(n); 26];
    
    // Store each character's positions
    for (i, c) in s.bytes().enumerate() {
        let idx = (c - b'a') as usize;
        pos[idx].push(i);
    }
    
    // Store candidate intervals
    let mut intervals = Vec::new();
    
    // Find intervals that satisfy conditions
    for i in 0..26 {
        if !pos[i].is_empty() {
            let mut l = pos[i][0];
            let mut r = pos[i][pos[i].len() - 1];
            let mut flag = true;
            
            while flag {
                flag = false;
                for j in 0..26 {
                    if !pos[j].is_empty() {
                        let low_idx = lower_bound(&pos[j], l);
                        let up_idx = upper_bound(&pos[j], r);
                        let cnt = up_idx - low_idx;
                        
                        if cnt > 0 && cnt < pos[j].len() {
                            l = min(l, pos[j][0]);
                            r = max(r, pos[j][pos[j].len() - 1]);
                            flag = true;
                        }
                    }
                }
            }
            
            if l > 0 || r < n - 1 {
                intervals.push(Interval { right: r, left: l });
            }
        }
    }
    
    // Sort intervals by right endpoint
    intervals.sort_by_key(|interval| interval.right);
    
    // Greedy selection of intervals
    let mut r: isize = -1;
    let mut cnt = 0;
    for interval in &intervals {
        if interval.left as isize > r {
            r = interval.right as isize;
            cnt += 1;
        }
    }
    
    cnt >= k
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        eprintln!("Error reading input");
        std::process::exit(1);
    }
    
    let s = parts[0];
    let k: usize = parts[1].parse().expect("Failed to parse k");
    
    // Call function to calculate result
    let result = max_substring_length(s, k);
    
    // Output result
    println!("{}", if result { "true" } else { "false" });
}