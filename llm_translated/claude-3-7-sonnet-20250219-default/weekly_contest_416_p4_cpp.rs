use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut diff = vec![0; 26];
        
        // Count character frequencies in word2 (as negative)
        for c in word2.chars() {
            diff[(c as usize) - ('a' as usize)] -= 1;
        }

        let mut res: i64 = 0;
        // Count how many character types have negative balance (need characters from word1)
        let mut cnt = diff.iter().filter(|&&c| c < 0).count();
        
        // Helper closure to update character counts and track deficient characters
        let mut update = |c: usize, add: i32| {
            diff[c] += add;
            if add == 1 && diff[c] == 0 {
                // Character deficit is resolved (from -1 to 0)
                cnt -= 1;
            } else if add == -1 && diff[c] == -1 {
                // New character deficit created (from 0 to -1)
                cnt += 1;
            }
        };

        let word1_chars: Vec<char> = word1.chars().collect();
        let n = word1_chars.len();
        
        let mut r = 0;
        for l in 0..n {
            // Expand right boundary until all character requirements are satisfied
            while r < n && cnt > 0 {
                update((word1_chars[r] as usize) - ('a' as usize), 1);
                r += 1;
            }
            
            // If all character requirements are satisfied
            if cnt == 0 {
                // Add the number of valid substrings starting at l
                res += (n - r + 1) as i64;
            }
            
            // Shrink left boundary
            update((word1_chars[l] as usize) - ('a' as usize), -1);
        }
        
        res
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read length of word1 (not used in solution but part of input format)
    let _len1: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read word1
    let word1 = lines.next().unwrap().unwrap();
    
    // Read length of word2 (not used in solution but part of input format)
    let _len2: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read word2
    let word2 = lines.next().unwrap().unwrap();
    
    let solution = Solution;
    println!("{}", solution::valid_substring_count(word1, word2));
}