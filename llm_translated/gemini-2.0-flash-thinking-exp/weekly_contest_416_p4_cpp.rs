use std::io;
use std::io::Read;

struct Solution {}

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut diff = vec![0; 26];
        for c in word2.chars() {
            diff[(c as u8 - b'a') as usize] -= 1;
        }

        let mut cnt = diff.iter().filter(|&c| *c < 0).count();
        let mut res: i64 = 0;
        let word1_bytes = word1.as_bytes();
        let n = word1.len();

        let mut update = |c: usize, add: i32| {
            diff[c] += add;
            if add == 1 && diff[c] == 0 {
                cnt -= 1;
            } else if add == -1 && diff[c] == -1 {
                cnt += 1;
            }
        };

        let mut l = 0;
        let mut r = 0;
        while l < n {
            while r < n && cnt > 0 {
                update((word1_bytes[r] - b'a') as usize, 1);
                r += 1;
            }
            if cnt == 0 {
                res += (n - r + 1) as i64;
            }
            update((word1_bytes[l] - b'a') as usize, -1);
            l += 1;
        }

        res
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let len1 = lines.next().unwrap().parse::<usize>().unwrap();
    let word1 = lines.next().unwrap().to_string();
    let len2 = lines.next().unwrap().parse::<usize>().unwrap();
    let word2 = lines.next().unwrap().to_string();

    let s = Solution {};
    println!("{}", s.valid_substring_count(word1, word2));

    Ok(())
}