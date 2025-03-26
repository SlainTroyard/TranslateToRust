use std::io::{self, BufRead};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the input string and K
    let s = lines.next().unwrap().expect("Failed to read line");
    let k: usize = lines.next().unwrap().expect("Failed to read line").trim().parse().expect("Failed to parse K");

    // Create a Solution instance and call max_substring_length
    let sol = Solution;
    println!("{}", sol.max_substring_length(s, k));
}

struct Solution;

impl Solution {
    fn max_substring_length(s: String, k: usize) -> bool {
        let n = s.len();
        let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 26];

        // Populate the positions of each character
        for (i, c) in s.chars().enumerate() {
            let idx = (c as u8 - b'a') as usize;
            pos[idx].push(i);
        }

        // Process each character's positions to find valid substrings
        let mut vec: Vec<(usize, usize)> = Vec::new();
        for i in 0..26 {
            if !pos[i].is_empty() {
                let mut l = pos[i][0];
                let mut r = *pos[i].last().unwrap();
                let mut flag = true;

                while flag {
                    flag = false;
                    for j in 0..26 {
                        let cnt = pos[j].partition_point(|&x| x <= r) - pos[j].partition_point(|&x| x < l);
                        if cnt > 0 && cnt < pos[j].len() {
                            l = l.min(*pos[j].first().unwrap());
                            r = r.max(*pos[j].last().unwrap());
                            flag = true;
                        }
                    }
                }

                if l > 0 || r < n - 1 {
                    vec.push((r, l));
                }
            }
        }

        // Sort the vector of (r, l) pairs
        vec.sort_unstable();

        // Count the number of valid substrings
        let mut r = -1;
        let mut cnt = 0;
        for (p_r, p_l) in vec {
            if p_l > r {
                r = p_r;
                cnt += 1;
            }
        }

        cnt >= k
    }
}