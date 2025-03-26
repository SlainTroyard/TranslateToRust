use std::io;
use std::cmp;
use std::vec::Vec;

#[derive(Debug, Clone)]
struct Interval {
    left: usize,
    right: usize,
}

fn max_substring_length(s: &str, k: usize) -> bool {
    let n = s.len();
    if n == 0 || k == 0 {
        return true;
    }

    // Initialize position arrays for each character
    let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 26];
    for (i, c) in s.chars().enumerate() {
        let index = c as usize - 'a' as usize;
        pos[index].push(i);
    }

    // Collect candidate intervals
    let mut intervals = Vec::new();
    for i in 0..26 {
        if !pos[i].is_empty() {
            let mut l = pos[i][0];
            let mut r = *pos[i].last().unwrap();
            let mut expanded = true;

            while expanded {
                expanded = false;
                for j in 0..26 {
                    if !pos[j].is_empty() {
                        let low_idx = pos[j].binary_search(&l).unwrap_or_else(|x| x);
                        let up_idx = {
                            let mut low = 0;
                            let mut high = pos[j].len();
                            while low < high {
                                let mid = low + (high - low) / 2;
                                if pos[j][mid] <= r {
                                    low = mid + 1;
                                } else {
                                    high = mid;
                                }
                            }
                            low
                        };
                        let cnt = up_idx - low_idx;

                        if cnt > 0 && cnt < pos[j].len() {
                            if pos[j][0] < l {
                                l = pos[j][0];
                                expanded = true;
                            }
                            if pos[j].last().unwrap() > &r {
                                r = *pos[j].last().unwrap();
                                expanded = true;
                            }
                        }
                    }
                }
            }

            if l > 0 || r < n - 1 {
                intervals.push(Interval { left: l, right: r });
            }
        }
    }

    // Sort intervals by right end
    intervals.sort_by(|a, b| a.right.cmp(&b.right));

    // Greedy interval selection
    let mut count = 0;
    let mut last_end = 0;
    for interval in intervals {
        if interval.left >= last_end {
            count += 1;
            last_end = interval.right + 1;
        }
    }

    count >= k
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 2 {
        eprintln!("Error reading input");
        return;
    }

    let s = parts[0];
    let k: usize = parts[1].parse().expect("Invalid k value");

    let result = max_substring_length(s, k);
    println!("{}", if result { "true" } else { "false" });
}