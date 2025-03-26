use std::cmp;
use std::io::{self, Write};

/// A simple struct representing an interval with a right endpoint and a left endpoint.
/// In the C code: typedef struct { int right; int left; } Interval;
#[derive(Debug)]
struct Interval {
    right: usize, 
    left: usize,
}

/// Binary search lower_bound: 
/// Finds the first index where the value at that index is >= val.
fn lower_bound(arr: &[usize], val: usize) -> usize {
    let (mut low, mut high) = (0, arr.len());
    while low < high {
        let mid = (low + high) / 2;
        if arr[mid] < val {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

/// Binary search upper_bound: 
/// Finds the first index where the value at that index is > val.
fn upper_bound(arr: &[usize], val: usize) -> usize {
    let (mut low, mut high) = (0, arr.len());
    while low < high {
        let mid = (low + high) / 2;
        if arr[mid] <= val {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

///
/// Translated logic from the C function bool maxSubstringLength(char* s, int k).
/// This performs the same calculations and returns true or false accordingly.
///
fn max_substring_length(s: &str, k: i32) -> bool {
    let n = s.len();
    // pos[c] will store indices where the character c appears in s (0-based).
    let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 26];

    // Populate pos[c] for each character in s.
    for (i, ch) in s.chars().enumerate() {
        let c_index = (ch as u8 - b'a') as usize;
        if c_index < 26 {
            pos[c_index].push(i);
        }
    }

    // We'll collect up to 26 intervals (one potential interval per character).
    let mut intervals: Vec<Interval> = Vec::with_capacity(26);

    // For each character's positions, try expanding the interval
    // repeatedly until it can't be expanded further.
    for i in 0..26 {
        if !pos[i].is_empty() {
            let mut l = pos[i][0];
            let mut r = pos[i][pos[i].len() - 1];
            let mut flag = true;

            // Continuously try to expand [l, r] by including other characters' ranges
            while flag {
                flag = false;
                for j in 0..26 {
                    if !pos[j].is_empty() {
                        let low_idx = lower_bound(&pos[j], l);
                        let up_idx = upper_bound(&pos[j], r);
                        let cnt = up_idx - low_idx;
                        
                        // If there's at least one occurrence in [l, r] but not all occurrences,
                        // we expand the interval to cover the entire range of character j.
                        if cnt > 0 && cnt < pos[j].len() {
                            l = cmp::min(l, pos[j][0]);
                            r = cmp::max(r, pos[j][pos[j].len() - 1]);
                            flag = true;
                        }
                    }
                }
            }

            // According to the C code: only record intervals if they do not cover the whole string.
            if l > 0 || r < n - 1 {
                intervals.push(Interval { right: r, left: l });
            }
        }
    }

    // Sort intervals by their right endpoint (ascending), just like qsort with compare_intervals.
    intervals.sort_by_key(|interval| interval.right);

    // Greedy approach: pick intervals that don't overlap, counting how many we can take.
    let mut cnt = 0;
    let mut current_end: isize = -1; 
    for interval in &intervals {
        // If the new interval's left is strictly greater than the last chosen interval's right,
        // we can select it.
        if interval.left as isize > current_end {
            current_end = interval.right as isize;
            cnt += 1;
        }
    }

    cnt as i32 >= k
}

fn main() {
    // Read the input in the same format as scanf("%s %d", s, &k).
    // We expect exactly two tokens: a string and an integer.
    let mut buffer = String::new();
    if io::stdin().read_line(&mut buffer).is_err() {
        eprintln!("Error reading input");
        std::process::exit(1);
    }

    let mut parts = buffer.split_whitespace();
    let s = match parts.next() {
        Some(txt) => txt,
        None => {
            eprintln!("Error reading input");
            std::process::exit(1);
        }
    };
    let k_str = match parts.next() {
        Some(txt) => txt,
        None => {
            eprintln!("Error reading input");
            std::process::exit(1);
        }
    };

    let k: i32 = match k_str.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error reading input");
            std::process::exit(1);
        }
    };

    // Compute the result with our translated function.
    let result = max_substring_length(s, k);

    // Print the final result: "true" or "false".
    println!("{}", if result { "true" } else { "false" });
}