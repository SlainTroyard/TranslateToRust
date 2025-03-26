use std::io;

struct Solution;

impl Solution {
    pub fn max_substring_length(&self, s: String, k: i32) -> bool {
        let n = s.len();
        // Initialize positions array for each lowercase character
        let mut pos = vec![vec![]; 26];
        for (i, c) in s.chars().enumerate() {
            let idx = c as usize - 'a' as usize;
            pos[idx].push(i);
        }

        let mut intervals = Vec::new();

        // Process each character's initial interval and expand it
        for i in 0..26 {
            if pos[i].is_empty() {
                continue;
            }
            let mut l = pos[i][0];
            let mut r = *pos[i].last().unwrap();

            // Expand interval until no more changes
            loop {
                let mut expanded = false;
                for j in 0..26 {
                    let pos_j = &pos[j];
                    if pos_j.is_empty() {
                        continue;
                    }
                    // Calculate number of positions in [l, r]
                    let lower = pos_j.partition_point(|&x| x < l);
                    let upper = pos_j.partition_point(|&x| x <= r);
                    let cnt = upper - lower;

                    if cnt > 0 && cnt < pos_j.len() {
                        let first_j = pos_j[0];
                        let last_j = *pos_j.last().unwrap();
                        if first_j < l {
                            l = first_j;
                            expanded = true;
                        }
                        if last_j > r {
                            r = last_j;
                            expanded = true;
                        }
                    }
                }
                if !expanded {
                    break;
                }
            }

            // Add interval if it doesn't cover entire string
            if l > 0 || r < n - 1 {
                intervals.push((r, l));
            }
        }

        // Sort intervals by right endpoint
        intervals.sort_by(|a, b| a.0.cmp(&b.0));

        // Count maximum non-overlapping intervals
        let mut current_end = -1;
        let mut count = 0;
        for &(r, l) in &intervals {
            if l as i64 > current_end {
                count += 1;
                current_end = r as i64;
            }
        }

        count >= k as usize
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        panic!("Invalid input format");
    }
    let s = parts[0].to_string();
    let k: i32 = parts[1].parse().expect("Invalid integer for K");
    let sol = Solution;
    let result = sol.max_substring_length(s, k);
    println!("{}", if result { 1 } else { 0 });
}