use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut diff = [0i32; 26];
        for c in word2.chars() {
            let idx = (c as u8 - b'a') as usize;
            diff[idx] -= 1;
        }

        let mut cnt = diff.iter().filter(|&&x| x < 0).count() as i32;
        let word1_chars: Vec<usize> = word1
            .chars()
            .map(|c| (c as u8 - b'a') as usize)
            .collect();
        let len = word1_chars.len();
        let mut res = 0i64;

        // Closure to update the diff array and cnt
        let mut update = |c: usize, add: i32| {
            let old = diff[c];
            diff[c] += add;
            let new = diff[c];

            // Update cnt based on the change
            if add == 1 {
                if old == -1 && new == 0 {
                    cnt -= 1;
                }
            } else if add == -1 {
                if old == 0 && new == -1 {
                    cnt += 1;
                }
            }
        };

        let mut l = 0;
        let mut r = 0;
        while l < len {
            // Expand window to the right as much as possible
            while r < len && cnt > 0 {
                let c = word1_chars[r];
                update(c, 1);
                r += 1;
            }

            // If all required characters are covered, add possible substrings
            if cnt == 0 {
                res += (len - r + 1) as i64;
            }

            // Move left pointer and update the diff
            let c = word1_chars[l];
            update(c, -1);
            l += 1;
        }

        res
    }
}

fn main() {
    let stdin = io::stdin();
    let mut tokens = stdin.lock().split_whitespace().map(|s| s.unwrap());

    let _len1: usize = tokens.next().unwrap().parse().unwrap();
    let word1 = tokens.next().unwrap();
    let _len2: usize = tokens.next().unwrap().parse().unwrap();
    let word2 = tokens.next().unwrap();

    let result = Solution::valid_substring_count(word1.to_string(), word2.to_string());
    println!("{}", result);
}