use std::io::{self, Read};

// Returns the smaller of two numbers
fn min(a: i32, b: i32) -> i32 {
    a.min(b)
}

// Returns the larger of two numbers
fn max(a: i32, b: i32) -> i32 {
    a.max(b)
}

// Represents an interval (equivalent to C++ pair<int, int>)
#[derive(Copy, Clone, PartialEq)]
struct Interval {
    right: i32,  // Right endpoint (used for sorting)
    left: i32,   // Left endpoint
}

// Comparison function for sorting intervals by right endpoint
fn compare_intervals(a: &Interval, b: &Interval) -> std::cmp::Ordering {
    a.right.cmp(&b.right)
}

// Binary search for lower_bound (finds the first position >= val)
fn lower_bound(arr: &[i32], val: i32) -> usize {
    let mut low = 0;
    let mut high = arr.len();
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

// Binary search for upper_bound (finds the first position > val)
fn upper_bound(arr: &[i32], val: i32) -> usize {
    let mut low = 0;
    let mut high = arr.len();
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

// Main function implementation
fn max_substring_length(s: &str, k: i32) -> bool {
    let n = s.len() as i32;
    
    // Store positions for each character
    let mut pos: [Vec<i32>; 26] = Default::default();
    
    // Store each character's positions in the corresponding vector
    for (i, c) in s.chars().enumerate() {
        let c = (c as u8 - b'a') as usize;
        pos[c].push(i as i32);
    }
    
    // Store candidate intervals
    let mut intervals = Vec::new();
    
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
    intervals.sort_by(compare_intervals);
    
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
    
    // Call function to calculate result
    let result = max_substring_length(s, k);
    
    // Output result
    println!("{}", if result { "true" } else { "false" });
    
    Ok(())
}