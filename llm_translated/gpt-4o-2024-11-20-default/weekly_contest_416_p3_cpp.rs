use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        // Frequency count of characters in word2
        let mut count = vec![0; 26];
        for c in word2.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }

        let n = word1.len();
        // Pre-computed prefix counts for word1
        let mut pre_count = vec![vec![0; 26]; n + 1];
        for i in 1..=n {
            pre_count[i] = pre_count[i - 1].clone();
            pre_count[i][(word1.as_bytes()[i - 1] - b'a') as usize] += 1;
        }

        // Closure to calculate the smallest valid position
        let get = |l: usize, r: usize| -> usize {
            let mut left = l;
            let mut right = r;
            while left < right {
                let m = (left + right) / 2;
                let mut valid = true;
                for i in 0..26 {
                    if pre_count[m][i] - pre_count[l - 1][i] < count[i] {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    right = m;
                } else {
                    left = m + 1;
                }
            }
            left
        };

        // Compute the result
        let mut res = 0;
        for l in 1..=n {
            let r = get(l, n + 1);
            res += (n - r + 1) as i64;
        }
        res
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parsing input
    let len1: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let word1: String = lines.next().unwrap().unwrap().trim().to_string();
    let len2: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let word2: String = lines.next().unwrap().unwrap().trim().to_string();

    // Ensure the input lengths match the expected string lengths
    assert_eq!(len1, word1.len());
    assert_eq!(len2, word2.len());

    // Solve and output result
    let result = Solution.valid_substring_count(word1, word2);
    println!("{}", result);
}