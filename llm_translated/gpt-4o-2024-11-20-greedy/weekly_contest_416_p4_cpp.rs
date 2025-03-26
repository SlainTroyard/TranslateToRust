use std::io::{self, BufRead};
use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn valid_substring_count(word1: &str, word2: &str) -> i64 {
        let mut diff = vec![0; 26];
        for c in word2.chars() {
            diff[(c as u8 - b'a') as usize] -= 1;
        }

        let mut res: i64 = 0;
        let mut cnt = diff.iter().filter(|&&c| c < 0).count();
        let mut l = 0;
        let mut r = 0;

        // Closure to update the `diff` array and adjust `cnt`
        let mut update = |c: usize, add: i32| {
            diff[c] += add;
            if add == 1 && diff[c] == 0 {
                // diff[c] transitions from -1 to 0
                cnt -= 1;
            } else if add == -1 && diff[c] == -1 {
                // diff[c] transitions from 0 to -1
                cnt += 1;
            }
        };

        let word1_chars: Vec<u8> = word1.bytes().collect();

        while l < word1_chars.len() {
            while r < word1_chars.len() && cnt > 0 {
                update((word1_chars[r] - b'a') as usize, 1);
                r += 1;
            }
            if cnt == 0 {
                res += (word1_chars.len() - r) as i64 + 1;
            }
            update((word1_chars[l] - b'a') as usize, -1);
            l += 1;
        }

        res
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read lengths, though they are not used directly
    let len1: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let word1 = lines.next().unwrap()?.trim().to_string();
    let len2: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let word2 = lines.next().unwrap()?.trim().to_string();

    // Ensure the actual length of the strings matches the given lengths (optional validation)
    assert_eq!(len1, word1.len());
    assert_eq!(len2, word2.len());

    let solution = Solution;
    let result = solution.valid_substring_count(&word1, &word2);

    // Output the result using the same format as the C++ code
    println!("{}", result);
    Ok(())
}