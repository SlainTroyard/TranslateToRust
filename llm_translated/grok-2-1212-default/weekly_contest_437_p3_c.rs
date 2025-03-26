use std::io::{self, Read, Write};
use std::cmp::{min, max};

// Structure to represent an interval
#[derive(Copy, Clone, PartialEq)]
struct Interval {
    right: i32,  // Right endpoint (used for sorting)
    left: i32,   // Left endpoint
}

// Implement Ord trait for sorting intervals by right endpoint
impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.right.cmp(&other.right)
    }
}

// Implement PartialOrd trait for sorting intervals
impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Implement Eq trait for intervals
impl Eq for Interval {}

// Binary search to find the first element greater than or equal to val
fn lower_bound(arr: &[i32], val: i32) -> usize {
    let (mut low, mut high) = (0, arr.len());
    while low < high {
        let mid = low + (high - low) / 2;
        if arr[mid] < val {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

// Binary search to find the first element greater than val
fn upper_bound(arr: &[i32], val: i32) -> usize {
    let (mut low, mut high) = (0, arr.len());
    while low < high {
        let mid = low + (high - low) / 2;
        if arr[mid] <= val {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

// Main function to solve the problem
fn max_substring_length(s: &str, k: i32) -> bool {
    let n = s.len() as i32;
    
    // Store positions for each character
    let mut pos: [Vec<i32>; 26] = Default::default();
    
    // Store positions of each character
    for (i, c) in s.chars().enumerate() {
        let c = c as usize - 'a' as usize;
        pos[c].push(i as i32);
    }
    
    // Store candidate intervals
    let mut intervals: Vec<Interval> = Vec::new();
    
    // Find intervals that satisfy the condition
    for i in 0..26 {
        if !pos[i].is_empty() {
            let mut l = pos[i][0];
            let mut r = *pos[i].last().unwrap();
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
                            r = max(r, *pos[j].last().unwrap());
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
    intervals.sort_unstable();
    
    // Greedily select intervals
    let mut r = -1;
    let mut cnt = 0;
    for &interval in &intervals {
        if interval.left > r {
            r = interval.right;
            cnt += 1;
        }
    }
    
    cnt >= k
}

fn main() -> io::Result<()> {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let mut lines = input.lines();
    let s = lines.next().unwrap().trim();
    let k: i32 = lines.next().unwrap().trim().parse().unwrap();
    
    // Calculate result
    let result = max_substring_length(s, k);
    
    // Output result
    writeln!(io::stdout(), "{}", if result { "true" } else { "false" })?;
    
    Ok(())
}