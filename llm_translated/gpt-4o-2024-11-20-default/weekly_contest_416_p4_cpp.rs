use std::io::{self, BufRead};
use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn valid_substring_count(word1: &str, word2: &str) -> i64 {
        let mut diff = vec![0; 26];

        // Decrement frequency of each character in `word2`
        for c in word2.chars() {
            diff[(c as u8 - b'a') as usize] -= 1;
        }

        let mut cnt = diff.iter().filter(|&&val| val < 0).count();
        let mut res = 0;

        // Closure to update the diff array and `cnt`
        let update = |c: usize, add: i32, diff: &mut Vec<i32>, cnt: &mut usize| {
            diff[c] += add;
            match add {
                1 if diff[c] == 0 => *cnt -= 1, // diff[c] changed from -1 to 0
                -1 if diff[c] == -1 => *cnt += 1, // diff[c] changed from 0 to -1
                _ => {}
            }
        };

        let mut l = 0;
        let mut r = 0;

        while l < word1.len() {
            let word1_chars: Vec<char> = word1.chars().collect();
            
            // Increment `r` until the condition `cnt > 0` breaks
            while r < word1.len() && cnt > 0 {
                update((word1_chars[r] as u8 - b'a') as usize, 1, &mut diff, &mut cnt);
                r += 1;
            }

            // If `cnt == 0`, calculate valid substrings using the current range
            if cnt == 0 {
                res += word1.len() as i64 - r as i64 + 1;
            }

            // Move the left pointer and update `diff` and `cnt`
            update((word1_chars[l] as u8 - b'a') as usize, -1, &mut diff, &mut cnt);
            l += 1;
        }

        res
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read input values as per original program
    let len1 = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let word1 = lines.next().unwrap().unwrap();
    assert_eq!(word1.len(), len1, "Length of word1 mismatch");

    let len2 = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let word2 = lines.next().unwrap().unwrap();
    assert_eq!(word2.len(), len2, "Length of word2 mismatch");

    // Initialize Solution and calculate the result
    let solution = Solution;
    println!("{}", solution.valid_substring_count(&word1, &word2));
}