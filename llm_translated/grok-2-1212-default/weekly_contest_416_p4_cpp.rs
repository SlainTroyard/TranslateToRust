use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut diff = vec![0; 26];
        for c in word2.chars() {
            diff[(c as u8 - b'a') as usize] -= 1;
        }

        let mut res = 0;
        let mut cnt = diff.iter().filter(|&&c| c < 0).count() as i32;
        let mut update = |c: usize, add: i32| {
            diff[c] += add;
            if add == 1 && diff[c] == 0 {
                // Indicates diff[c] changed from -1 to 0
                cnt -= 1;
            } else if add == -1 && diff[c] == -1 {
                // Indicates diff[c] changed from 0 to -1
                cnt += 1;
            }
        };

        let word1_bytes = word1.as_bytes();
        let mut l = 0;
        let mut r = 0;
        while l < word1.len() {
            while r < word1.len() && cnt > 0 {
                update((word1_bytes[r] - b'a') as usize, 1);
                r += 1;
            }
            if cnt == 0 {
                res += (word1.len() - r + 1) as i64;
            }
            update((word1_bytes[l] - b'a') as usize, -1);
            l += 1;
        }
        res
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let len1: usize = lines.next().unwrap()?.parse().unwrap();
    let word1 = lines.next().unwrap()?;
    let len2: usize = lines.next().unwrap()?.parse().unwrap();
    let word2 = lines.next().unwrap()?;

    let s = Solution;
    println!("{}", s.valid_substring_count(word1, word2));

    Ok(())
}