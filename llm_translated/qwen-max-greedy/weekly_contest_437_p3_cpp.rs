use std::io::{self, BufRead, Write};

struct Solution;

impl Solution {
    pub fn max_substring_length(s: &str, k: usize) -> bool {
        let n = s.len();
        let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 26];

        // Populate the positions of each character
        for (i, c) in s.chars().enumerate() {
            pos[(c as u8 - b'a') as usize].push(i);
        }

        // Define a type alias for convenience
        type Pii = (usize, usize);

        let mut vec: Vec<Pii> = Vec::new();
        for i in 0..26 {
            if !pos[i].is_empty() {
                let mut l = *pos[i].first().unwrap();
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

        // Sort the vector of pairs
        vec.sort_unstable();

        let mut r = usize::MAX;
        let mut cnt = 0;
        for &(p, l) in &vec {
            if l > r {
                r = p;
                cnt += 1;
            }
        }
        cnt >= k
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read input from stdin
    stdin.lock().read_line(&mut buffer).expect("Failed to read line");
    let input: Vec<&str> = buffer.trim().split_whitespace().collect();
    let s = input[0];
    let k: usize = input[1].parse().expect("Failed to parse K");

    // Create an instance of Solution and call the method
    let result = Solution::max_substring_length(s, k);

    // Write the result to stdout
    writeln!(stdout, "{}", result).expect("Failed to write to stdout");
}