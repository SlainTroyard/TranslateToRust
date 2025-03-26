use std::io;

pub struct Solution;

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut diff = [0i32; 26];
        for c in word2.chars() {
            let idx = (c as u8 - b'a') as usize;
            diff[idx] -= 1;
        }

        let mut cnt = diff.iter().filter(|&&x| x < 0).count();

        let word1_chars: Vec<char> = word1.chars().collect();
        let n = word1_chars.len();
        let mut res = 0i64;
        let mut r = 0;

        for l in 0..n {
            while r < n && cnt > 0 {
                let c = word1_chars[r];
                let idx = (c as u8 - b'a') as usize;
                let prev = diff[idx];
                diff[idx] += 1;
                if prev == -1 && diff[idx] == 0 {
                    cnt -= 1;
                }
                r += 1;
            }

            if cnt == 0 {
                res += (n - r) as i64 + 1;
            }

            let c = word1_chars[l];
            let idx = (c as u8 - b'a') as usize;
            let prev = diff[idx];
            diff[idx] -= 1;
            if prev == 0 && diff[idx] == -1 {
                cnt += 1;
            }
        }

        res
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();

    let _len1: usize = iter.next().unwrap().parse().unwrap();
    let word1 = iter.next().unwrap().to_string();
    let _len2: usize = iter.next().unwrap().parse().unwrap();
    let word2 = iter.next().unwrap().to_string();

    let solution = Solution;
    println!("{}", solution.valid_substring_count(word1, word2));
}